use super::frame::FrameExtWinit;
use crate::input::InputProcessor;
use crate::{winit_set_background_color, winit_set_cursor_color};
use emacs::bindings::{
    add_keyboard_wait_descriptor, block_input, gl_renderer_free_frame_resources,
    gl_renderer_free_terminal_resources, init_sigio, interrupt_input, unblock_input,
    wr_after_update_window_line, wr_clear_frame, wr_clear_frame_area, wr_defined_color,
    wr_draw_fringe_bitmap, wr_draw_glyph_string, wr_draw_vertical_window_border,
    wr_draw_window_cursor, wr_draw_window_divider, wr_flush_display, wr_free_pixmap, wr_new_font,
    wr_scroll_run, wr_update_end, wr_update_window_begin, wr_update_window_end,
};
use emacs::terminal::TerminalRef;
use raw_window_handle::HasRawDisplayHandle;
use std::ptr;
use std::sync::OnceLock;
use std::time::Duration;
use winit::event::{ElementState, Event, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use winit::platform::pump_events::{EventLoopExtPumpEvents, PumpStatus};

use webrender_api::{self, units::*};

use crate::event::create_emacs_event;
use emacs::display_info::{DisplayInfo, DisplayInfoRef};

use emacs::{
    bindings::{
        create_terminal, current_kboard, frame_parm_handler, initial_kboard, note_mouse_highlight,
        output_method, redisplay_interface, scroll_bar_part, terminal, xlispstrdup, Emacs_Cursor,
        Fcons,
    },
    bindings::{
        gui_clear_end_of_line, gui_clear_window_mouse_face, gui_fix_overlapping_area,
        gui_get_glyph_overhangs, gui_insert_glyphs, gui_produce_glyphs, gui_set_alpha,
        gui_set_autolower, gui_set_autoraise, gui_set_border_width, gui_set_bottom_divider_width,
        gui_set_font, gui_set_font_backend, gui_set_fullscreen, gui_set_horizontal_scroll_bars,
        gui_set_left_fringe, gui_set_line_spacing, gui_set_no_special_glyphs,
        gui_set_right_divider_width, gui_set_right_fringe, gui_set_screen_gamma,
        gui_set_scroll_bar_height, gui_set_scroll_bar_width, gui_set_unsplittable,
        gui_set_vertical_scroll_bars, gui_set_visibility, gui_write_glyphs, input_event,
        kbd_buffer_store_event_hold, Time, PT_PER_INCH,
    },
    frame::{all_frames, Frame, FrameRef},
    globals::{Qnil, Qparent_frame, Qwinit},
    keyboard::allocate_keyboard,
    lisp::LispObject,
};

fn get_frame_parm_handlers() -> [frame_parm_handler; 51] {
    // Keep this list in the same order as frame_parms in frame.c.
    // Use None for unsupported frame parameters.
    let handlers: [frame_parm_handler; 51] = [
        Some(gui_set_autoraise),
        Some(gui_set_autolower),
        Some(winit_set_background_color),
        None, // x_set_border_color
        Some(gui_set_border_width),
        Some(winit_set_cursor_color),
        None, // x_set_cursor_type
        Some(gui_set_font),
        None, // x_set_foreground_color
        None, // x_set_icon_name
        None, // x_set_icon_type
        None, // x_set_child_frame_border_width
        None, // x_set_internal_border_width
        Some(gui_set_right_divider_width),
        Some(gui_set_bottom_divider_width),
        Some(winit_set_menu_bar_lines),
        None, // x_set_mouse_color
        None, // x_explicitly_set_name
        Some(gui_set_scroll_bar_width),
        Some(gui_set_scroll_bar_height),
        None, // x_set_title
        Some(gui_set_unsplittable),
        Some(gui_set_vertical_scroll_bars),
        Some(gui_set_horizontal_scroll_bars),
        Some(gui_set_visibility),
        None, // x_set_tab_bar_lines
        None, // x_set_tool_bar_lines
        None, // x_set_scroll_bar_foreground
        None, // x_set_scroll_bar_background
        Some(gui_set_screen_gamma),
        Some(gui_set_line_spacing),
        Some(gui_set_left_fringe),
        Some(gui_set_right_fringe),
        None, // x_set_wait_for_wm
        Some(gui_set_fullscreen),
        Some(gui_set_font_backend),
        Some(gui_set_alpha),
        None, // x_set_sticky
        None, // x_set_tool_bar_position
        None, // x_set_inhibit_double_buffering,
        None, // x_set_undecorated
        Some(winit_set_parent_frame),
        None, // x_set_skip_taskbar
        None, // x_set_no_focus_on_map
        None, // x_set_no_accept_focus
        None, // x_set_z_group
        None, // x_set_override_redirect,
        Some(gui_set_no_special_glyphs),
        None, // x_set_alpha_background,
        None, // x_set_use_frame_synchronization,
        None, // x_set_shaded,
    ];

    handlers
}

struct RedisplayInterface(pub redisplay_interface);
unsafe impl Sync for RedisplayInterface {}
unsafe impl Send for RedisplayInterface {}

static REDISPLAY_INTERFACE: OnceLock<RedisplayInterface> = OnceLock::new();
impl RedisplayInterface {
    fn global() -> &'static RedisplayInterface {
        REDISPLAY_INTERFACE.get_or_init(|| {
            log::trace!("REDISPLAY_INTERFACE is being created...");
            let frame_parm_handlers = Box::new(get_frame_parm_handlers());

            let interface = redisplay_interface {
                frame_parm_handlers: (Box::into_raw(frame_parm_handlers)) as *mut Option<_>,
                produce_glyphs: Some(gui_produce_glyphs),
                write_glyphs: Some(gui_write_glyphs),
                insert_glyphs: Some(gui_insert_glyphs),
                clear_end_of_line: Some(gui_clear_end_of_line),
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
                clear_frame_area: Some(wr_clear_frame_area),
                clear_under_internal_border: None,
                draw_window_cursor: Some(wr_draw_window_cursor),
                draw_vertical_window_border: Some(wr_draw_vertical_window_border),
                draw_window_divider: Some(wr_draw_window_divider),
                shift_glyphs_for_insert: None, /* Never called; see comment in xterm.c.  */
                show_hourglass: None,
                hide_hourglass: None,
                default_font_parameter: None,
            };

            RedisplayInterface(interface)
        })
    }
}

extern "C" fn get_string_resource(
    _rdb: *mut libc::c_void,
    _name: *const libc::c_char,
    _class: *const libc::c_char,
) -> *const libc::c_char {
    ptr::null()
}

extern "C" fn winit_frame_visible_invisible(frame: *mut Frame, is_visible: bool) {
    let mut f: FrameRef = frame.into();

    f.set_visible_(is_visible);
}

extern "C" fn winit_define_frame_cursor(f: *mut Frame, cursor: Emacs_Cursor) {
    let frame: FrameRef = f.into();
    frame.set_cursor_icon(cursor);
}

extern "C" fn winit_read_input_event(terminal: *mut terminal, hold_quit: *mut input_event) -> i32 {
    let mut terminal: TerminalRef = terminal.into();
    let mut display_info = terminal.display_info();

    let data = terminal.clone().winit_term_data();

    let mut count = 0;
    let mut handle_event = |e: Event<i32>| {
        match e {
            Event::WindowEvent {
                window_id, event, ..
            } => {
                let frame = all_frames().find(|f| {
                    f.output().winit_term_data().window.as_ref().unwrap().id() == window_id
                });

                if frame.is_none() {
                    return;
                }

                let mut frame: FrameRef = frame.unwrap();
                //lisp frame
                let lframe: LispObject = frame.into();

                match event {
                    WindowEvent::RedrawRequested => {}
                    WindowEvent::ModifiersChanged(modifiers) => {
                        let _ = InputProcessor::handle_modifiers_changed(modifiers.state());
                    }

                    WindowEvent::KeyboardInput { event, .. } => match event.state {
                        ElementState::Pressed => match event.logical_key {
                            Key::Character(_) | Key::Named(NamedKey::Space) => {
                                for (_i, key_code) in
                                    event.logical_key.to_text().unwrap().chars().enumerate()
                                {
                                    if let Some(mut iev) =
                                        InputProcessor::handle_receive_char(key_code, lframe)
                                    {
                                        unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                                        count += 1;
                                    }
                                }
                            }
                            _ => {
                                if let Some(mut iev) =
                                    InputProcessor::handle_key_pressed(event.physical_key, lframe)
                                {
                                    unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                                    count += 1;
                                }
                            }
                        },
                        ElementState::Released => {
                            InputProcessor::handle_key_released();
                        }
                    },

                    WindowEvent::MouseInput { state, button, .. } => {
                        if let Some(mut iev) =
                            InputProcessor::handle_mouse_pressed(button, state, lframe)
                        {
                            unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                            count += 1;
                        }
                    }

                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        if let Some(mut iev) =
                            InputProcessor::handle_mouse_wheel_scrolled(delta, phase, lframe)
                        {
                            unsafe { kbd_buffer_store_event_hold(&mut iev, hold_quit) };
                            count += 1;
                        }

                        frame.set_mouse_moved(false);
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        unsafe {
                            note_mouse_highlight(
                                frame.as_mut(),
                                position.x as i32,
                                position.y as i32,
                            )
                        };

                        frame.set_cursor_position(position);

                        frame.set_mouse_moved(true);
                    }

                    WindowEvent::Focused(is_focused) => {
                        let mut top_frame = lframe.as_frame().unwrap();

                        let focus_frame = if !top_frame.focus_frame.eq(Qnil) {
                            top_frame.focus_frame.as_frame().unwrap().as_mut()
                        } else {
                            top_frame.as_mut()
                        };
                        display_info.highlight_frame = if is_focused {
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
                        let scale_factor = frame.winit_scale_factor();
                        let size = DeviceIntSize::new(
                            (size.width as f64 / scale_factor).round() as i32,
                            (size.height as f64 / scale_factor).round() as i32,
                        );
                        frame.handle_size_change(size, scale_factor);
                    }

                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        inner_size_writer: _,
                    } => {
                        frame.handle_scale_factor_change(scale_factor);
                    }

                    WindowEvent::CloseRequested => {
                        let mut event = create_emacs_event(
                            emacs::bindings::event_kind::DELETE_WINDOW_EVENT,
                            lframe,
                        );

                        unsafe { kbd_buffer_store_event_hold(&mut event, hold_quit) };
                        count += 1;
                    }

                    _ => {}
                }
            }
            _ => {}
        }
    };

    let status =
        terminal
            .winit_term_data()
            .event_loop
            .pump_events(Some(Duration::ZERO), |e, _elwt| {
                if let Event::WindowEvent { event, .. } = &e {
                    // Print only Window events to reduce noise
                    log::trace!("{event:?}");
                }

                match e {
                    Event::AboutToWait => {
                        all_frames().for_each(|f| {
                            let window = &f.output().winit_term_data().window;
                            match window {
                                Some(w) => w.request_redraw(),
                                None => {}
                            }
                        });
                        spin_sleep::sleep(Duration::from_millis(8));
                    }
                    Event::WindowEvent {
                        event, window_id, ..
                    } => match event {
                        WindowEvent::Resized(_)
                        | WindowEvent::KeyboardInput { .. }
                        | WindowEvent::ModifiersChanged(_)
                        | WindowEvent::MouseInput { .. }
                        | WindowEvent::CursorMoved { .. }
                        | WindowEvent::ThemeChanged(_)
                        | WindowEvent::Focused(_)
                        | WindowEvent::MouseWheel { .. }
                        | WindowEvent::RedrawRequested
                        | WindowEvent::CloseRequested => {
                            handle_event(Event::WindowEvent { window_id, event });
                        }
                        _ => {}
                    },
                    Event::UserEvent(_nfds) => {}
                    _ => {}
                }
            });
    if let PumpStatus::Exit(_exit_code) = status {
        // break 'main ExitCode::from(exit_code as u8);
    }

    count
}

extern "C" fn winit_fullscreen(f: *mut Frame) {
    let frame: FrameRef = f.into();
    frame.fullscreen();
}

extern "C" fn winit_menu_show(
    _f: *mut Frame,
    _x: ::libc::c_int,
    _y: ::libc::c_int,
    _menuflags: ::libc::c_int,
    _title: LispObject,
    _error_name: *mut *const ::libc::c_char,
) -> LispObject {
    message!("Menu functionalities currently is not available for Winit");
    Qnil
}

// This function should be called by Emacs redisplay code to set the
// name; names set this way will never override names set by the user's
// lisp code.
extern "C" fn winit_implicitly_set_name(frame: *mut Frame, arg: LispObject, old_val: LispObject) {
    let mut frame: FrameRef = frame.into();

    frame.implicitly_set_name(arg, old_val);
}

extern "C" fn winit_get_focus_frame(frame: *mut Frame) -> LispObject {
    let frame: FrameRef = frame.into();
    let mut terminal = frame.terminal();

    let focus_frame = terminal.winit_term_data().focus_frame;

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
extern "C" fn winit_make_frame_visible_invisible(f: *mut Frame, visible: bool) {
    let mut frame: FrameRef = f.into();

    frame.set_visible_(visible);
}

extern "C" fn winit_iconify_frame(f: *mut Frame) {
    let mut frame: FrameRef = f.into();
    frame.iconify();
}

extern "C" fn winit_mouse_position(
    fp: *mut *mut Frame,
    _insist: i32,
    bar_window: *mut LispObject,
    part: *mut scroll_bar_part::Type,
    x: *mut LispObject,
    y: *mut LispObject,
    _timestamp: *mut Time,
) {
    let (dpyinfo, cursor_pos) = {
        let frame: FrameRef = unsafe { (*fp).into() };

        (frame.display_info(), frame.cursor_position())
    };

    // Clear the mouse-moved flag for every frame on this display.
    for mut frame in all_frames() {
        if frame.display_info() == dpyinfo {
            frame.set_mouse_moved(false);
        }
    }

    unsafe { *bar_window = Qnil };
    unsafe { *part = 0 };

    unsafe { *x = cursor_pos.x.into() };
    unsafe { *y = cursor_pos.y.into() };
}

// cleanup frame resource after frame is deleted
extern "C" fn winit_destroy_frame(f: *mut Frame) {
    unsafe { gl_renderer_free_frame_resources(f) };
    let frame: FrameRef = f.into();
    frame.output().free_winit_term_data();
}

extern "C" fn winit_set_menu_bar_lines(f: *mut Frame, value: LispObject, _old_value: LispObject) {
    let frame = FrameRef::from(f);
    /* Right now, menu bars don't work properly in minibuf-only frames;
    most of the commands try to apply themselves to the minibuffer
    frame itself, and get an error because you can't switch buffers
    in or split the minibuffer window.  */
    if frame.is_minibuf_only() || frame.parent_frame().is_some() {
        return;
    }

    //TODO unimplemented set_menu_bar_lines
    return;
}

extern "C" fn winit_set_parent_frame(f: *mut Frame, value: LispObject, old_value: LispObject) {
    if value.is_not_nil()
        && (!value.is_frame()
            || !FrameRef::from(value).is_live()
            || !FrameRef::from(value).is_current_window_system())
    {
        FrameRef::from(f).store_param(Qparent_frame, old_value);
        error!("Invalid specification of `parent-frame'");
    }
    println!("new child frame {value:?}, old {old_value:?}");

    let p = FrameRef::from(value);
    let f = FrameRef::from(f);
    let parent_frame = f.parent_frame();

    if parent_frame.is_some() && parent_frame.unwrap() != p {
        unsafe { block_input() };
        if !p.is_null() {
            if f.display_info() != p.display_info() {
                error!("Cross display reparent.");
            }
        }

        if p.is_null() {
            //
        } else {
        }

        unsafe { unblock_input() };
        f.set_parent(value);
    }
}

#[no_mangle]
pub extern "C" fn set_frame_menubar(_f: *mut Frame, _deep_p: bool) {
    todo!()
}

fn winit_create_terminal(mut dpyinfo: DisplayInfoRef) -> TerminalRef {
    let redisplay_interface = RedisplayInterface::global();
    let terminal_ptr = unsafe {
        create_terminal(
            output_method::output_winit,
            &redisplay_interface.0 as *const _ as *mut _,
        )
    };

    let mut terminal = TerminalRef::new(terminal_ptr);

    // Link terminal and dpyinfo together
    terminal.display_info.winit = dpyinfo.as_mut();
    dpyinfo.terminal = terminal.as_mut();

    // Terminal hooks
    // Other hooks are NULL by default.
    terminal.get_string_resource_hook = Some(get_string_resource);
    terminal.set_new_font_hook = Some(wr_new_font);
    terminal.defined_color_hook = Some(wr_defined_color);
    terminal.frame_visible_invisible_hook = Some(winit_frame_visible_invisible);
    terminal.clear_frame_hook = Some(wr_clear_frame);
    terminal.read_socket_hook = Some(winit_read_input_event);
    terminal.fullscreen_hook = Some(winit_fullscreen);
    terminal.menu_show_hook = Some(winit_menu_show);
    terminal.implicit_set_name_hook = Some(winit_implicitly_set_name);
    terminal.get_focus_frame = Some(winit_get_focus_frame);
    terminal.frame_visible_invisible_hook = Some(winit_make_frame_visible_invisible);
    terminal.iconify_frame_hook = Some(winit_iconify_frame);
    terminal.mouse_position_hook = Some(winit_mouse_position);
    terminal.update_end_hook = Some(wr_update_end);
    terminal.free_pixmap = Some(wr_free_pixmap);
    terminal.delete_frame_hook = Some(winit_destroy_frame);
    terminal.delete_terminal_hook = Some(winit_delete_terminal);

    // Init term data for winit
    let _ = terminal.winit_term_data();

    terminal
}

extern "C" fn winit_delete_terminal(terminal: *mut terminal) {
    unsafe { gl_renderer_free_terminal_resources(terminal) };
    let mut terminal: TerminalRef = terminal.into();
    terminal.free_winit_term_data();
}

pub fn winit_term_init(display_name: LispObject) -> DisplayInfoRef {
    log::info!("Winit term init");

    let dpyinfo = Box::new(DisplayInfo::default());
    let mut dpyinfo_ref = DisplayInfoRef::new(Box::into_raw(dpyinfo));
    let mut terminal = winit_create_terminal(dpyinfo_ref);

    fn register_io_fd(fd: std::os::fd::RawFd) {
        unsafe {
            add_keyboard_wait_descriptor(fd);
            libc::fcntl(fd, libc::F_SETOWN, libc::getpid());
        }
    }

    //TODO add support for macOS/windows for interrupt_input
    let fd = emacs::display_descriptor(terminal.raw_display_handle());

    #[cfg(free_unix)]
    {
        use std::os::fd::AsRawFd;
        register_io_fd(terminal.winit_term_data().event_loop.as_raw_fd());
    }
    #[cfg(not(free_unix))]
    register_io_fd(fd);

    unsafe {
        if interrupt_input {
            init_sigio(fd);
        }
    };

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
