#[cfg(feature = "tao")]
use tao::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::{KeyCode as VirtualKeyCode, ModifiersState},
};
#[cfg(feature = "winit")]
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, ModifiersState, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode,
    },
};

use emacs::{
    bindings::{event_kind, input_event, scroll_bar_part},
    globals::{Qnil, Qt},
    lisp::LispObject,
    sys::EmacsModifiers::{
        ctrl_modifier, down_modifier, meta_modifier, shift_modifier, super_modifier, up_modifier,
    },
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

struct Modifiers {
    state: ModifiersState,
}

#[cfg(feature = "tao")]
impl Modifiers {
    pub fn new(state: ModifiersState) -> Modifiers {
        Modifiers { state }
    }

    pub fn shift_key(&self) -> bool {
        self.state.shift_key()
    }
    pub fn control_key(&self) -> bool {
        self.state.control_key()
    }
    pub fn alt_key(&self) -> bool {
        self.state.alt_key()
    }
    pub fn super_key(&self) -> bool {
        self.state.super_key()
    }

    pub fn set_modifiers(&self, input_processor: &mut InputProcessor) {
        if self.state.is_empty() {
            input_processor.modifiers = self.state;
        } else if self.shift_key() {
            input_processor
                .modifiers
                .set(ModifiersState::SHIFT, self.shift_key());
        } else if self.alt_key() {
            input_processor
                .modifiers
                .set(ModifiersState::ALT, self.alt_key());
        } else if self.super_key() {
            input_processor
                .modifiers
                .set(ModifiersState::SUPER, self.super_key());
        } else if self.control_key() {
            input_processor
                .modifiers
                .set(ModifiersState::CONTROL, self.control_key());
        }
    }
}

#[cfg(feature = "tao")]
fn virtual_keycode(code: VirtualKeyCode) -> u32 {
    let code = unsafe { std::mem::transmute::<VirtualKeyCode, i64>(code) };
    u32::try_from(code).unwrap()
}

#[cfg(feature = "winit")]
impl Modifiers {
    pub fn new(state: ModifiersState) -> Modifiers {
        Modifiers { state }
    }

    pub fn shift_key(&self) -> bool {
        self.state.shift()
    }
    pub fn control_key(&self) -> bool {
        self.state.ctrl()
    }
    pub fn alt_key(&self) -> bool {
        self.state.alt()
    }
    pub fn super_key(&self) -> bool {
        self.state.logo()
    }

    pub fn set_modifiers(&self, input_processor: &mut InputProcessor) {
        if self.state.is_empty() {
            input_processor.modifiers = self.state;
        } else if self.shift_key() {
            let val = self.shift_key();
            input_processor.modifiers.set(ModifiersState::SHIFT, val);
        } else if self.alt_key() {
            input_processor
                .modifiers
                .set(ModifiersState::ALT, self.alt_key());
        } else if self.super_key() {
            input_processor
                .modifiers
                .set(ModifiersState::LOGO, self.super_key());
        } else if self.control_key() {
            input_processor
                .modifiers
                .set(ModifiersState::CTRL, self.control_key());
        }
    }
}

#[cfg(feature = "winit")]
fn virtual_keycode(code: VirtualKeyCode) -> u32 {
    code as u32
}

pub static INPUT_PROCESSOR: Lazy<Mutex<InputProcessor>> =
    Lazy::new(|| Mutex::new(InputProcessor::new()));

pub struct InputProcessor {
    modifiers: ModifiersState,
    suppress_chars: bool,
    cursor_positon: PhysicalPosition<f64>,

    total_delta: PhysicalPosition<f64>,
}

impl InputProcessor {
    pub fn new() -> InputProcessor {
        InputProcessor {
            modifiers: ModifiersState::empty(),
            suppress_chars: false,
            cursor_positon: PhysicalPosition::new(0.0, 0.0),

            total_delta: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn receive_char(&self, c: char, top_frame: LispObject) -> Option<input_event> {
        if self.suppress_chars {
            return None;
        }

        let iev: input_event = InputEvent {
            kind: event_kind::ASCII_KEYSTROKE_EVENT,
            part: scroll_bar_part::scroll_bar_nowhere,
            code: Self::remove_control(c) as u32,
            modifiers: Self::to_emacs_modifiers(self.modifiers),
            x: 0.into(),
            y: 0.into(),
            timestamp: 0,
            frame_or_window: top_frame,
            arg: Qnil,
            device: Qt,
        }
        .into();

        Some(iev)
    }

    pub fn key_pressed(
        &mut self,
        key_code: VirtualKeyCode,
        top_frame: LispObject,
    ) -> Option<input_event> {
        if winit_keycode_emacs_key_name(key_code).is_null() {
            return None;
        }

        self.suppress_chars = true;
        let code = virtual_keycode(key_code);

        let iev: input_event = InputEvent {
            kind: event_kind::NON_ASCII_KEYSTROKE_EVENT,
            part: scroll_bar_part::scroll_bar_nowhere,
            code,
            modifiers: Self::to_emacs_modifiers(self.modifiers),
            x: 0.into(),
            y: 0.into(),
            timestamp: 0,
            frame_or_window: top_frame,
            arg: Qnil,
            device: Qt,
        }
        .into();

        Some(iev)
    }

    pub fn key_released(&mut self) {
        self.suppress_chars = false;
    }

    pub fn mouse_pressed(
        &self,
        button: MouseButton,
        state: ElementState,
        top_frame: LispObject,
    ) -> Option<input_event> {
        let c = match button {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
            MouseButton::Other(_) => 0,
            _ => todo!(),
        };

        let s = match state {
            ElementState::Pressed => down_modifier,
            ElementState::Released => up_modifier,
            _ => todo!(),
        };

        let iev: input_event = InputEvent {
            kind: event_kind::MOUSE_CLICK_EVENT,
            part: scroll_bar_part::scroll_bar_nowhere,
            code: c as u32,
            modifiers: Self::to_emacs_modifiers(self.modifiers) | s,
            x: (self.cursor_positon.x as i32).into(),
            y: (self.cursor_positon.y as i32).into(),
            timestamp: 0,
            frame_or_window: top_frame,
            arg: Qnil,
            device: Qt,
        }
        .into();

        Some(iev)
    }

    pub fn mouse_wheel_scrolled(
        &mut self,
        delta: MouseScrollDelta,
        phase: TouchPhase,
        top_frame: LispObject,
    ) -> Option<input_event> {
        if phase != TouchPhase::Moved {
            self.total_delta = PhysicalPosition::new(0.0, 0.0);
        }

        let line_height = top_frame.as_frame().unwrap().line_height as f64;

        let event_meta = match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                if y == 0.0 && x == 0.0 {
                    None
                } else if y != 0.0 {
                    let lines = y.abs() as i32;
                    Some((event_kind::WHEEL_EVENT, y > 0.0, lines))
                } else {
                    let lines = x.abs() as i32;
                    Some((event_kind::HORIZ_WHEEL_EVENT, x > 0.0, lines))
                }
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.total_delta.y = self.total_delta.y + pos.y;
                self.total_delta.x = self.total_delta.x + pos.x;

                if self.total_delta.y.abs() >= self.total_delta.x.abs()
                    && self.total_delta.y.abs() > line_height
                {
                    let lines = (self.total_delta.y / line_height).abs() as i32;

                    self.total_delta.y = self.total_delta.y % line_height;
                    self.total_delta.x = 0.0;

                    Some((event_kind::WHEEL_EVENT, self.total_delta.y > 0.0, lines))
                } else if self.total_delta.x.abs() > self.total_delta.y.abs()
                    && self.total_delta.x.abs() > line_height
                {
                    let lines = (self.total_delta.x / line_height).abs() as i32;

                    self.total_delta.x = self.total_delta.x % line_height;
                    self.total_delta.y = 0.0;

                    Some((
                        event_kind::HORIZ_WHEEL_EVENT,
                        self.total_delta.x > 0.0,
                        lines,
                    ))
                } else {
                    None
                }
            }
            _ => todo!(),
        };

        if event_meta.is_none() {
            return None;
        }

        let (kind, is_upper, lines) = event_meta.unwrap();

        let s = if is_upper { up_modifier } else { down_modifier };
        let iev: input_event = InputEvent {
            kind,
            part: scroll_bar_part::scroll_bar_nowhere,
            code: 0,
            modifiers: Self::to_emacs_modifiers(self.modifiers) | s,
            x: (self.cursor_positon.x as i32).into(),
            y: (self.cursor_positon.y as i32).into(),
            timestamp: 0,
            frame_or_window: top_frame,
            arg: lines.into(),
            device: Qt,
        }
        .into();

        Some(iev)
    }

    pub fn cursor_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor_positon = position;
    }

    pub fn change_modifiers(&mut self, state: ModifiersState) {
        let modifiers = Modifiers::new(state);
        modifiers.set_modifiers(self);
        log::trace!("modifier changed {:?}", self.modifiers);
    }

    pub fn current_cursor_position(&self) -> &PhysicalPosition<f64> {
        &self.cursor_positon
    }

    fn remove_control(c: char) -> char {
        let mut c = c as u8;

        if c < 32 {
            let mut new_c = c + 64;

            if new_c >= 65 && new_c <= 90 {
                new_c += 32;
            }

            c = new_c;
        }

        c as char
    }

    fn to_emacs_modifiers(modifiers: ModifiersState) -> u32 {
        let mut emacs_modifiers: u32 = 0;
        let modifiers = Modifiers::new(modifiers);

        if modifiers.alt_key() {
            emacs_modifiers |= meta_modifier;
        }
        if modifiers.shift_key() {
            emacs_modifiers |= shift_modifier;
        }
        if modifiers.control_key() {
            emacs_modifiers |= ctrl_modifier;
        }
        if modifiers.super_key() {
            emacs_modifiers |= super_modifier;
        }

        emacs_modifiers
    }
}

// macro for building key_name c string
macro_rules! kn {
    ($e:expr) => {
        concat!($e, '\0').as_ptr() as *const libc::c_char
    };
}

pub fn winit_keycode_emacs_key_name(keycode: VirtualKeyCode) -> *const libc::c_char {
    match keycode {
        VirtualKeyCode::Escape => kn!("escape"),
        #[cfg(feature = "winit")]
        VirtualKeyCode::Back => kn!("backspace"),
        #[cfg(feature = "tao")]
        VirtualKeyCode::Backspace => kn!("backspace"),
        #[cfg(feature = "winit")]
        VirtualKeyCode::Return => kn!("return"),
        #[cfg(feature = "tao")]
        VirtualKeyCode::Enter => kn!("return"),
        VirtualKeyCode::Tab => kn!("tab"),

        VirtualKeyCode::Home => kn!("home"),
        VirtualKeyCode::End => kn!("end"),
        VirtualKeyCode::PageUp => kn!("prior"),
        VirtualKeyCode::PageDown => kn!("next"),

        #[cfg(feature = "winit")]
        VirtualKeyCode::Left => kn!("left"),
        #[cfg(feature = "tao")]
        VirtualKeyCode::ArrowLeft => kn!("left"),
        #[cfg(feature = "winit")]
        VirtualKeyCode::Right => kn!("right"),
        #[cfg(feature = "tao")]
        VirtualKeyCode::ArrowRight => kn!("right"),
        #[cfg(feature = "winit")]
        VirtualKeyCode::Up => kn!("up"),
        #[cfg(feature = "tao")]
        VirtualKeyCode::ArrowUp => kn!("up"),
        #[cfg(feature = "winit")]
        VirtualKeyCode::Down => kn!("down"),
        #[cfg(feature = "tao")]
        VirtualKeyCode::ArrowDown => kn!("down"),

        VirtualKeyCode::Insert => kn!("insert"),

        VirtualKeyCode::F1 => kn!("f1"),
        VirtualKeyCode::F2 => kn!("f2"),
        VirtualKeyCode::F3 => kn!("f3"),
        VirtualKeyCode::F4 => kn!("f4"),
        VirtualKeyCode::F5 => kn!("f5"),
        VirtualKeyCode::F6 => kn!("f6"),
        VirtualKeyCode::F7 => kn!("f7"),
        VirtualKeyCode::F8 => kn!("f8"),
        VirtualKeyCode::F9 => kn!("f9"),
        VirtualKeyCode::F10 => kn!("f10"),
        VirtualKeyCode::F11 => kn!("f11"),
        VirtualKeyCode::F12 => kn!("f12"),
        VirtualKeyCode::F13 => kn!("f13"),
        VirtualKeyCode::F14 => kn!("f14"),
        VirtualKeyCode::F15 => kn!("f15"),
        VirtualKeyCode::F16 => kn!("f16"),
        VirtualKeyCode::F17 => kn!("f17"),
        VirtualKeyCode::F18 => kn!("f18"),
        VirtualKeyCode::F19 => kn!("f19"),
        VirtualKeyCode::F20 => kn!("f20"),
        VirtualKeyCode::F21 => kn!("f21"),
        VirtualKeyCode::F22 => kn!("f22"),
        VirtualKeyCode::F23 => kn!("f23"),
        VirtualKeyCode::F24 => kn!("f24"),

        _ => std::ptr::null(), // null pointer
    }
}

pub struct InputEvent {
    pub kind: event_kind::Type,
    pub part: scroll_bar_part::Type,
    pub code: ::libc::c_uint,
    pub modifiers: ::libc::c_uint,
    pub x: LispObject,
    pub y: LispObject,
    pub timestamp: emacs::bindings::Time,
    pub frame_or_window: LispObject,
    pub arg: LispObject,
    pub device: LispObject,
}

impl From<InputEvent> for input_event {
    fn from(val: InputEvent) -> Self {
        let mut iev = input_event::default();
        iev.set_kind(val.kind);
        iev.set_part(val.part);
        iev.code = val.code;
        iev.modifiers = val.modifiers;
        iev.x = val.x;
        iev.y = val.y;
        iev.timestamp = val.timestamp;
        iev.frame_or_window = val.frame_or_window;
        iev.arg = val.arg;
        iev.device = val.device;
        iev
    }
}
