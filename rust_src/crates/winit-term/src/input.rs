use std::sync::OnceLock;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
    keyboard::{KeyCode as VirtualKeyCode, ModifiersState},
};

use emacs::{
    bindings::{event_kind, input_event, scroll_bar_part},
    globals::{Qnil, Qt},
    lisp::LispObject,
    sys::EmacsModifiers::{
        ctrl_modifier, down_modifier, meta_modifier, shift_modifier, super_modifier, up_modifier,
    },
};

use crate::frame::LispFrameWinitExt;

static mut INPUT_STATE: OnceLock<InputProcessor> = OnceLock::new();
impl InputProcessor {
    pub fn global() -> &'static InputProcessor {
        unsafe {
            INPUT_STATE.get_or_init(|| {
                log::trace!("INPUT_STATE is being created...");
                InputProcessor {
                    modifiers: ModifiersState::default(),
                    total_delta: PhysicalPosition::new(0.0, 0.9),
                }
            })
        }
    }

    fn update(new_state: InputProcessor) {
        log::trace!(
            "modifer changed {:?} {:?}",
            new_state.modifiers,
            new_state.total_delta
        );
        unsafe {
            let _ = INPUT_STATE.take();
        };
        if let Ok(_) = unsafe { INPUT_STATE.set(new_state) } {
            log::debug!("Global input state changed");
        } else {
            log::error!("Failed to update input state");
        }
    }
}

pub struct InputProcessor {
    modifiers: ModifiersState,
    total_delta: PhysicalPosition<f64>,
}

impl InputProcessor {
    pub fn handle_modifiers_changed(new_state: ModifiersState) {
        let InputProcessor {
            modifiers,
            total_delta,
        } = Self::global();
        let mut modifiers = modifiers.clone();
        if new_state.is_empty() {
            modifiers = new_state;
        } else if new_state.shift_key() {
            modifiers.set(ModifiersState::SHIFT, new_state.shift_key());
        } else if new_state.control_key() {
            modifiers.set(ModifiersState::CONTROL, new_state.control_key());
        } else if new_state.alt_key() {
            modifiers.set(ModifiersState::ALT, new_state.alt_key());
        } else if new_state.super_key() {
            modifiers.set(ModifiersState::SUPER, new_state.super_key());
        }

        Self::update(InputProcessor {
            modifiers,
            total_delta: total_delta.clone(),
        });
    }

    fn set_total_delta(total_delta: PhysicalPosition<f64>) {
        let InputProcessor { modifiers, .. } = Self::global();
        Self::update(InputProcessor {
            modifiers: modifiers.clone(),
            total_delta,
        });
    }

    fn get_modifiers() -> ModifiersState {
        let InputProcessor { modifiers, .. } = Self::global();
        modifiers.clone()
    }
}

impl InputProcessor {
    pub fn handle_receive_char(c: char, top_frame: LispObject) -> Option<input_event> {
        let iev: input_event = InputEvent {
            kind: event_kind::ASCII_KEYSTROKE_EVENT,
            part: scroll_bar_part::scroll_bar_nowhere,
            code: Self::remove_control(c) as u32,
            modifiers: Self::to_emacs_modifiers(Self::get_modifiers()),
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

    pub fn handle_key_pressed(
        key_code: VirtualKeyCode,
        top_frame: LispObject,
    ) -> Option<input_event> {
        let InputProcessor { modifiers, .. } = Self::global().clone();
        if winit_keycode_emacs_key_name(key_code).is_null() {
            return None;
        }

        let code = unsafe { std::mem::transmute::<VirtualKeyCode, i64>(key_code) };
        let code = u32::try_from(code).unwrap();

        let iev: input_event = InputEvent {
            kind: event_kind::NON_ASCII_KEYSTROKE_EVENT,
            part: scroll_bar_part::scroll_bar_nowhere,
            code,
            modifiers: Self::to_emacs_modifiers(modifiers.to_owned()),
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

    pub fn handle_key_released() {}

    pub fn handle_mouse_pressed(
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

        let mut pos = PhysicalPosition::new(0, 0);

        if let Some(frame) = top_frame.as_frame() {
            pos = frame.cursor_position();
        }

        let InputProcessor { modifiers, .. } = Self::global();
        let iev: input_event = InputEvent {
            kind: event_kind::MOUSE_CLICK_EVENT,
            part: scroll_bar_part::scroll_bar_nowhere,
            code: c as u32,
            modifiers: Self::to_emacs_modifiers(modifiers.clone()) | s,
            x: pos.x.into(),
            y: pos.y.into(),
            timestamp: 0,
            frame_or_window: top_frame,
            arg: Qnil,
            device: Qt,
        }
        .into();

        Some(iev)
    }

    pub fn handle_mouse_wheel_scrolled(
        delta: MouseScrollDelta,
        phase: TouchPhase,
        top_frame: LispObject,
    ) -> Option<input_event> {
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
                let mut total_delta = Self::global().total_delta.clone();
                if phase != TouchPhase::Moved {
                    total_delta = PhysicalPosition::new(0.0, 0.0);
                }
                total_delta.y = total_delta.y + pos.y;
                total_delta.x = total_delta.x + pos.x;

                if total_delta.y.abs() >= total_delta.x.abs() && total_delta.y.abs() > line_height {
                    let lines = (total_delta.y / line_height).abs() as i32;

                    total_delta.y = total_delta.y % line_height;
                    total_delta.x = 0.0;

                    let _ = Self::set_total_delta(total_delta.clone());

                    Some((event_kind::WHEEL_EVENT, total_delta.y > 0.0, lines))
                } else if total_delta.x.abs() > total_delta.y.abs()
                    && total_delta.x.abs() > line_height
                {
                    let lines = (total_delta.x / line_height).abs() as i32;

                    total_delta.x = total_delta.x % line_height;
                    total_delta.y = 0.0;

                    let _ = Self::set_total_delta(total_delta.clone());

                    Some((event_kind::HORIZ_WHEEL_EVENT, total_delta.x > 0.0, lines))
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

        let mut pos = PhysicalPosition::new(0, 0);

        if let Some(frame) = top_frame.as_frame() {
            pos = frame.cursor_position();
        }

        let s = if is_upper { up_modifier } else { down_modifier };
        let iev: input_event = InputEvent {
            kind,
            part: scroll_bar_part::scroll_bar_nowhere,
            code: 0,
            modifiers: Self::to_emacs_modifiers(Self::get_modifiers()) | s,
            x: pos.x.into(),
            y: pos.y.into(),
            timestamp: 0,
            frame_or_window: top_frame,
            arg: lines.into(),
            device: Qt,
        }
        .into();

        Some(iev)
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
        VirtualKeyCode::Backspace => kn!("backspace"),
        VirtualKeyCode::Delete => kn!("deletechar"),
        VirtualKeyCode::Enter
            |VirtualKeyCode::NumpadEnter=> kn!("return"),
        VirtualKeyCode::Tab => kn!("tab"),

        VirtualKeyCode::Home => kn!("home"),
        VirtualKeyCode::End => kn!("end"),
        VirtualKeyCode::PageUp => kn!("prior"),
        VirtualKeyCode::PageDown => kn!("next"),

        VirtualKeyCode::ArrowLeft => kn!("left"),
        VirtualKeyCode::ArrowRight => kn!("right"),
        VirtualKeyCode::ArrowUp => kn!("up"),
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
