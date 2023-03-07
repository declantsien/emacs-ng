// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Window;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkMountOperation")]
    pub struct MountOperation(Object<ffi::GtkMountOperation, ffi::GtkMountOperationClass>) @extends gio::MountOperation;

    match fn {
        type_ => || ffi::gtk_mount_operation_get_type(),
    }
}

impl MountOperation {
    pub const NONE: Option<&'static MountOperation> = None;

    #[doc(alias = "gtk_mount_operation_new")]
    pub fn new(parent: Option<&impl IsA<Window>>) -> MountOperation {
        assert_initialized_main_thread!();
        unsafe {
            gio::MountOperation::from_glib_full(ffi::gtk_mount_operation_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MountOperation`] objects.
    ///
    /// This method returns an instance of [`MountOperationBuilder`](crate::builders::MountOperationBuilder) which can be used to create [`MountOperation`] objects.
    pub fn builder() -> MountOperationBuilder {
        MountOperationBuilder::default()
    }
}

impl Default for MountOperation {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MountOperation`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MountOperationBuilder {
    parent: Option<Window>,
    screen: Option<gdk::Screen>,
    anonymous: Option<bool>,
    choice: Option<i32>,
    domain: Option<String>,
    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    is_tcrypt_hidden_volume: Option<bool>,
    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    is_tcrypt_system_volume: Option<bool>,
    password: Option<String>,
    //password-save: /*Unknown type*/,
    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pim: Option<u32>,
    username: Option<String>,
}

impl MountOperationBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`MountOperationBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`MountOperation`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MountOperation {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref screen) = self.screen {
            properties.push(("screen", screen));
        }
        if let Some(ref anonymous) = self.anonymous {
            properties.push(("anonymous", anonymous));
        }
        if let Some(ref choice) = self.choice {
            properties.push(("choice", choice));
        }
        if let Some(ref domain) = self.domain {
            properties.push(("domain", domain));
        }
        #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
        if let Some(ref is_tcrypt_hidden_volume) = self.is_tcrypt_hidden_volume {
            properties.push(("is-tcrypt-hidden-volume", is_tcrypt_hidden_volume));
        }
        #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
        if let Some(ref is_tcrypt_system_volume) = self.is_tcrypt_system_volume {
            properties.push(("is-tcrypt-system-volume", is_tcrypt_system_volume));
        }
        if let Some(ref password) = self.password {
            properties.push(("password", password));
        }
        #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
        if let Some(ref pim) = self.pim {
            properties.push(("pim", pim));
        }
        if let Some(ref username) = self.username {
            properties.push(("username", username));
        }
        glib::Object::new::<MountOperation>(&properties)
    }

    pub fn parent(mut self, parent: &impl IsA<Window>) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn screen(mut self, screen: &gdk::Screen) -> Self {
        self.screen = Some(screen.clone());
        self
    }

    pub fn anonymous(mut self, anonymous: bool) -> Self {
        self.anonymous = Some(anonymous);
        self
    }

    pub fn choice(mut self, choice: i32) -> Self {
        self.choice = Some(choice);
        self
    }

    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pub fn is_tcrypt_hidden_volume(mut self, is_tcrypt_hidden_volume: bool) -> Self {
        self.is_tcrypt_hidden_volume = Some(is_tcrypt_hidden_volume);
        self
    }

    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pub fn is_tcrypt_system_volume(mut self, is_tcrypt_system_volume: bool) -> Self {
        self.is_tcrypt_system_volume = Some(is_tcrypt_system_volume);
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pub fn pim(mut self, pim: u32) -> Self {
        self.pim = Some(pim);
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = Some(username.to_string());
        self
    }
}

pub trait MountOperationExt: 'static {
    #[doc(alias = "gtk_mount_operation_get_parent")]
    #[doc(alias = "get_parent")]
    fn parent(&self) -> Option<Window>;

    #[doc(alias = "gtk_mount_operation_get_screen")]
    #[doc(alias = "get_screen")]
    fn screen(&self) -> Option<gdk::Screen>;

    #[doc(alias = "gtk_mount_operation_is_showing")]
    fn is_showing(&self) -> bool;

    #[doc(alias = "gtk_mount_operation_set_parent")]
    fn set_parent(&self, parent: Option<&impl IsA<Window>>);

    #[doc(alias = "gtk_mount_operation_set_screen")]
    fn set_screen(&self, screen: &gdk::Screen);

    #[doc(alias = "is-showing")]
    fn connect_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "parent")]
    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "screen")]
    fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation>> MountOperationExt for O {
    fn parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_screen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_showing(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_mount_operation_is_showing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_parent(&self, parent: Option<&impl IsA<Window>>) {
        unsafe {
            ffi::gtk_mount_operation_set_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_mount_operation_set_screen(
                self.as_ref().to_glib_none().0,
                screen.to_glib_none().0,
            );
        }
    }

    fn connect_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_showing_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-showing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_showing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screen_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screen\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screen_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MountOperation")
    }
}