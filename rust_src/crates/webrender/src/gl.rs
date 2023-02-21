use log::warn;
#[cfg(feature="glutin")]
use glutin::{
    surface::{Surface, WindowSurface, SurfaceAttributesBuilder},
    context::{
        ContextApi, ContextAttributesBuilder, Version,PossiblyCurrentContext,
        PossiblyCurrentContextGlSurfaceAccessor,
        NotCurrentGlContextSurfaceAccessor,
    },
    display::{Display, DisplayApiPreference, GlDisplay, GetGlDisplay},
    config::{Api, ConfigTemplateBuilder, GlConfig},
    prelude::GlSurface
};

#[cfg(feature="glutin")]
use std::{
    num::NonZeroU32,
    ffi::CString,
};

#[cfg(feature="surfman")]
use surfman::{
    GLApi,
    Connection,
    SurfaceType,
};

#[cfg(feature="surfman")]
use webrender_surfman::WebrenderSurfman;

use euclid::Size2D;

use raw_window_handle::{
    RawWindowHandle,
    RawDisplayHandle,
};
use webrender::api::units::DevicePixel;

use std::rc::Rc;

use gleam::gl::{
    Gl, GlFns, GlesFns, ErrorCheckingGl
};

#[cfg(feature="glutin")]
pub struct GlContext {
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    gl: Rc<dyn Gl>
}

#[cfg(feature="glutin")]
impl GlContext {
    pub fn new(size: Size2D<i32, DevicePixel>, display_handle: RawDisplayHandle, window_handle: RawWindowHandle) -> Self {
        let width = NonZeroU32::new(size.width as u32).unwrap();
        let height = NonZeroU32::new(size.height as u32).unwrap();


        // TODO proper preference logic here.
        let preference = DisplayApiPreference::Egl;
        let gl_display = unsafe {Display::new(display_handle, preference) }.unwrap();
        let template = ConfigTemplateBuilder::new().build(); // TODO do we need to do anything to this?
        let gl_config = unsafe {
            let configs = gl_display.find_configs(template).unwrap();
            // get best config
            configs
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        };

        let context_attributes = ContextAttributesBuilder::new().build(Some(window_handle));
        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(window_handle));

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(Some(window_handle));

        let mut context = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap_or_else(|_| {
                gl_display.create_context(&gl_config, &fallback_context_attributes).unwrap_or_else(
                    |_| {
                        gl_display
                            .create_context(&gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    },
                )
            })
        });

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle, width, height
        );
        let surface = unsafe { gl_display.create_window_surface(&gl_config, &attrs).unwrap()};
        let context = context.take().unwrap().make_current(&surface).unwrap();
        // TODO haven't we done this above?
        surface.resize(&context, width, height);

        let gl = {
            let flags = gl_config.api();
            if flags.contains(Api::OPENGL) {
                unsafe {GlFns::load_with(|symbol| gl_config.display().get_proc_address(&CString::new(symbol).unwrap()) as *const _)}
            } else if flags.intersects(Api::GLES1 | Api::GLES2 | Api::GLES3 ) {
                unsafe {GlesFns::load_with(|symbol| gl_config.display().get_proc_address(&CString::new(symbol).unwrap()) as *const _)}
            } else {
                unimplemented!();
            }
        };

        GlContext {
            surface,
            context,
            gl,
        }
    }

    pub fn bind_framebuffer(&self) {}

    pub fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).ok();
    }

    pub fn resize(&self, size: Size2D<i32, DevicePixel>) {
        self.surface
            .resize(
                &self.context,
                NonZeroU32::new(size.width as u32).unwrap(),
                NonZeroU32::new(size.height as u32).unwrap());
    }

    pub fn ensure_context_is_current(&mut self) {
        // Make sure the gl context is made current.
        if let Err(err) = self.context.make_current(&self.surface) {
            warn!("Failed to make GL context current: {:?}", err);
        }
        self.assert_no_gl_error();
    }

    #[track_caller]
    pub fn assert_no_gl_error(&self) {
        debug_assert_eq!(self.gl.get_error(), gleam::gl::NO_ERROR);
    }

    #[track_caller]
    pub fn assert_gl_framebuffer_complete(&self) {
        debug_assert_eq!(
            (
                self.gl.get_error(),
                self.gl.check_frame_buffer_status(gleam::gl::FRAMEBUFFER)
            ),
            (gleam::gl::NO_ERROR, gleam::gl::FRAMEBUFFER_COMPLETE)
        );
    }

    pub fn get_gl(&self) -> Rc<dyn Gl> {
        ErrorCheckingGl::wrap(self.gl.clone())
    }
}
#[cfg(feature="surfman")]
pub struct GlContext {
    webrender_surfman: WebrenderSurfman,
    gl: Rc<dyn Gl>
}

#[cfg(feature="surfman")]
impl GlContext {
    pub fn new(size: Size2D<i32, DevicePixel>, display_handle: RawDisplayHandle, window_handle: RawWindowHandle) -> Self {
        let connection = match Connection::from_raw_display_handle(display_handle) {
            Ok(connection) => connection,
            Err(error) => panic!("Device not open {:?}", error),
        };

        let adapter = connection
            .create_adapter()
            .expect("Failed to create adapter");

        let native_widget = connection
            .create_native_widget_from_rwh(window_handle)
            .expect("Failed to create native widget");

        let surface_type = SurfaceType::Widget { native_widget };

        let webrender_surfman = WebrenderSurfman::create(&connection, &adapter, surface_type)
            .expect("Failed to create WR surfman");

        webrender_surfman
            .resize(Size2D::new(size.width as i32, size.height as i32))
            .unwrap();

        // Get GL bindings
        let gl = match webrender_surfman.connection().gl_api() {
            GLApi::GL => unsafe { GlFns::load_with(|s| webrender_surfman.get_proc_address(s)) },
            GLApi::GLES => unsafe {
                GlesFns::load_with(|s| webrender_surfman.get_proc_address(s))
            },
        };
        webrender_surfman.make_gl_context_current().unwrap();

        GlContext {
            webrender_surfman,
            gl,
        }
    }

    pub fn bind_framebuffer(&mut self) {
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
    }

    pub fn swap_buffers(&self) {
	// Perform the page flip. This will likely block for a while.
        if let Err(err) = self.webrender_surfman.present() {
            warn!("Failed to present surface: {:?}", err);
        }
    }

    #[track_caller]
    pub fn assert_gl_framebuffer_complete(&self) {
        debug_assert_eq!(
            (
                self.gl.get_error(),
                self.gl.check_frame_buffer_status(gleam::gl::FRAMEBUFFER)
            ),
            (gleam::gl::NO_ERROR, gleam::gl::FRAMEBUFFER_COMPLETE)
        );
    }

    pub fn resize(&self, size: Size2D<i32, DevicePixel>) {
        self.webrender_surfman
            .resize(Size2D::new(size.width as i32, size.height as i32))
            .unwrap();
    }

    pub fn ensure_context_is_current(&mut self) {
        self.webrender_surfman.make_gl_context_current();
    }

    #[track_caller]
    pub fn assert_no_gl_error(&self) {
        debug_assert_eq!(self.gl.get_error(), gleam::gl::NO_ERROR);
    }
    // TODO move duplicate stuff to trait
    pub fn get_gl(&self) -> Rc<dyn Gl> {
        ErrorCheckingGl::wrap(self.gl.clone())
    }
}
