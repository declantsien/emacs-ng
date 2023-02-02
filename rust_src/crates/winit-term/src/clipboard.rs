use emacs::windowing::event_loop::EventLoop;

#[cfg(use_tao)]
pub type Clipboard = emacs::windowing::clipboard::Clipboard;
#[cfg(not(use_tao))]
pub type Clipboard = Box<dyn copypasta::ClipboardProvider>;

pub trait ClipboardExt {
    fn build(event_loop: &EventLoop<i32>) -> Self;
    fn write(&mut self, content: String);
    fn read(&mut self) -> String;
}

#[cfg(use_tao)]
impl ClipboardExt for Clipboard {
    fn build(_: &EventLoop<i32>) -> Self {
        emacs::windowing::clipboard::Clipboard::new()
    }
    fn write(&mut self, content: String) {
        self.write_text(content);
    }

    fn read(&mut self) -> String {
        match &self.read_text() {
            Some(s) => s.to_string(),
            None => String::from(""),
        }
    }
}

#[cfg(not(use_tao))]
impl ClipboardExt for Clipboard {
    fn build(_event_loop: &EventLoop<i32>) -> Self {
        #[cfg(free_unix)]
        {
            use crate::emacs::windowing::platform::wayland::EventLoopWindowTargetExtWayland;
            if _event_loop.is_wayland() {
                let wayland_display = _event_loop
                    .wayland_display()
                    .expect("Fetch Wayland display failed");
                let (_, clipboard) = unsafe {
                    copypasta::wayland_clipboard::create_clipboards_from_external(wayland_display)
                };
                Box::new(clipboard)
            } else {
                Box::new(
                    copypasta::x11_clipboard::X11ClipboardContext::<
                        copypasta::x11_clipboard::Clipboard,
                    >::new()
                    .unwrap(),
                )
            }
        }
        #[cfg(windows_platform)]
        {
            return Box::new(WindowsClipboardContext::new().unwrap());
        }
        #[cfg(macos_platform)]
        {
            return Box::new(OSXClipboardContext::new().unwrap());
        }
    }

    fn write(&mut self, content: String) {
        if let Err(err) = self.set_contents(content) {
            log::error!("Failed to write to clipboard {err:?}")
        };
    }

    fn read(&mut self) -> String {
        match &self.get_contents() {
            Ok(s) => s.to_string(),
            Err(_) => String::from(""),
        }
    }
}
