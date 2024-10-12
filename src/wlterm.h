/* Definitions and headers for communication with Wayland client protocol.
   Copyright (C) 2024 Free Software Foundation,
   Inc.

This file is part of GNU Emacs.

GNU Emacs is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or (at
your option) any later version.

GNU Emacs is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with GNU Emacs.  If not, see <https://www.gnu.org/licenses/>.  */

#ifndef WLTERM_H
#define WLTERM_H

#include "dispextern.h"
#include "termhooks.h"


INLINE_HEADER_BEGIN

typedef int *Emacs_Cursor;
typedef int Window;
typedef int Display; /* HDC so it doesn't conflict with xpm lib.  */


/* For each Wayland display, we have a structure that records
   information about it.  */

struct wl_display_info
{
  /* Gl Renderer global data in Rust */
  void *gl_renderer_data;
  
  /* Chain of all x_display_info structures.  */
  struct wl_display_info *next;

  /* The generic display parameters corresponding to this X display. */
  struct terminal *terminal;

  /* Represents the client(Emacs) connection to a Wayland compositor.  */
  struct wl_display *display;

  struct wl_compositor *compositor;

  /* This is a cons cell of the form (NAME . FONT-LIST-CACHE).  */
  Lisp_Object name_list_element;

  /* Number of frames that are on this display.  */
  int reference_count;

  /* Minimum width over all characters in all fonts in font_table.  */
  int smallest_char_width;

  /* Minimum font height over all fonts in font_table.  */
  int smallest_font_height;

  /* Information about the range of text currently shown in
     mouse-face.  */
  Mouse_HLInfo mouse_highlight;

  /* The number of fonts opened for this display.  */
  int n_fonts;

  /* Pointer to bitmap records.  */
  struct wr_bitmap_record *bitmaps;

  /* Allocated size of bitmaps field.  */
  ptrdiff_t bitmaps_size;

  /* Last used bitmap index.  */
  ptrdiff_t bitmaps_last;

  /* Dots per inch of the screen.  */
  double resx, resy;

  /* Number of planes on this screen.  */
  int n_planes;

  /* Mask of things that cause the mouse to be grabbed.  */
  int grabbed;

  /* The root window of this screen.  */
  Window root_window;

  /* The frame which currently has the visual highlight, and should get
     keyboard input (other sorts of input have the frame encoded in the
     event).  It points to the X focus frame's selected window's
     frame.  It differs from x_focus_frame when we're using a global
     minibuffer.  */
  struct frame *highlight_frame;

  /* The frame where the mouse was last time we reported a ButtonPress event.  */
  struct frame *last_mouse_frame;

  /* The frame where the mouse was last time we reported a mouse motion.  */
  struct frame *last_mouse_motion_frame;

  /* Position where the mouse was last time we reported a motion.
     This is a position on last_mouse_motion_frame.  It is used in
     some situations to report the mouse position as well: see
     XTmouse_position.  */
  int last_mouse_motion_x;
  int last_mouse_motion_y;
};

extern struct wl_display_info *wl_term_init (Lisp_Object);

/* This is a chain of structures for all the PGTK displays currently in use.  */
extern struct wl_display_info *x_display_list;

/* Each Wayland frame object points to its own struct wl_output object
   in the output_data.wl field.  The wl_output structure contains the
   information that is specific to Wayland windows.  */

struct wl_output
{
  /* Inner perporty in Rust */
  void *gl_renderer;
  
  /* Default ASCII font of this frame.  */
  struct font *font;

  /* The baseline offset of the default ASCII font.  */
  int baseline_offset;

  /* If a fontset is specified for this frame instead of font, this
     value contains an ID of the fontset, else -1.  */
  int fontset;

  /* This is the Emacs structure for the X display this frame is on.  */
  struct wl_display_info *display_info;

  /* True means our parent is another application's window
     and was explicitly specified.  */
  bool_bf explicit_parent : 1;

  /* The Wayland window used for this frame.
     May be zero while the frame object is being created
     and the X window has not yet been created.  */
  Window window_desc;

  /* The Wayland window that is the parent of this Wayland window.
     Usually this is a window that was made by the window manager,
     but it can be the root window, and it can be explicitly specified
     (see the explicit_parent field, below).  */
  Window parent_desc;

  /* Descriptor for the cursor in use for this window.  */
  Emacs_Cursor current_cursor;
  Emacs_Cursor text_cursor;
  Emacs_Cursor nontext_cursor;
  Emacs_Cursor modeline_cursor;
  Emacs_Cursor hand_cursor;
  Emacs_Cursor hourglass_cursor;
  Emacs_Cursor horizontal_drag_cursor;
  Emacs_Cursor vertical_drag_cursor;
  Emacs_Cursor left_edge_cursor;
  Emacs_Cursor top_left_corner_cursor;
  Emacs_Cursor top_edge_cursor;
  Emacs_Cursor top_right_corner_cursor;
  Emacs_Cursor right_edge_cursor;
  Emacs_Cursor bottom_right_corner_cursor;
  Emacs_Cursor bottom_edge_cursor;
  Emacs_Cursor bottom_left_corner_cursor;

};

/* Defined in wlterm.c */
extern void wl_delete_terminal (struct terminal *);

/* Return the X output data for frame F.  */
#define FRAME_WAYLAND_OUTPUT(f) ((f)->output_data.wayland)
#define FRAME_OUTPUT_DATA(f) FRAME_WAYLAND_OUTPUT (f)
#define FRAME_X_OUTPUT(f) FRAME_WAYLAND_OUTPUT (f)

/* Return the X window used for displaying data in frame F.  */
#define FRAME_WAYLAND_WINDOW(f) ((f)->output_data.wayland->window_desc)
#define FRAME_NATIVE_WINDOW(f) FRAME_WAYLAND_WINDOW (f)
#define FRAME_BASELINE_OFFSET(f) ((f)->output_data.wayland->baseline_offset)


#define FRAME_FONT(f) ((f)->output_data.wayland->font)
#define FRAME_FONTSET(f) ((f)->output_data.wayland->fontset)

/* This gives the wl_display_info structure for the display F is on.  */
#define FRAME_DISPLAY_INFO(f) ((f)->output_data.wayland->display_info)

#include "webrender_ffi.h"

INLINE_HEADER_END

#endif /* XTERM_H */
