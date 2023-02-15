//! Display generation from window structure and buffer text.

/// Display a null-terminated echo area message M.  If M is 0, clear
/// out any existing message, and let the mini-buffer text show through.

/// The buffer M must continue to exist until after the echo area gets
/// cleared or some other message gets displayed there.  Do not pass
/// text that is stored in a Lisp string.  Do not pass text in a buffer
/// that was alloca'd.

pub fn message1(m: *const ::libc::c_char) {
    unsafe { crate::bindings::message1(m) };
}

pub trait Redisplay {
//     // pub frame_parm_handlers: *mut frame_parm_handler,
//     pub produce_glyphs: ::std::option::Option<unsafe extern "C" fn(it: *mut it)>,
//     pub write_glyphs: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             row: *mut glyph_row,
//             string: *mut glyph,
//             area: glyph_row_area::Type,
//             len: ::libc::c_int,
//         ),
//     >,
//     pub insert_glyphs: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             row: *mut glyph_row,
//             start: *mut glyph,
//             area: glyph_row_area::Type,
//             len: ::libc::c_int,
//         ),
//     >,
//     pub clear_end_of_line: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             row: *mut glyph_row,
//             area: glyph_row_area::Type,
//             x: ::libc::c_int,
//         ),
//     >,
//     pub scroll_run_hook: ::std::option::Option<unsafe extern "C" fn(w: *mut window, run: *mut run)>,
//     pub after_update_window_line_hook:
//         ::std::option::Option<unsafe extern "C" fn(w: *mut window, desired_row: *mut glyph_row)>,
//     pub update_window_begin_hook: ::std::option::Option<unsafe extern "C" fn(w: *mut window)>,
//     pub update_window_end_hook: ::std::option::Option<
//         unsafe extern "C" fn(w: *mut window, cursor_on_p: bool, mouse_face_overwritten_p: bool),
//     >,
//     pub flush_display: ::std::option::Option<unsafe extern "C" fn(f: *mut frame)>,
//     pub clear_window_mouse_face: ::std::option::Option<unsafe extern "C" fn(w: *mut window)>,
//     pub get_glyph_overhangs: ::std::option::Option<
//         unsafe extern "C" fn(
//             glyph: *mut glyph,
//             f: *mut frame,
//             left: *mut ::libc::c_int,
//             right: *mut ::libc::c_int,
//         ),
//     >,
//     pub fix_overlapping_area: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             row: *mut glyph_row,
//             area: glyph_row_area::Type,
//             arg1: ::libc::c_int,
//         ),
//     >,
//     pub draw_fringe_bitmap: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             row: *mut glyph_row,
//             p: *mut draw_fringe_bitmap_params,
//         ),
//     >,
//     pub define_fringe_bitmap: ::std::option::Option<
//         unsafe extern "C" fn(
//             which: ::libc::c_int,
//             bits: *mut ::libc::c_ushort,
//             h: ::libc::c_int,
//             wd: ::libc::c_int,
//         ),
//     >,
//     pub destroy_fringe_bitmap: ::std::option::Option<unsafe extern "C" fn(which: ::libc::c_int)>,
//     pub compute_glyph_string_overhangs:
//         ::std::option::Option<unsafe extern "C" fn(s: *mut glyph_string)>,
//     pub draw_glyph_string: ::std::option::Option<unsafe extern "C" fn(s: *mut glyph_string)>,
//     pub define_frame_cursor:
//         ::std::option::Option<unsafe extern "C" fn(f: *mut frame, cursor: Emacs_Cursor)>,
//     pub clear_frame_area: ::std::option::Option<
//         unsafe extern "C" fn(
//             f: *mut frame,
//             x: ::libc::c_int,
//             y: ::libc::c_int,
//             width: ::libc::c_int,
//             height: ::libc::c_int,
//         ),
//     >,
//     pub clear_under_internal_border: ::std::option::Option<unsafe extern "C" fn(f: *mut frame)>,
//     pub draw_window_cursor: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             glyph_row: *mut glyph_row,
//             x: ::libc::c_int,
//             y: ::libc::c_int,
//             cursor_type: text_cursor_kinds::Type,
//             cursor_width: ::libc::c_int,
//             on_p: bool,
//             active_p: bool,
//         ),
//     >,
//     pub draw_vertical_window_border: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             x: ::libc::c_int,
//             y_0: ::libc::c_int,
//             y_1: ::libc::c_int,
//         ),
//     >,
//     pub draw_window_divider: ::std::option::Option<
//         unsafe extern "C" fn(
//             w: *mut window,
//             x_0: ::libc::c_int,
//             x_1: ::libc::c_int,
//             y_0: ::libc::c_int,
//             y_1: ::libc::c_int,
//         ),
//     >,
//     pub shift_glyphs_for_insert: ::std::option::Option<
//         unsafe extern "C" fn(
//             f: *mut frame,
//             x: ::libc::c_int,
//             y: ::libc::c_int,
//             width: ::libc::c_int,
//             height: ::libc::c_int,
//             shift_by: ::libc::c_int,
//         ),
//     >,
//     pub show_hourglass: ::std::option::Option<unsafe extern "C" fn(f: *mut frame)>,
//     pub hide_hourglass: ::std::option::Option<unsafe extern "C" fn(f: *mut frame)>,
//     pub default_font_parameter:
//         ::std::option::Option<unsafe extern "C" fn(f: *mut frame, parms: Lisp_Object)>,

//     /* Handlers for setting frame parameters.  */
//     frame_parm_handler *frame_parm_handlers;

//   /* Produce glyphs/get display metrics for the display element IT is
//      loaded with.  */
//   void (*produce_glyphs) (struct it *it);

//   /* Write or insert LEN glyphs from STRING at the nominal output
//      position.  */
//   void (*write_glyphs) (struct window *w, struct glyph_row *row,
// 			struct glyph *string, enum glyph_row_area area,
// 			int len);
//   void (*insert_glyphs) (struct window *w, struct glyph_row *row,
// 			 struct glyph *start, enum glyph_row_area area,
// 			 int len);

//   /* Clear from nominal output position to X.  X < 0 means clear
//      to right end of display.  */
//   void (*clear_end_of_line) (struct window *w, struct glyph_row *row,
// 			     enum glyph_row_area area, int x);

//   /* Function to call to scroll the display as described by RUN on
//      window W.  */
//   void (*scroll_run_hook) (struct window *w, struct run *run);

//   /* Function to call after a line in a display has been completely
//      updated.  Used to draw truncation marks and alike.  DESIRED_ROW
//      is the desired row which has been updated.  */
//   void (*after_update_window_line_hook) (struct window *w,
// 					 struct glyph_row *desired_row);

//   /* Function to call before beginning to update window W in
//      window-based redisplay.  */
//   void (*update_window_begin_hook) (struct window *w);

//   /* Function to call after window W has been updated in window-based
//      redisplay.  CURSOR_ON_P true means switch cursor on.
//      MOUSE_FACE_OVERWRITTEN_P true means that some lines in W
//      that contained glyphs in mouse-face were overwritten, so we
//      have to update the mouse highlight.  */
//   void (*update_window_end_hook) (struct window *w, bool cursor_on_p,
//                                   bool mouse_face_overwritten_p);

//   /* Flush the display of frame F.  For X, this is XFlush.  */
//   void (*flush_display) (struct frame *f);

//   /* Clear the mouse highlight in window W, if there is any.  */
//   void (*clear_window_mouse_face) (struct window *w);

//   /* Set *LEFT and *RIGHT to the left and right overhang of GLYPH on
//      frame F.  */
//   void (*get_glyph_overhangs) (struct glyph *glyph, struct frame *f,
//                                int *left, int *right);

//   /* Fix the display of AREA of ROW in window W for overlapping rows.
//      This function is called from redraw_overlapping_rows after
//      desired rows have been made current.  */
//   void (*fix_overlapping_area) (struct window *w, struct glyph_row *row,
//                                 enum glyph_row_area area, int);

// #ifdef HAVE_WINDOW_SYSTEM

//   /* Draw a fringe bitmap in window W of row ROW using parameters P.  */
//   void (*draw_fringe_bitmap) (struct window *w, struct glyph_row *row,
//                               struct draw_fringe_bitmap_params *p);

//   /* Define and destroy fringe bitmap no. WHICH.  */
//   void (*define_fringe_bitmap) (int which, unsigned short *bits,
//                                 int h, int wd);
//   void (*destroy_fringe_bitmap) (int which);

//   /* Compute left and right overhang of glyph string S.
//      A NULL pointer if platform does not support this. */
//   void (*compute_glyph_string_overhangs) (struct glyph_string *s);

//   /* Draw a glyph string S.  */
//   void (*draw_glyph_string) (struct glyph_string *s);

//   /* Define cursor CURSOR on frame F.  */
//   void (*define_frame_cursor) (struct frame *f, Emacs_Cursor cursor);

//   /* Clear the area at (X,Y,WIDTH,HEIGHT) of frame F.  */
//   void (*clear_frame_area) (struct frame *f, int x, int y,
//                             int width, int height);

//  /* Clear area of frame F's internal border.  If the internal border
//     face of F has been specified (is not null), fill the area with
//     that face.  */
//   void (*clear_under_internal_border) (struct frame *f);

//   /* Draw specified cursor CURSOR_TYPE of width CURSOR_WIDTH
//      at row GLYPH_ROW on window W if ON_P is true.  If ON_P is
//      false, don't draw cursor.  If ACTIVE_P is true, system caret
//      should track this cursor (when applicable).  */
//   void (*draw_window_cursor) (struct window *w,
// 			      struct glyph_row *glyph_row,
// 			      int x, int y,
// 			      enum text_cursor_kinds cursor_type,
// 			      int cursor_width, bool on_p, bool active_p);

//   /* Draw vertical border for window W from (X,Y_0) to (X,Y_1).  */
//   void (*draw_vertical_window_border) (struct window *w,
//                                        int x, int y_0, int y_1);

//   /* Draw window divider for window W from (X_0, Y_0) to (X_1, ,Y_1).  */
//   void (*draw_window_divider) (struct window *w,
// 			       int x_0, int x_1, int y_0, int y_1);

//   /* Shift display of frame F to make room for inserted glyphs.
//      The area at pixel (X,Y) of width WIDTH and height HEIGHT is
//      shifted right by SHIFT_BY pixels.  */
//   void (*shift_glyphs_for_insert) (struct frame *f,
//                                    int x, int y, int width,
//                                    int height, int shift_by);

//   /* Start display hourglass cursor on frame F.  */
//   void (*show_hourglass) (struct frame *f);

//   /* Cancel hourglass cursor on frame F.  */
//   void (*hide_hourglass) (struct frame *f);

//   /* Called to (re)calculate the default face when changing the font
//      backend.  */
//   void (*default_font_parameter) (struct frame *f, Lisp_Object parms);

}
