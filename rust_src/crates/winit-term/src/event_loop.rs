use crate::clipboard::Clipboard;
use crate::clipboard::ClipboardExt;
use errno::{set_errno, Errno};
use nix::sys::signal::{self, Signal};
use std::sync::OnceLock;
use std::{
    cell::RefCell,
    ptr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use emacs::bindings::{inhibit_window_system, make_timespec, thread_select};
#[cfg(all(x11_platform, not(use_tao)))]
use emacs::windowing::platform::x11::EventLoopWindowTargetExtX11;
use emacs::windowing::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
    monitor::MonitorHandle,
    platform::run_return::EventLoopExtRunReturn,
};
use libc::{c_void, fd_set, pselect, sigset_t, timespec};

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
    clipboard: Clipboard,
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

    pub fn get_clipboard(&mut self) -> &mut Clipboard {
        &mut self.clipboard
    }
}

pub static EVENT_LOOP: OnceLock<Arc<Mutex<WrEventLoop>>> = OnceLock::new();
impl WrEventLoop {
    pub fn global() -> &'static Arc<Mutex<WrEventLoop>> {
        EVENT_LOOP.get_or_init(|| {
            log::trace!("wr event loop is being created...");
            let (el, clipboard) = {
                #[cfg(not(use_tao))]
                let el = emacs::windowing::event_loop::EventLoopBuilder::<i32>::with_user_event()
                    .build();
                #[cfg(use_tao)]
                let el = EventLoop::<i32>::with_user_event();

                let clipboard = Clipboard::build(&el);
                (el, clipboard)
            };

            Arc::new(Mutex::new(Self { clipboard, el }))
        })
    }
}

pub fn global_event_buffer() -> &'static Mutex<Vec<GUIEvent>> {
    static EVENT_BUFFER: OnceLock<Mutex<Vec<GUIEvent>>> = OnceLock::new();
    EVENT_BUFFER.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn flush_events() -> Vec<GUIEvent> {
    let event_buffer = global_event_buffer().try_lock();

    if event_buffer.is_err() {
        return Vec::new();
    }

    let mut event_buffer = event_buffer.ok().unwrap();
    let events = event_buffer.clone();
    event_buffer.clear();
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
    log::trace!("winit select");
    let lock_result = WrEventLoop::global().try_lock();

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

    if let Ok(mut event_buffer) = global_event_buffer().try_lock() {
        // We mush run winit in main thread, because the macOS platfrom limitation.
        event_loop.el.run_return(|e, _target, control_flow| {
            *control_flow = ControlFlow::WaitUntil(deadline);

            if let Event::WindowEvent { event, .. } = &e {
                // Print only Window events to reduce noise
                log::trace!("{:?}", event);
            }

            let mut keyboard_event =
                |nfds_result: &RefCell<i32>, control_flow: &mut ControlFlow, e: Event<'_, i32>| {
                    event_buffer.push(e.to_static().unwrap());
                    // notify emacs's code that a keyboard event arrived.
                    match signal::raise(Signal::SIGIO) {
                        Ok(_) => {}
                        Err(err) => log::error!("sigio err: {err:?}"),
                    };
                    let _is_x11 = false;

                    #[cfg(all(not(use_tao), x11_platform))]
                    let _is_x11 = _target.is_x11();

                    if _is_x11 {
                        nfds_result.replace(1);
                    } else {
                        /* Pretend that `select' is interrupted by a signal.  */
                        set_errno(Errno(libc::EINTR));
                        debug_assert_eq!(nix::errno::errno(), libc::EINTR);
                        nfds_result.replace(-1);
                    }

                    *control_flow = ControlFlow::Exit;
                };

            match e {
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(_)
                    | WindowEvent::KeyboardInput { .. }
                    | WindowEvent::ModifiersChanged(_)
                    | WindowEvent::MouseInput { .. }
                    | WindowEvent::CursorMoved { .. }
                    | WindowEvent::ThemeChanged(_)
                    | WindowEvent::Focused(_)
                    | WindowEvent::MouseWheel { .. }
                    | WindowEvent::CloseRequested => {
                        keyboard_event(&nfds_result, control_flow, e);
                    }
                    #[cfg(not(use_tao))]
                    WindowEvent::ReceivedCharacter(_) => {
                        keyboard_event(&nfds_result, control_flow, e);
                    }

                    #[cfg(use_tao)]
                    WindowEvent::ReceivedImeText(_) => {
                        keyboard_event(&nfds_result, control_flow, e);
                    }
                    _ => {}
                },
                Event::UserEvent(nfds) => {
                    nfds_result.replace(nfds);
                    *control_flow = ControlFlow::Exit;
                }
                Event::RedrawRequested(_) => {
                    event_buffer.push(e.to_static().unwrap());
                    log::debug!("WindowEvent:: RedrawRequested");
                }
                Event::RedrawEventsCleared => {
                    event_buffer.push(e.to_static().unwrap());
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });
    }

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
    let result = WrEventLoop::global().try_lock();
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
                | WindowEvent::ModifiersChanged(_)
                | WindowEvent::MouseInput { .. }
                | WindowEvent::CursorMoved { .. }
                | WindowEvent::Focused(_)
                | WindowEvent::MouseWheel { .. }
                | WindowEvent::CloseRequested => {
                    result.replace(Some(e.to_static().unwrap()));
                    *control_flow = ControlFlow::Exit;
                }
                #[cfg(use_tao)]
                WindowEvent::ReceivedImeText(_) => {
                    result.replace(Some(e.to_static().unwrap()));
                    *control_flow = ControlFlow::Exit;
                }

                #[cfg(not(use_tao))]
                WindowEvent::ReceivedCharacter(_) => {
                    result.replace(Some(e.to_static().unwrap()));
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                result.replace(Some(e.to_static().unwrap()));
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

#[cfg(use_tao)]
pub fn ensure_window(id: emacs::windowing::window::WindowId) {
    let now = std::time::Instant::now();
    log::trace!("ensure window is created {:?}", id);
    let result = WrEventLoop::global().try_lock();
    if result.is_err() {
        log::trace!("failed to grab a EVENT_LOOP lock");
        return;
    }
    let mut event_loop = result.unwrap();
    event_loop.el.run_return(|e, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event, .. } = &e {
            log::trace!("{:?}", event);
        }

        match e {
            Event::WindowEvent {
                ref event,
                window_id,
                ..
            } => match event {
                WindowEvent::Focused(is_focused) => {
                    if id == window_id {
                        if *is_focused {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
    let elapsed = now.elapsed();
    log::trace!("window creation takes for {:?} in {:?}", id, elapsed);
}
