use crate::frame::LispFrameExt;
use emacs::frame::LispFrameRef;
use euclid::Size2D;
use gleam::gl::ErrorCheckingGl;
use gleam::gl::GlesFns;
use std::rc::Rc;
use webrender::api::units::DevicePixel;
use winit::platform::unix::WindowExtUnix;
// use gdk::GLContext;
use gleam::gl::Gl;
use gleam::gl::GlFns;
use gtk::builders::GLAreaBuilder;
use gtk::prelude::*;
use gtk::GLArea;

use std::marker::PhantomData;

pub struct GlContext {
    ctx: *mut gdk_sys::GdkGLContext,
    // // area: GLArea,
    area: *mut gtk_sys::GtkGLArea,
    gl: Rc<dyn Gl>,
}

impl GlContext {
    pub fn new(frame: LispFrameRef) -> Self {
        let mut output = frame.output();
        let area = output.as_raw().canvas;
        // let gwin = unsafe { gtk_sys::gtk_widget_get_window(widget) };
        // let mut error = ptr::null_mut();
        // let ctx = unsafe { gdk_sys::gdk_window_create_gl_context(gwin, &mut error) };

        // let frame_inner = frame.output().get_inner();
        // let window = frame_inner.window.as_ref().expect("No window");

        // let gtkwin = window.gtk_window();

        // // TODO config of pf_reqs and gl_attr
        // let area = GLAreaBuilder::new().has_alpha(true).build();
        // let vbox = gtkwin.children().pop().unwrap().downcast::<gtk::Box>().unwrap();
        // vbox.pack_start(&area, true, true, 0);
        // area.grab_focus();
        // gtkwin.show_all();
        // let ctx = area.window().unwrap().create_gl_context().expect("failed to create gdk context");
        // ctx.realize();
        // ctx.make_current();
        // let version = ctx.version();
        // log::trace!("gl version {:?}", version);

        let mut ctx = std::ptr::null_mut();
        unsafe {
            // gtk_sys::gtk_gl_area_set_use_es(area, 0);
            gtk_sys::gtk_gl_area_make_current(area);
            let window = gtk_sys::gtk_widget_get_window(area as *mut _ as *mut gtk_sys::GtkWidget);
            let mut err = std::ptr::null_mut();
            ctx = gdk_sys::gdk_window_create_gl_context(window, err);
            // gdk_gl_context_set_required_version(ctx,
            //                                 params->major_ver,
            //                                 params->minor_ver);
            gdk_sys::gdk_gl_context_realize(ctx, err);
        }

        // epoxy::load_with(|s| {
        //     unsafe {
        // 	match DynamicLibrary::open(None).unwrap().symbol(s) {
        // 	    Ok(v) => v,
        // 	    Err(_) => ptr::null(),
        // 	}
        //     }
        // });
        gl_loader::init_gl();

        // area.make_current();
        // let gl = unsafe { GlFns::load_with(|s| gl_loader::get_proc_address(s) as *const _) };
        let gl = unsafe { GlesFns::load_with(|s| gl_loader::get_proc_address(s) as *const _) };
        // TODO detect es

        // area.connect_render(move |_, _| {
        //     // gl.draw_frame([0.0; 4]);
        //     gtk::Inhibit(false)
        // });

        GlContext { ctx, area, gl }
    }

    pub fn bind_framebuffer(&self) {}

    #[inline]
    pub fn swap_buffers(&self) {
        // GTK swaps the buffers after each "render" signal itself
        // self.area.queue_render();
        unsafe { gtk_sys::gtk_gl_area_queue_render(self.area) }
    }

    // #[inline]
    // pub fn get_api(&self) -> Api {
    //     // TODO detect es
    //     Api::OpenGl
    // }

    #[inline]
    pub fn is_current(&self) -> bool {
        // self.area.context() == gdk::GLContext::current()
        true
    }

    // #[inline]
    // pub unsafe fn make_not_current(&self) -> Result<(), gdk::ContextError> {
    //     gdk::GLContext::clear_current();
    //     Ok(())
    // }

    #[inline]
    pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
        gl_loader::get_proc_address(addr) as *const _
    }

    pub fn resize(&self, size: Size2D<i32, DevicePixel>) {
        // Ignored because widget will be resized automatically
    }

    pub fn ensure_context_is_current(&mut self) {
        // self.ctx.make_current();
        // self.area.make_current();
        unsafe { gtk_sys::gtk_gl_area_make_current(self.area) };
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
