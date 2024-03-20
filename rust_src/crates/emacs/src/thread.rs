use crate::bindings::current_thread;
pub use crate::bindings::thread_state as ThreadState;
use crate::buffer::BufferRef;
use crate::lisp::ExternalPtr;

pub type ThreadStateRef = ExternalPtr<ThreadState>;

impl ThreadState {
    pub fn current_buffer_unchecked() -> Option<BufferRef> {
        BufferRef::new(unsafe { (*current_thread).m_current_buffer })
    }

    pub fn current_buffer() -> Option<BufferRef> {
        unsafe { BufferRef::from_ptr((*current_thread).m_current_buffer as *mut libc::c_void) }
    }

    pub fn current_thread() -> Option<ThreadStateRef> {
        ThreadStateRef::new(unsafe { current_thread })
    }
}
// FontRef::new(self.output().font as *mut _)
