use emacs::lisp::ExternalPtr;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::frame::LispFrameExt;
use euclid::default::Size2D;
use gleam::gl;
use log::warn;
use std::collections::HashMap;

use std::ptr;
use surfman::GLApi;
use webrender_surfman::WebrenderSurfman;

use surfman::Connection;
use surfman::SurfaceType;

use webrender::{self, api::units::*, api::*, RenderApi, Renderer, Transaction};

use emacs::frame::LispFrameRef;
use raw_window_handle::RawWindowHandle;

use super::display_info::DisplayInfoRef;
use super::texture::TextureResourceManager;
use super::util::HandyDandyRectBuilder;
use super::{font::FontRef, font_db::FontDB, font_db::FontDescriptor};

#[cfg(wayland_platform)]
use emacs::{bindings::globals, multibyte::LispStringRef};

pub struct CanvasData {

    fonts: HashMap<FontDescriptor, FontKey>,
    font_instances: HashMap<
        (
            FontKey,
            FontSize,
            FontInstanceFlags,
            Option<ColorU>,
            SyntheticItalics,
        ),
        FontInstanceKey,
    >,
    font_render_mode: Option<FontRenderMode>,
    allow_mipmaps: bool,

    pub render_api: RenderApi,
    pub document_id: DocumentId,
    pipeline_id: PipelineId,
    epoch: Epoch,

    display_list_builder: Option<DisplayListBuilder>,
    previous_frame_image: Option<ImageKey>,

    pub background_color: ColorF,
    pub cursor_color: ColorF,
    pub cursor_foreground_color: ColorF,

    // The drop order is important here.

    // Need to dropped before window context
    texture_resources: Rc<RefCell<TextureResourceManager>>,

    // Need to droppend before window context
    renderer: Renderer,

    webrender_surfman: WebrenderSurfman,
    gl: Rc<dyn gl::Gl>,

    frame: LispFrameRef,
    pub raw_window_handle: RawWindowHandle,
}

impl CanvasData {
    pub fn build(raw_window_handle: RawWindowHandle, frame: LispFrameRef) -> Self {
        let dpyinfo = frame.display_info();
        let dpyinfo_ref = dpyinfo.get_inner();
        let connection = dpyinfo_ref.connection.as_ref();
        let size = DeviceIntSize::new(frame.pixel_width, frame.pixel_height);
        let webrender_surfman = Self::new_webrender_surfman(raw_window_handle, connection);
        webrender_surfman
            .resize(Size2D::new(size.width as i32, size.height as i32))
            .unwrap();

        // Get GL bindings
        let gl = match webrender_surfman.connection().gl_api() {
            GLApi::GL => unsafe { gl::GlFns::load_with(|s| webrender_surfman.get_proc_address(s)) },
            GLApi::GLES => unsafe {
                gl::GlesFns::load_with(|s| webrender_surfman.get_proc_address(s))
            },
        };

        let gl = gl::ErrorCheckingGl::wrap(gl);

        // Make sure the gl context is made current.
        webrender_surfman.make_gl_context_current().unwrap();

        let webrender_opts = webrender::WebRenderOptions {
            clear_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
            ..webrender::WebRenderOptions::default()
        };

        let notifier = Box::new(Notifier::new());
        let (mut renderer, sender) =
            webrender::create_webrender_instance(gl.clone(), notifier, webrender_opts, None)
                .unwrap();

        let texture_resources = Rc::new(RefCell::new(TextureResourceManager::new(
            gl.clone(),
            sender.create_api(),
        )));

        let external_image_handler = texture_resources.borrow_mut().new_external_image_handler();

        renderer.set_external_image_handler(external_image_handler);

        let epoch = Epoch(0);
        let pipeline_id = PipelineId(0, 0);
        let mut txn = Transaction::new();
        txn.set_root_pipeline(pipeline_id);

        let mut api = sender.create_api();
        let device_size = DeviceIntSize::new(size.width as i32, size.height as i32);
        let document_id = api.add_document(device_size);
        api.send_transaction(document_id, txn);

        Self {
            fonts: HashMap::new(),
            font_instances: HashMap::new(),
            font_render_mode: None,
            allow_mipmaps: false,
            render_api: api,
            document_id,
            pipeline_id,
            gl,
            epoch,
            display_list_builder: None,
            previous_frame_image: None,
            background_color: ColorF::WHITE,
            cursor_color: ColorF::BLACK,
            cursor_foreground_color: ColorF::WHITE,
            renderer,
            webrender_surfman,
            texture_resources,
            frame,
            raw_window_handle,
        }
    }

    fn copy_framebuffer_to_texture(&self, device_rect: DeviceIntRect) -> ImageKey {
        let mut origin = device_rect.min;

        let device_size = self.device_size();

        if !self.renderer.device.surface_origin_is_top_left() {
            origin.y = device_size.height - origin.y - device_rect.height();
        }

        let fb_rect = FramebufferIntRect::from_origin_and_size(
            FramebufferIntPoint::from_untyped(origin.to_untyped()),
            FramebufferIntSize::from_untyped(device_rect.size().to_untyped()),
        );

        let need_flip = !self.renderer.device.surface_origin_is_top_left();

        let (image_key, texture_id) = self.texture_resources.borrow_mut().new_image(
            self.document_id,
            fb_rect.size(),
            need_flip,
        );

        let gl = &self.gl;
        gl.bind_texture(gl::TEXTURE_2D, texture_id);

        gl.copy_tex_sub_image_2d(
            gl::TEXTURE_2D,
            0,
            0,
            0,
            fb_rect.min.x,
            fb_rect.min.y,
            fb_rect.size().width,
            fb_rect.size().height,
        );

        gl.bind_texture(gl::TEXTURE_2D, 0);

        image_key
    }

    fn layout_size(&self) -> LayoutSize {
        LayoutSize::new(
            self.frame.pixel_width as f32,
            self.frame.pixel_height as f32,
        )
    }

    fn new_builder(&mut self, image: Option<(ImageKey, LayoutRect)>) -> DisplayListBuilder {
        let pipeline_id = self.pipeline_id;

        let layout_size = self.layout_size();
        let mut builder = DisplayListBuilder::new(pipeline_id);
        builder.begin();

        if let Some((image_key, image_rect)) = image {
            let space_and_clip = SpaceAndClipInfo::root_scroll(pipeline_id);

            let bounds = (0, 0).by(layout_size.width as i32, layout_size.height as i32);

            builder.push_image(
                &CommonItemProperties::new(bounds, space_and_clip),
                image_rect,
                ImageRendering::Auto,
                AlphaType::PremultipliedAlpha,
                image_key,
                ColorF::WHITE,
            );
        }

        builder
    }

    pub fn get_frame(&self) -> LispFrameRef {
        self.frame
    }

    pub fn device_size(&self) -> DeviceIntSize {
        DeviceIntSize::new(self.frame.pixel_width, self.frame.pixel_height)
    }

    pub fn display<F>(&mut self, f: F)
    where
        F: Fn(&mut DisplayListBuilder, SpaceAndClipInfo),
    {
        if self.display_list_builder.is_none() {
            let layout_size = self.layout_size();

            let image_and_pos = self
                .previous_frame_image
                .map(|image_key| (image_key, LayoutRect::from_size(layout_size)));

            self.display_list_builder = Some(self.new_builder(image_and_pos));
        }

        let pipeline_id = PipelineId(0, 0);

        if let Some(builder) = &mut self.display_list_builder {
            let space_and_clip = SpaceAndClipInfo::root_scroll(pipeline_id);

            f(builder, space_and_clip);
        }

        self.assert_no_gl_error();
    }

    fn ensure_context_is_current(&mut self) {
        // Make sure the gl context is made current.
        if let Err(err) = self.webrender_surfman.make_gl_context_current() {
            warn!("Failed to make GL context current: {:?}", err);
        }
        self.assert_no_gl_error();
    }

    pub fn flush(&mut self) {
        self.assert_no_gl_error();

        let builder = std::mem::replace(&mut self.display_list_builder, None);

        if let Some(mut builder) = builder {
            let layout_size = self.layout_size();

            let epoch = self.epoch;
            let mut txn = Transaction::new();

            txn.set_display_list(epoch, None, layout_size.to_f32(), builder.end());
            txn.set_root_pipeline(self.pipeline_id);
            txn.generate_frame(0, RenderReasons::NONE);

            self.display_list_builder = None;

            self.render_api.send_transaction(self.document_id, txn);

            self.render_api.flush_scene_builder();

            let device_size = self.device_size();

            // Bind the webrender framebuffer
            self.ensure_context_is_current();

            let framebuffer_object = self
                .webrender_surfman
                .context_surface_info()
                .unwrap_or(None)
                .map(|info| info.framebuffer_object)
                .unwrap_or(0);
            self.gl
                .bind_framebuffer(gleam::gl::FRAMEBUFFER, framebuffer_object);
            self.assert_gl_framebuffer_complete();

            self.renderer.update();

            self.assert_no_gl_error();

            self.renderer.render(device_size, 0).unwrap();
            let _ = self.renderer.flush_pipeline_info();

            self.texture_resources.borrow_mut().clear();

            let image_key = self.copy_framebuffer_to_texture(DeviceIntRect::from_size(device_size));
            self.previous_frame_image = Some(image_key);

            // Perform the page flip. This will likely block for a while.
            if let Err(err) = self.webrender_surfman.present() {
                warn!("Failed to present surface: {:?}", err);
            }
        }
    }

    #[track_caller]
    fn assert_no_gl_error(&self) {
        debug_assert_eq!(self.gl.get_error(), gleam::gl::NO_ERROR);
    }

    #[track_caller]
    fn assert_gl_framebuffer_complete(&self) {
        debug_assert_eq!(
            (
                self.gl.get_error(),
                self.gl.check_frame_buffer_status(gleam::gl::FRAMEBUFFER)
            ),
            (gleam::gl::NO_ERROR, gleam::gl::FRAMEBUFFER_COMPLETE)
        );
    }

    pub fn get_previous_frame(&self) -> Option<ImageKey> {
        self.previous_frame_image
    }

    pub fn clear_display_list_builder(&mut self) {
        let _ = std::mem::replace(&mut self.display_list_builder, None);
    }

    pub fn wr_add_font_instance(
        &mut self,
        font_key: FontKey,
        size: f32,
        flags: FontInstanceFlags,
        render_mode: Option<FontRenderMode>,
        bg_color: Option<ColorU>,
        synthetic_italics: SyntheticItalics,
    ) -> FontInstanceKey {
        let key = self.render_api.generate_font_instance_key();
        let mut txn = Transaction::new();
        let mut options: FontInstanceOptions = Default::default();
        options.flags |= flags;
        if let Some(render_mode) = render_mode {
            options.render_mode = render_mode;
        }
        if let Some(bg_color) = bg_color {
            options.bg_color = bg_color;
        }
        options.synthetic_italics = synthetic_italics;
        txn.add_font_instance(key, font_key, size, Some(options), None, Vec::new());
        self.render_api.send_transaction(self.document_id, txn);
        key
    }

    #[allow(dead_code)]
    pub fn wr_delete_font_instance(&mut self, key: FontInstanceKey) {
        let mut txn = Transaction::new();
        txn.delete_font_instance(key);
        self.render_api.send_transaction(self.document_id, txn);
    }

    pub fn wr_add_font(&mut self, data: FontTemplate) -> FontKey {
        let font_key = self.render_api.generate_font_key();
        let mut txn = Transaction::new();
        match data {
            FontTemplate::Raw(ref bytes, index) => {
                txn.add_raw_font(font_key, bytes.to_vec(), index)
            }
            FontTemplate::Native(native_font) => txn.add_native_font(font_key, native_font),
        }

        self.render_api.send_transaction(self.document_id, txn);

        font_key
    }

    pub fn allow_mipmaps(&mut self, allow_mipmaps: bool) {
        self.allow_mipmaps = allow_mipmaps;
    }

    pub fn set_font_render_mode(&mut self, render_mode: Option<FontRenderMode>) {
        self.font_render_mode = render_mode;
    }

    pub fn get_or_create_font(
        &mut self,
        font_db: &FontDB,
        desc: FontDescriptor,
    ) -> Option<(FontKey, FontTemplate)> {
        let result = font_db.font_from_desc(desc.clone());

        if result.is_none() {
            return None;
        }

        let font = result.unwrap();

        let result = font_db.db.with_face_data(font.id, |font_data, face_index| {
            let font_bytes = Rc::new(font_data.to_vec());
            (font_bytes, face_index)
        });

        if result.is_none() {
            return None;
        }

        let (font_bytes, face_index) = result.unwrap();

        let font_template_raw = FontTemplate::Raw(
            Arc::new(font_bytes.to_vec()),
            face_index.try_into().unwrap(),
        );

        let wr_font_key = self.fonts.get(&desc);

        if let Some(key) = wr_font_key {
            return Some((*key, font_template_raw));
        }

        let wr_font_key = {
            #[cfg(macos_platform)]
            {
                let app_locale = fontdb::Language::English_UnitedStates;
                let family = font.families.iter().find(|family| family.1 == app_locale);

                if let Some((name, _)) = family {
                    let key = self.wr_add_font(FontTemplate::Native(NativeFontHandle {
                        name: name.to_owned(),
                    }));
                    Some(key)
                } else {
                    None
                }
            }

            #[cfg(not(macos_platform))]
            Some(self.wr_add_font(font_template_raw.clone()))
        };

        if let Some(key) = wr_font_key {
            self.fonts.insert(desc, key);
            return Some((key, font_template_raw));
        };

        None
    }

    pub fn get_or_create_font_instance(
        &mut self,
        font_key: FontKey,
        size: f32,
        bg_color: Option<ColorU>,
        flags: FontInstanceFlags,
        synthetic_italics: SyntheticItalics,
    ) -> FontInstanceKey {
        let font_render_mode = self.font_render_mode;
        let hash_map_key = (font_key, size.into(), flags, bg_color, synthetic_italics);
        let font_instance_key = self.font_instances.get(&hash_map_key);
        //TODO update font instances
        match font_instance_key {
            Some(instance_key) => *instance_key,
            None => {
                let instance_key = self.wr_add_font_instance(
                    font_key,
                    size,
                    flags,
                    font_render_mode,
                    bg_color,
                    synthetic_italics,
                );
                self.font_instances.insert(hash_map_key, instance_key);
                instance_key
            }
        }
    }

    pub fn add_image(&mut self, width: i32, height: i32, image_data: Arc<Vec<u8>>) -> ImageKey {
        let image_key = self.render_api.generate_image_key();

        self.update_image(image_key, width, height, image_data);

        image_key
    }

    pub fn update_image(
        &mut self,
        image_key: ImageKey,
        width: i32,
        height: i32,
        image_data: Arc<Vec<u8>>,
    ) {
        let mut txn = Transaction::new();

        txn.add_image(
            image_key,
            ImageDescriptor::new(
                width,
                height,
                ImageFormat::RGBA8,
                ImageDescriptorFlags::empty(),
            ),
            ImageData::Raw(image_data),
            None,
        );

        self.render_api.send_transaction(self.document_id, txn);
    }

    pub fn delete_image(&mut self, image_key: ImageKey) {
        let mut txn = Transaction::new();

        txn.delete_image(image_key);

        self.render_api.send_transaction(self.document_id, txn);
    }

    pub fn resize(&mut self, size: &DeviceIntSize) {
        log::trace!("resize {size:?}");
        let device_size = DeviceIntSize::new(size.width as i32, size.height as i32);
        self.frame.pixel_width = size.width;
        self.frame.pixel_height = size.height;

        let device_rect =
            DeviceIntRect::from_origin_and_size(DeviceIntPoint::new(0, 0), device_size);

        let mut txn = Transaction::new();
        txn.set_document_view(device_rect);
        self.render_api.send_transaction(self.document_id, txn);

        self.webrender_surfman
            .resize(Size2D::new(size.width as i32, size.height as i32))
            .unwrap();
    }

    pub fn new_webrender_surfman(
        handle: raw_window_handle::RawWindowHandle,
        connection: Option<&Connection>,
    ) -> WebrenderSurfman {
        let connection = connection.expect("device not open");
        let adapter = connection
            .create_adapter()
            .expect("Failed to create adapter");
        let native_widget = connection
            .create_native_widget_from_rwh(handle)
            .expect("Failed to create native widget");
        let surface_type = SurfaceType::Widget { native_widget };
        let webrender_surfman = WebrenderSurfman::create(&connection, &adapter, surface_type)
            .expect("Failed to create WR surfman");

        webrender_surfman
    }
}

pub type CanvasDataRef = ExternalPtr<CanvasData>;

#[cfg(window_system = "winit")]
pub type output = emacs::bindings::winit_output;
#[cfg(window_system = "pgtk")]
pub type output = emacs::bindings::pgtk_output;

#[derive(Default)]
#[repr(transparent)]
pub struct Output(output);
pub type OutputRef = ExternalPtr<Output>;

impl Output {
    pub fn new() -> Self {
        let ret = Output::default();
        ret
    }

    pub fn empty_inner(&mut self) {
        self.0.inner = ptr::null_mut();
    }

    pub fn set_inner(&mut self, data: Box<CanvasData>) {
        self.0.inner = Box::into_raw(data) as *mut libc::c_void;
    }

    // pub fn init_inner(&mut self) {
    //     // let inner = Box::new(DisplayInfoInner::default());
    //     // self.0.inner = Box::into_raw(inner) as *mut libc::c_void;
    // }

    pub fn set_display_info(&mut self, mut dpyinfo: DisplayInfoRef) {
        self.0.display_info = dpyinfo.get_raw().as_mut();
    }

    pub fn set_font(&mut self, mut font: FontRef) {
        self.0.font = font.as_mut();
    }

    pub fn set_fontset(&mut self, fontset: i32) {
        self.0.fontset = fontset;
    }

    pub fn display_info(&self) -> DisplayInfoRef {
        DisplayInfoRef::new(self.0.display_info as *mut _)
    }

    pub fn font(&self) -> FontRef {
        FontRef::new(self.0.font as *mut _)
    }

    pub fn fontset(&self) -> i32 {
	self.0.fontset
    }

    pub fn canvas_data(&self) -> CanvasDataRef {
        CanvasDataRef::new(self.0.inner as *mut CanvasData)
    }

    pub fn get_raw(&mut self) -> ExternalPtr<output> {
        (&mut self.0 as *mut output).into()
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        if self.0.inner != ptr::null_mut() {
            unsafe {
                let _ = Box::from_raw(self.0.inner as *mut CanvasData);
            }
        }
    }
}

struct Notifier {
    // events_proxy: winit::event_loop::EventLoopProxy<i32>,
}

impl Notifier {
    // fn new(events_proxy: winit::event_loop::EventLoopProxy<i32>) -> Notifier {
    //     Notifier { events_proxy }
    // }
    fn new() -> Notifier {
        Notifier {}
    }
}

impl RenderNotifier for Notifier {
    fn clone(&self) -> Box<dyn RenderNotifier> {
        Box::new(Notifier {
            // events_proxy: self.events_proxy.clone(),
        })
    }

    fn wake_up(&self, _composite_needed: bool) {}

    fn new_frame_ready(&self, _: DocumentId, _scrolled: bool, composite_needed: bool) {
        self.wake_up(composite_needed);
    }
}
