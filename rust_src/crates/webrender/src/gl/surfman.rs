use crate::frame::LispFrameExt;
use emacs::frame::LispFrameRef;

use surfman::{Connection, GLApi, SurfaceType};

use euclid::Size2D;

use webrender::api::units::DevicePixel;

use std::rc::Rc;

use gleam::gl::{ErrorCheckingGl, Gl, GlFns, GlesFns};

use webrender_surfman::WebrenderSurfman;

pub struct GlContext {
    webrender_surfman: WebrenderSurfman,
    gl: Rc<dyn Gl>,
}

impl GlContext {
    pub fn new(frame: LispFrameRef) -> Self {
        let display_handle = frame
            .display_handle()
            .expect("Failed to raw display handle from frame");
        let window_handle = frame
            .window_handle()
            .expect("Failed to get raw window handle from frame");
        let size = frame.size();

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
            GLApi::GLES => unsafe { GlesFns::load_with(|s| webrender_surfman.get_proc_address(s)) },
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
            log::error!("Failed to present surface: {:?}", err);
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
        if let Err(err) = self.webrender_surfman.make_gl_context_current() {
            log::error!("Failed to make gl context current: {:?}", err);
        }
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
