pub fn initialized() -> bool {
    unsafe { crate::bindings::initialized }
}

pub fn set_initialized(initialized: bool) {
    unsafe { crate::bindings::initialized = initialized };
}
