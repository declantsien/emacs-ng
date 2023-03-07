// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventController;
use crate::Gesture;
use crate::GestureSingle;
use crate::PropagationPhase;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkGestureSwipe")]
    pub struct GestureSwipe(Object<ffi::GtkGestureSwipe, ffi::GtkGestureSwipeClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        type_ => || ffi::gtk_gesture_swipe_get_type(),
    }
}

impl GestureSwipe {
    #[doc(alias = "gtk_gesture_swipe_new")]
    pub fn new(widget: &impl IsA<Widget>) -> GestureSwipe {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_swipe_new(widget.as_ref().to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GestureSwipe`] objects.
    ///
    /// This method returns an instance of [`GestureSwipeBuilder`](crate::builders::GestureSwipeBuilder) which can be used to create [`GestureSwipe`] objects.
    pub fn builder() -> GestureSwipeBuilder {
        GestureSwipeBuilder::default()
    }

    #[doc(alias = "gtk_gesture_swipe_get_velocity")]
    #[doc(alias = "get_velocity")]
    pub fn velocity(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut velocity_x = mem::MaybeUninit::uninit();
            let mut velocity_y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_gesture_swipe_get_velocity(
                self.to_glib_none().0,
                velocity_x.as_mut_ptr(),
                velocity_y.as_mut_ptr(),
            ));
            if ret {
                Some((velocity_x.assume_init(), velocity_y.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "swipe")]
    pub fn connect_swipe<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn swipe_trampoline<F: Fn(&GestureSwipe, f64, f64) + 'static>(
            this: *mut ffi::GtkGestureSwipe,
            velocity_x: libc::c_double,
            velocity_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), velocity_x, velocity_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    swipe_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for GestureSwipe {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GestureSwipe`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GestureSwipeBuilder {
    button: Option<u32>,
    exclusive: Option<bool>,
    touch_only: Option<bool>,
    n_points: Option<u32>,
    window: Option<gdk::Window>,
    propagation_phase: Option<PropagationPhase>,
    widget: Option<Widget>,
}

impl GestureSwipeBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GestureSwipeBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GestureSwipe`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GestureSwipe {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref button) = self.button {
            properties.push(("button", button));
        }
        if let Some(ref exclusive) = self.exclusive {
            properties.push(("exclusive", exclusive));
        }
        if let Some(ref touch_only) = self.touch_only {
            properties.push(("touch-only", touch_only));
        }
        if let Some(ref n_points) = self.n_points {
            properties.push(("n-points", n_points));
        }
        if let Some(ref window) = self.window {
            properties.push(("window", window));
        }
        if let Some(ref propagation_phase) = self.propagation_phase {
            properties.push(("propagation-phase", propagation_phase));
        }
        if let Some(ref widget) = self.widget {
            properties.push(("widget", widget));
        }
        glib::Object::new::<GestureSwipe>(&properties)
    }

    pub fn button(mut self, button: u32) -> Self {
        self.button = Some(button);
        self
    }

    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = Some(exclusive);
        self
    }

    pub fn touch_only(mut self, touch_only: bool) -> Self {
        self.touch_only = Some(touch_only);
        self
    }

    pub fn n_points(mut self, n_points: u32) -> Self {
        self.n_points = Some(n_points);
        self
    }

    pub fn window(mut self, window: &gdk::Window) -> Self {
        self.window = Some(window.clone());
        self
    }

    pub fn propagation_phase(mut self, propagation_phase: PropagationPhase) -> Self {
        self.propagation_phase = Some(propagation_phase);
        self
    }

    pub fn widget(mut self, widget: &impl IsA<Widget>) -> Self {
        self.widget = Some(widget.clone().upcast());
        self
    }
}

impl fmt::Display for GestureSwipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GestureSwipe")
    }
}