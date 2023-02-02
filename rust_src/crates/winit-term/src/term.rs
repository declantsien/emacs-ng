use crate::event_loop::flush_events;
use crate::event_loop::poll_a_event;
use crate::event_loop::EVENT_LOOP;
use crate::winit_term::frame_uuid;
use emacs::bindings::{
    add_keyboard_wait_descriptor, init_sigio, interrupt_input, Fwaiting_for_user_input_p,
};
use lazy_static::lazy_static;
#[cfg(wayland_platform)]
use raw_window_handle::WaylandDisplayHandle;
use raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle};
#[cfg(x11_platform)]
use raw_window_handle::{XcbDisplayHandle, XlibDisplayHandle};
use std::ptr;
use std::time::Duration;
#[cfg(feature = "tao")]
use tao::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, KeyEvent, WindowEvent, WindowEvent::KeyboardInput},
    keyboard::{Key, KeyCode},
    window::WindowBuilder,
};
#[cfg(wayland_platform)]
use wayland_sys::client::{wl_display, WAYLAND_CLIENT_HANDLE};
#[cfg(feature = "winit")]
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    window::WindowBuilder,
};

use crate::winit_term::WINIT_WINDOWS;
use webrender_bindings::DeviceIntSize;
use webrender_bindings::*;

use crate::cursor::emacs_to_winit_cursor;
use crate::event::create_emacs_event;
use crate::event_loop::EVENT_BUFFER;
use crate::input::INPUT_PROCESSOR;
use crate::winit_term::remove_winit_window;
use webrender_bindings::display_info::{DisplayInfo, DisplayInfoRef};
use webrender_bindings::frame::LispFrameExt;

use emacs::{
    bindings::{
        create_terminal, current_kboard, frame_parm_handler, fullscreen_type, initial_kboard,
        note_mouse_highlight, output_method, redisplay_interface, scroll_bar_part, terminal,
        xlispstrdup, Emacs_Cursor, Fcons,
    },
    bindings::{
        do_pending_window_change, gui_clear_end_of_line, gui_clear_window_mouse_face,
        gui_fix_overlapping_area, gui_get_glyph_overhangs, gui_produce_glyphs, gui_set_alpha,
        gui_set_autolower, gui_set_autoraise, gui_set_border_width, gui_set_bottom_divider_width,
        gui_set_font, gui_set_font_backend, gui_set_fullscreen, gui_set_horizontal_scroll_bars,
        gui_set_left_fringe, gui_set_line_spacing, gui_set_no_special_glyphs,
        gui_set_right_divider_width, gui_set_right_fringe, gui_set_screen_gamma,
        gui_set_scroll_bar_height, gui_set_scroll_bar_width, gui_set_unsplittable,
        gui_set_vertical_scroll_bars, gui_set_visibility, gui_write_glyphs, input_event,
        kbd_buffer_store_event_hold, Time, PT_PER_INCH,
    },
    frame::{all_frames, LispFrameRef, Lisp_Frame},
    globals::{Qfullscreen, Qmaximized, Qnil, Qwinit},
    keyboard::allocate_keyboard,
    lisp::{ExternalPtr, LispObject},
};

pub type TerminalRef = ExternalPtr<terminal>;

fn get_frame_parm_handlers() -> [frame_parm_handler; 48] {
    // Keep this list in the same order as frame_parms in frame.c.
    // Use None for unsupported frame parameters.
    let handlers: [frame_parm_handler; 48] = [
        Some(gui_set_autoraise),
        Some(gui_set_autolower),
        Some(wr_set_background_color),
        None,
        Some(gui_set_border_width),
        Some(wr_set_cursor_color),
        None,
        Some(gui_set_font),
        None,
        None,
        None,
        None,
        None,
        Some(gui_set_right_divider_width),
        Some(gui_set_bottom_divider_width),
        None,
        None,
        None,
        Some(gui_set_scroll_bar_width),
        Some(gui_set_scroll_bar_height),
        None,
        Some(gui_set_unsplittable),
        Some(gui_set_vertical_scroll_bars),
        Some(gui_set_horizontal_scroll_bars),
        Some(gui_set_visibility),
        None,
        None,
        None,
        None,
        Some(gui_set_screen_gamma),
        Some(gui_set_line_spacing),
        Some(gui_set_left_fringe),
        Some(gui_set_right_fringe),
        None,
        Some(gui_set_fullscreen),
        Some(gui_set_font_backend),
        Some(gui_set_alpha),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(gui_set_no_special_glyphs),
    ];

    handlers
}

struct RedisplayInterface(pub redisplay_interface);
unsafe impl Sync for RedisplayInterface {}

lazy_static! {
    static ref REDISPLAY_INTERFACE: RedisplayInterface = {
        let frame_parm_handlers = Box::new(get_frame_parm_handlers());

        let interface = redisplay_interface {
            frame_parm_handlers: (Box::into_raw(frame_parm_handlers)) as *mut Option<_>,
            produce_glyphs: Some(gui_produce_glyphs),
            write_glyphs: Some(gui_write_glyphs),
            insert_glyphs: None,
            clear_end_of_line: Some(gui_clear_end_of_line),
            clear_under_internal_border: None,
            scroll_run_hook: Some(wr_scroll_run),
            after_update_window_line_hook: Some(wr_after_update_window_line),
            update_window_begin_hook: Some(wr_update_window_begin),
            update_window_end_hook: Some(wr_update_window_end),
            flush_display: Some(wr_flush_display),
            clear_window_mouse_face: Some(gui_clear_window_mouse_face),
            get_glyph_overhangs: Some(gui_get_glyph_overhangs),
            fix_overlapping_area: Some(gui_fix_overlapping_area),
            draw_fringe_bitmap: Some(wr_draw_fringe_bitmap),
            define_fringe_bitmap: None,
            destroy_fringe_bitmap: None,
            compute_glyph_string_overhangs: None,
            draw_glyph_string: Some(wr_draw_glyph_string),
            define_frame_cursor: Some(winit_define_frame_cursor),
            default_font_parameter: None,
            clear_frame_area: Some(wr_clear_frame_area),
            draw_window_cursor: Some(wr_draw_window_cursor),
            draw_vertical_window_border: Some(wr_draw_vertical_window_border),
            draw_window_divider: Some(wr_draw_window_divider),
            shift_glyphs_for_insert: None,
            show_hourglass: None,
            hide_hourglass: None,
        };

        RedisplayInterface(interface)
    };
}

extern "C" fn get_string_resource(
    _rdb: *mut libc::c_void,
    _name: *const libc::c_char,
    _class: *const libc::c_char,
) -> *const libc::c_char {
    ptr::null()
}

extern "C" fn winit_frame_visible_invisible(frame: *mut Lisp_Frame, is_visible: bool) {
    let mut f: LispFrameRef = frame.into();

    f.set_visible(is_visible as u32);

    let uuid = f.uuid();
    let wins = WINIT_WINDOWS.try_lock().unwrap();
    let window = wins.get(&uuid).unwrap();

    if is_visible {
        window.set_visible(true);
    } else {
        window.set_visible(false);
    }
}

extern "C" fn winit_define_frame_cursor(f: *mut Lisp_Frame, cursor: Emacs_Cursor) {
    let frame: LispFrameRef = f.into();
    let uuid = frame.uuid();

    let cursor = emacs_to_winit_cursor(cursor);
    WINIT_WINDOWS
        .try_lock()
        .unwrap()
        .get(&uuid)
        .unwrap()
        .set_cursor_icon(cursor);
}

extern "C" fn winit_read_input_event(terminal: *mut terminal, hold_quit: *mut input_event) -> i32 {
    let terminal: TerminalRef = terminal.into();
    let mut dpyinfo = DisplayInfoRef::new(unsafe { terminal.display_info.winit } as *mut _);

    let dpyinfo = dpyinfo.get_inner();
    let input_processor = INPUT_PROCESSOR.try_lock();

    if input_processor.is_err() {
        return 0;
    }
    let mut input_processor = input_processor.unwrap();

    let mut count = 0;

    let mut events = flush_events();
    if events.len() == 0 && unsafe { Fwaiting_for_user_input_p() }.is_nil() {
        if let Some(rwh) = dpyinfo.raw_display_handle {
            match rwh {
                // #[cfg(wayland_platform)]
                // RawDisplayHandle::Wayland(WaylandDisplayHandle { .. }) => {
                //     if let Some(event) = poll_a_event(Duration::from_millis(0)) {
                //         events.push(event);
                //     }
                // }
                // #[cfg(x11_platform)]
                // RawDisplayHandle::Xlib(XlibDisplayHandle { .. })
                // | RawDisplayHandle::Xcb(XcbDisplayHandle { .. }) => {
                //     if let Some(event) = poll_a_event(Duration::from_millis(1)) {
                //         events.push(event);
                //     }
                // }
                _ => {
                    if let Some(event) = poll_a_event(Duration::from_millis(1)) {
                        events.push(event);
                    }
                }
            }
        }
    }

    for e in events.iter() {
        let e = e.clone();

        match e {
            Event::WindowEvent {
                window_id, event, ..
            } => {
                let uuid = frame_uuid(&window_id);

                if uuid.is_none() {
                    continue;
                }
                let uuid = uuid.unwrap();

                let frame = dpyinfo.frames.get(&uuid);

                if frame.is_none() {
                    continue;
                }

                let frame: LispFrameRef = *frame.unwrap();
                let mut canvas = frame.canvas();
                let frame: LispObject = frame.into();

                match event {
                    WindowEvent::ReceivedImeText(text) => {
                        // for (_i, key_code) in text.chars().enumerate() {
                        //     if let Some(mut iev) = input_processor.receive_char(key_code, frame) {
                        //         unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                        //         count += 1;
                        //     }
                        // }
                    }

                    WindowEvent::ModifiersChanged(state) => {
                        input_processor.change_modifiers(state);
                    }

                    WindowEvent::KeyboardInput { event, .. } => match event.state {
                        ElementState::Pressed => {
                            if let Some(mut iev) =
                                input_processor.key_pressed(event.physical_key, frame)
                            {
                                unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                                count += 1;
                            }

                            // if let Some(text) = event.key_without_modifiers().to_text() {
                            if let Some(text) = event.logical_key.to_text() {
                                for (_i, key_code) in text.chars().enumerate() {
                                    if let Some(mut iev) =
                                        input_processor.receive_char(key_code, frame)
                                    {
                                        unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                                        count += 1;
                                    }
                                }
                            }
                        }
                        ElementState::Released => {
                            input_processor.key_released();
                        }
                        _ => todo!(),
                    },

                    WindowEvent::MouseInput { state, button, .. } => {
                        if let Some(mut iev) = input_processor.mouse_pressed(button, state, frame) {
                            unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                            count += 1;
                        }
                    }

                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        if let Some(mut iev) =
                            input_processor.mouse_wheel_scrolled(delta, phase, frame)
                        {
                            unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                            count += 1;
                        }

                        let mut frame: LispFrameRef = frame.into();
                        frame.set_mouse_moved(false);
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let mut frame: LispFrameRef = frame.into();

                        unsafe {
                            note_mouse_highlight(
                                frame.as_mut(),
                                position.x as i32,
                                position.y as i32,
                            )
                        };

                        input_processor.cursor_move(position);

                        frame.set_mouse_moved(true);
                    }

                    WindowEvent::Focused(is_focused) => {
                        let mut dpyinfo =
                            DisplayInfoRef::new(unsafe { terminal.display_info.winit } as *mut _);

                        let mut top_frame = frame.as_frame().unwrap();

                        let focus_frame = if !top_frame.focus_frame.eq(Qnil) {
                            top_frame.focus_frame.as_frame().unwrap().as_mut()
                        } else {
                            top_frame.as_mut()
                        };

                        dpyinfo.get_raw().highlight_frame = if is_focused {
                            focus_frame
                        } else {
                            ptr::null_mut()
                        };

                        let event_type = if is_focused {
                            emacs::bindings::event_kind::FOCUS_IN_EVENT
                        } else {
                            emacs::bindings::event_kind::FOCUS_OUT_EVENT
                        };

                        let mut event = create_emacs_event(event_type, top_frame.into());

                        unsafe { kbd_buffer_store_event_hold(&mut event, hold_quit) };
                        count += 1;
                    }

                    WindowEvent::Resized(size) => {
                        let size = { DeviceIntSize::new(size.width as i32, size.height as i32) };
                        canvas.resize(&size);

                        let frame: LispFrameRef = frame.into();
                        frame.change_size(
                            size.width as i32,
                            size.height as i32 - frame.menu_bar_height,
                            false,
                            true,
                            false,
                        );

                        unsafe { do_pending_window_change(false) };
                    }

                    WindowEvent::CloseRequested => {
                        let mut event = create_emacs_event(
                            emacs::bindings::event_kind::DELETE_WINDOW_EVENT,
                            frame,
                        );

                        unsafe { kbd_buffer_store_event_hold(&mut event, hold_quit) };
                        count += 1;
                    }

                    _ => {}
                }
            }
            _ => {}
        };
    }

    count
}

extern "C" fn winit_fullscreen(f: *mut Lisp_Frame) {
    let frame: LispFrameRef = f.into();

    if !frame.is_visible() {
        return;
    }

    if frame.want_fullscreen() == fullscreen_type::FULLSCREEN_MAXIMIZED {
        let uuid = frame.uuid();
        let wins = WINIT_WINDOWS.try_lock().unwrap();
        let window = wins.get(&uuid).unwrap();
        window.set_maximized(true);
        frame.store_param(Qfullscreen, Qmaximized);
    }
}

// This function should be called by Emacs redisplay code to set the
// name; names set this way will never override names set by the user's
// lisp code.
extern "C" fn winit_implicitly_set_name(
    frame: *mut Lisp_Frame,
    arg: LispObject,
    _oldval: LispObject,
) {
    let mut frame: LispFrameRef = frame.into();

    if frame.name.eq(arg) {
        return;
    }

    frame.name = arg;

    let title = format!("{}", arg.force_string());
    let uuid = frame.uuid();
    WINIT_WINDOWS
        .try_lock()
        .unwrap()
        .get(&uuid)
        .unwrap()
        .set_title(&title);
}

extern "C" fn winit_get_focus_frame(frame: *mut Lisp_Frame) -> LispObject {
    let frame: LispFrameRef = frame.into();
    let mut dpyinfo = frame.display_info();

    let focus_frame = dpyinfo.get_inner().focus_frame;

    match focus_frame.is_null() {
        true => Qnil,
        false => focus_frame.into(),
    }
}

// This tries to wait until the frame is really visible, depending on
// the value of Vx_wait_for_event_timeout.
// However, if the window manager asks the user where to position
// the frame, this will return before the user finishes doing that.
// The frame will not actually be visible at that time,
// but it will become visible later when the window manager
// finishes with it.
extern "C" fn winit_make_frame_visible_invisible(f: *mut Lisp_Frame, visible: bool) {
    let mut frame: LispFrameRef = f.into();

    frame.set_visible(visible as u32);
    let uuid = frame.uuid();
    log::info!("uuid: {uuid:?}");
    let wins = WINIT_WINDOWS.try_lock().unwrap();
    let winit_window = wins.get(&uuid).unwrap();

    if visible {
        winit_window.set_visible(true);
    } else {
        winit_window.set_visible(false);
    }
}

extern "C" fn winit_iconify_frame(f: *mut Lisp_Frame) {
    let mut frame: LispFrameRef = f.into();

    frame.set_iconified(true);

    let uuid = frame.uuid();
    WINIT_WINDOWS
        .try_lock()
        .unwrap()
        .get(&uuid)
        .unwrap()
        .set_visible(false);
}

extern "C" fn winit_mouse_position(
    fp: *mut *mut Lisp_Frame,
    _insist: i32,
    bar_window: *mut LispObject,
    part: *mut scroll_bar_part::Type,
    x: *mut LispObject,
    y: *mut LispObject,
    _timestamp: *mut Time,
) {
    let dpyinfo = {
        let frame: LispFrameRef = unsafe { (*fp).into() };
        frame.display_info()
    };

    // Clear the mouse-moved flag for every frame on this display.
    for mut frame in all_frames() {
        if frame.display_info() == dpyinfo {
            frame.set_mouse_moved(false);
        }
    }

    unsafe { *bar_window = Qnil };
    unsafe { *part = 0 };

    let cursor_pos: PhysicalPosition<i32> = INPUT_PROCESSOR
        .try_lock()
        .unwrap()
        .current_cursor_position()
        .cast();

    unsafe { *x = cursor_pos.x.into() };
    unsafe { *y = cursor_pos.y.into() };
}

// cleanup frame resource after frame is deleted
extern "C" fn winit_destroy_frame(f: *mut Lisp_Frame) {
    let frame: LispFrameRef = f.into();
    let mut output = frame.output();
    let mut data = frame.canvas();
    let mut display_info = frame.display_info();
    let uuid = frame.uuid();

    display_info.get_inner().frames.remove(&uuid);
    remove_winit_window(&uuid);

    // Take back output ownership and destroy it
    let _ = unsafe { Box::from_raw(data.as_mut()).deinit() };
    output.empty_inner();
}

fn wr_create_terminal(mut dpyinfo: DisplayInfoRef) -> TerminalRef {
    let terminal_ptr = unsafe {
        create_terminal(
            output_method::output_winit,
            &REDISPLAY_INTERFACE.0 as *const _ as *mut _,
        )
    };

    let mut terminal = TerminalRef::new(terminal_ptr);

    // Link terminal and dpyinfo together
    terminal.display_info.winit = dpyinfo.get_raw().as_mut();
    dpyinfo.get_inner().terminal = terminal;
    dpyinfo.get_raw().terminal = terminal.as_mut();

    // Terminal hooks
    // Other hooks are NULL by default.
    terminal.get_string_resource_hook = Some(get_string_resource);
    terminal.set_new_font_hook = Some(wr_new_font);
    terminal.defined_color_hook = Some(wr_defined_color);
    terminal.frame_visible_invisible_hook = Some(winit_frame_visible_invisible);
    terminal.clear_frame_hook = Some(wr_clear_frame);
    terminal.read_socket_hook = Some(winit_read_input_event);
    terminal.fullscreen_hook = Some(winit_fullscreen);
    terminal.implicit_set_name_hook = Some(winit_implicitly_set_name);
    terminal.get_focus_frame = Some(winit_get_focus_frame);
    terminal.frame_visible_invisible_hook = Some(winit_make_frame_visible_invisible);
    terminal.iconify_frame_hook = Some(winit_iconify_frame);
    terminal.mouse_position_hook = Some(winit_mouse_position);
    terminal.update_end_hook = Some(wr_update_end);
    terminal.free_pixmap = Some(wr_free_pixmap);
    terminal.delete_frame_hook = Some(winit_destroy_frame);

    terminal
}

pub fn winit_term_init(display_name: LispObject) -> DisplayInfoRef {
    log::info!("Emacs Webrender term init");

    let dpyinfo = Box::new(DisplayInfo::new());
    let mut dpyinfo_ref = DisplayInfoRef::new(Box::into_raw(dpyinfo));

    let event_loop = EVENT_LOOP.try_lock().unwrap();
    let window_builder = WindowBuilder::new().with_visible(false);
    let window = window_builder.build(&event_loop.el()).unwrap();
    let raw_handle = window.raw_display_handle();
    let scale_factor = window.scale_factor();

    dpyinfo_ref.get_inner().raw_display_handle = Some(raw_handle);
    dpyinfo_ref.get_inner().scale_factor = scale_factor as f32;

    println!("scale factor {:?}", scale_factor);

    let mut _conn = None;

    match raw_handle {
        #[cfg(wayland_platform)]
        RawDisplayHandle::Wayland(WaylandDisplayHandle { display, .. }) => {
            log::trace!("wayland display {display:?}");
            let fd =
                unsafe { (WAYLAND_CLIENT_HANDLE.wl_display_get_fd)(display as *mut wl_display) };
            log::trace!("wayland display fd {fd:?}");
            _conn = Some(fd);
        }
        #[cfg(x11_platform)]
        RawDisplayHandle::Xlib(XlibDisplayHandle { display, .. }) => {
            log::trace!("xlib display {display:?}");
            let fd = unsafe { x11::xlib::XConnectionNumber(display as *mut x11::xlib::Display) };
            log::trace!("xlib display fd {fd:?}");
            _conn = Some(fd);
        }
        #[cfg(x11_platform)]
        RawDisplayHandle::Xcb(XcbDisplayHandle { .. }) => {} // How does this differs from xlib?
        _ => {
            _conn = Some(0);
        }
    }

    if let Some(fd) = _conn {
        unsafe {
            add_keyboard_wait_descriptor(fd);
            libc::fcntl(fd, libc::F_SETOWN, libc::getpid());
            if interrupt_input {
                init_sigio(fd);
            }
        };
    }

    let mut terminal = wr_create_terminal(dpyinfo_ref);

    let mut kboard = allocate_keyboard(Qwinit);

    terminal.kboard = kboard.as_mut();

    // Don't let the initial kboard remain current longer than necessary.
    // That would cause problems if a file loaded on startup tries to
    // prompt in the mini-buffer.
    unsafe {
        if current_kboard == initial_kboard {
            current_kboard = terminal.kboard;
        }
    }

    kboard.add_ref();

    {
        let mut dpyinfo_ref = dpyinfo_ref.get_raw();
        dpyinfo_ref.name_list_element = unsafe { Fcons(display_name, Qnil) };

        // https://lists.gnu.org/archive/html/emacs-devel/2015-11/msg00194.html
        dpyinfo_ref.smallest_font_height = 1;
        dpyinfo_ref.smallest_char_width = 1;

        // we have https://docs.rs/winit/0.23.0/winit/dpi/index.html
        // set to base DPI PT_PER_INCH to equal out POINT_TO_PIXEL/PIXEL_TO_POINT
        dpyinfo_ref.resx = PT_PER_INCH;
        dpyinfo_ref.resy = PT_PER_INCH;
    }

    // Set the name of the terminal.
    terminal.name = unsafe { xlispstrdup(display_name) };

    dpyinfo_ref
}
