use errno::{set_errno, Errno};
use nix::sys::signal::{self, Signal};
use std::{
    cell::RefCell,
    ptr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tao::window::WindowId;

#[cfg(macos_platform)]
use copypasta::osx_clipboard::OSXClipboardContext;
#[cfg(windows_platform)]
use copypasta::windows_clipboard::WindowsClipboardContext;
use copypasta::ClipboardProvider;
#[cfg(free_unix)]
use copypasta::{
    wayland_clipboard::create_clipboards_from_external,
    x11_clipboard::{Clipboard, X11ClipboardContext},
};
#[cfg(feature = "tao")]
use tao::platform::unix::EventLoopWindowTargetExtUnix;

use libc::{c_void, fd_set, pselect, sigset_t, timespec};
use once_cell::sync::Lazy;
#[cfg(feature = "tao")]
use tao::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    monitor::MonitorHandle,
    platform::run_return::EventLoopExtRunReturn,
};
#[cfg(all(free_unix, feature = "winit"))]
use winit::platform::wayland::EventLoopWindowTargetExtWayland;
#[cfg(all(x11_platform, feature = "winit"))]
use winit::platform::x11::EventLoopWindowTargetExtX11;
#[cfg(feature = "winit")]
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    event_loop::EventLoopBuilder,
    monitor::MonitorHandle,
    platform::run_return::EventLoopExtRunReturn,
};

use emacs::bindings::{inhibit_window_system, make_timespec, thread_select};

pub type GUIEvent = Event<'static, i32>;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Platform {
    X11,
    Wayland(*mut c_void),
    MacOS,
    Windows,
}

unsafe impl Send for Platform {}

pub struct WrEventLoop {
    clipboard: tao::clipboard::Clipboard,
    el: EventLoop<i32>,
}

unsafe impl Send for WrEventLoop {}
unsafe impl Sync for WrEventLoop {}

impl WrEventLoop {
    pub fn el(&self) -> &EventLoop<i32> {
        &self.el
    }

    pub fn get_available_monitors(&self) -> impl Iterator<Item = MonitorHandle> {
        self.el.available_monitors()
    }

    pub fn get_primary_monitor(&self) -> MonitorHandle {
        self.el
            .primary_monitor()
            .unwrap_or_else(|| -> MonitorHandle { self.get_available_monitors().next().unwrap() })
    }

    pub fn get_clipboard(&mut self) -> &mut tao::clipboard::Clipboard {
        &mut self.clipboard
    }
}

pub static EVENT_LOOP: Lazy<Arc<Mutex<WrEventLoop>>> = Lazy::new(|| {
    #[cfg(feature = "winit")]
    let el = EventLoopBuilder::<i32>::with_user_event().build();
    #[cfg(feature = "tao")]
    let el = EventLoop::<i32>::with_user_event();
    let clipboard = tao::clipboard::Clipboard::new();

    Arc::new(Mutex::new(WrEventLoop { clipboard, el }))
});

pub static EVENT_BUFFER: Lazy<Mutex<Vec<GUIEvent>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn flush_events() -> Vec<GUIEvent> {
    let event_buffer = EVENT_BUFFER.try_lock();

    if event_buffer.is_err() {
        return Vec::new();
    }

    let mut buffer = event_buffer.ok().unwrap();
    let events = buffer.clone();
    buffer.clear();
    events
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FdSet(pub *mut fd_set);

unsafe impl Send for FdSet {}
unsafe impl Sync for FdSet {}

impl FdSet {
    fn clear(&self) {
        if self.0 != ptr::null_mut() {
            unsafe { libc::FD_ZERO(self.0) };
        }
    }
}

impl Drop for FdSet {
    fn drop(&mut self) {
        self.clear()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Timespec(pub *mut timespec);

unsafe impl Send for Timespec {}
unsafe impl Sync for Timespec {}

#[no_mangle]
pub extern "C" fn winit_select(
    nfds: i32,
    readfds: *mut fd_set,
    writefds: *mut fd_set,
    _exceptfds: *mut fd_set,
    timeout: *mut timespec,
    _sigmask: *mut sigset_t,
) -> i32 {
    let lock_result = EVENT_LOOP.try_lock();

    if lock_result.is_err() || unsafe { inhibit_window_system } {
        if lock_result.is_err() {
            log::debug!("Failed to grab a lock {:?}", lock_result.err());
        }

        return unsafe {
            thread_select(
                Some(pselect),
                nfds,
                readfds,
                writefds,
                _exceptfds,
                timeout,
                _sigmask,
            )
        };
    }

    let mut event_loop = lock_result.unwrap();

    let deadline = Instant::now()
        + unsafe { Duration::new((*timeout).tv_sec as u64, (*timeout).tv_nsec as u32) };

    let nfds_result = RefCell::new(0);

    // We mush run winit in main thread, because the macOS platfrom limitation.
    event_loop.el.run_return(|e, _target, control_flow| {
        *control_flow = ControlFlow::WaitUntil(deadline);

        if let Event::WindowEvent { event, .. } = &e {
            // Print only Window events to reduce noise
            log::trace!("{:?}", event);
        }

        match e {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(_)
                | WindowEvent::KeyboardInput { .. }
                | WindowEvent::ReceivedImeText(_)
                | WindowEvent::ModifiersChanged(_)
                | WindowEvent::MouseInput { .. }
                | WindowEvent::CursorMoved { .. }
                | WindowEvent::ThemeChanged(_)
                | WindowEvent::Focused(_)
                | WindowEvent::MouseWheel { .. }
                | WindowEvent::CloseRequested => {
                    if let Ok(mut event_buffer) = EVENT_BUFFER.try_lock() {
                        event_buffer.push(e.to_static().unwrap());
                        // notify emacs's code that a keyboard event arrived.
                        match signal::raise(Signal::SIGIO) {
                            Ok(_) => {}
                            Err(err) => log::error!("sigio err: {err:?}"),
                        };
                        let _is_x11 = false;

                        #[cfg(x11_platform)]
                        let _is_x11 = _target.is_x11();

                        if _is_x11 {
                            nfds_result.replace(1);
                        } else {
                            /* Pretend that `select' is interrupted by a signal.  */
                            set_errno(Errno(libc::EINTR));
                            debug_assert_eq!(nix::errno::errno(), libc::EINTR);
                            nfds_result.replace(-1);
                        }
                    } else {
                        log::debug!("Failed to grab a lock for EVENT_BUFFER");
                    }

                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::UserEvent(nfds) => {
                nfds_result.replace(nfds);
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawEventsCleared => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        };
    });
    let ret = nfds_result.into_inner();
    if ret == 0 {
        let timespec = unsafe { make_timespec(0, 0) };
        // Add some delay here avoding high cpu usage on macOS
        // #[cfg(macos_platform)]
        spin_sleep::sleep(Duration::from_millis(16));
        let nfds =
            unsafe { libc::pselect(nfds, readfds, writefds, _exceptfds, &timespec, _sigmask) };
        // log::trace!("pselect: {nfds:?}");
        return nfds;
    }

    log::trace!("winit event run_return: {ret:?}");

    ret
}

// Polling C-g when emacs is blocked
pub fn poll_a_event(timeout: Duration) -> Option<GUIEvent> {
    log::trace!("poll a event {:?}", timeout);
    let result = EVENT_LOOP.try_lock();
    if result.is_err() {
        log::trace!("failed to grab a EVENT_LOOP lock");
        return None;
    }
    let mut event_loop = result.unwrap();
    let deadline = Instant::now() + timeout;
    let result = RefCell::new(None);
    event_loop.el.run_return(|e, _target, control_flow| {
        *control_flow = ControlFlow::WaitUntil(deadline);

        if let Event::WindowEvent { event, .. } = &e {
            log::trace!("{:?}", event);
        }

        match e {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(_)
                | WindowEvent::KeyboardInput { .. }
                | WindowEvent::ReceivedImeText(_)
                | WindowEvent::ModifiersChanged(_)
                | WindowEvent::MouseInput { .. }
                | WindowEvent::CursorMoved { .. }
                | WindowEvent::Focused(_)
                | WindowEvent::MouseWheel { .. }
                | WindowEvent::CloseRequested => {
                    result.replace(Some(e.to_static().unwrap()));
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::RedrawRequested(e) => {
                log::debug!("WindowEvent:: RedrawRequested");
            }
            Event::RedrawEventsCleared => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        };
    });
    result.into_inner()
}

pub fn ensure_window(id: WindowId) {
    let now = std::time::Instant::now();
    log::trace!("ensure window is created {:?}", id);
    let result = EVENT_LOOP.try_lock();
    if result.is_err() {
        log::trace!("failed to grab a EVENT_LOOP lock");
        return;
    }
    let mut event_loop = result.unwrap();
    event_loop.el.run_return(|e, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
        match e {
            Event::WindowEvent { window_id, .. } => {
                if id == window_id {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        };
    });
    let elapsed = now.elapsed();
    log::trace!("window creation takes for {:?} in {:?}", id, elapsed);
}
