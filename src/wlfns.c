/* Functions for the Wayland Window System.

Copyright (C) 2024 Free Software Foundation, Inc.

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

#include <config.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <unistd.h>

#include "lisp.h"
#include "character.h"
#include "wlterm.h"
#include "frame.h"
#include "window.h"
#include "buffer.h"
#include "dispextern.h"
#include "keyboard.h"
#include "blockinput.h"
#include "charset.h"
#include "coding.h"
#include "termhooks.h"
#include "font.h"



/***********************************************************************
			    Initialization
 ***********************************************************************/

/* Keep this list in the same order as frame_parms in frame.c.
   Use 0 for unsupported frame parameters.  */

frame_parm_handler wl_frame_parm_handlers[] =
{
  gui_set_autoraise,
  gui_set_autolower,
  NULL, /* x_set_background_color, */
  NULL, /* x_set_border_color, */
  gui_set_border_width,
  NULL, /* x_set_cursor_color, */
  NULL, /* x_set_cursor_type, */
  gui_set_font,
  NULL, /* x_set_foreground_color, */
  NULL, /* x_set_icon_name, */
  NULL, /* x_set_icon_type, */
  NULL, /* x_set_child_frame_border_width, */
  NULL, /* x_set_internal_border_width, */
  gui_set_right_divider_width,
  gui_set_bottom_divider_width,
  NULL, /* x_set_menu_bar_lines, */
  NULL, /* x_set_mouse_color, */
  NULL, /* x_explicitly_set_name, */
  gui_set_scroll_bar_width,
  gui_set_scroll_bar_height,
  NULL, /* x_set_title, */
  gui_set_unsplittable,
  gui_set_vertical_scroll_bars,
  gui_set_horizontal_scroll_bars,
  gui_set_visibility,
  NULL, /* x_set_tab_bar_lines, */
  NULL, /* x_set_tool_bar_lines, */
  NULL, /* x_set_scroll_bar_foreground, */
  NULL, /* x_set_scroll_bar_background, */
  gui_set_screen_gamma,
  gui_set_line_spacing,
  gui_set_left_fringe,
  gui_set_right_fringe,
  NULL, /* x_set_wait_for_wm, */
  gui_set_fullscreen,
  gui_set_font_backend,
  NULL, /* x_set_alpha, */
  NULL, /* x_set_sticky, */
  NULL, /* x_set_tool_bar_position, */
#ifdef HAVE_XDBE
  NULL, /* x_set_inhibit_double_buffering, */
#else
  NULL,
#endif
  NULL, /* x_set_undecorated, */
  NULL, /* x_set_parent_frame, */
  NULL, /* x_set_skip_taskbar, */
  NULL, /* x_set_no_focus_on_map, */
  NULL, /* x_set_no_accept_focus, */
  NULL, /* x_set_z_group, */
  NULL, /* x_set_override_redirect, */
  gui_set_no_special_glyphs,
  NULL, /* x_set_alpha_background, */
  NULL, /* x_set_use_frame_synchronization, */
  NULL, /* x_set_shaded, */
};

DEFUN ("x-hide-tip", Fx_hide_tip, Sx_hide_tip, 0, 0, 0,
       doc: /* Hide the current tooltip window, if there is any.
Value is t if tooltip was open, nil otherwise.  */)
  (void)
{
  return Qnil;
}

DEFUN ("xw-display-color-p", Fxw_display_color_p, Sxw_display_color_p, 0, 1, 0,
       doc: /* Internal function called by `display-color-p'.  */)
  (Lisp_Object terminal)
{
  return Qnil;
}

DEFUN ("x-display-grayscale-p", Fx_display_grayscale_p, Sx_display_grayscale_p,
       0, 1, 0,
       doc: /* Return t if the X display supports shades of gray.
Note that color displays do support shades of gray.
The optional argument TERMINAL specifies which display to ask about.
TERMINAL should be a terminal object, a frame or a display name (a string).
If omitted or nil, that stands for the selected frame's display.  */)
  (Lisp_Object terminal)
{
  return Qnil;
}


DEFUN ("wayland-open-connection", Fwayland_open_connection, Swayland_open_connection, 1, 3, 0,
       doc: /* Open a connection to a display server.
DISPLAY is the name of the display to connect to.
Optional second arg XRM-STRING is a string of resources in xrdb format.
If the optional third arg MUST-SUCCEED is non-nil,
terminate Emacs if we can't open the connection.  */)
  (Lisp_Object display, Lisp_Object resource_string, Lisp_Object must_succeed)
{
  struct wl_display_info *dpyinfo;

  dpyinfo = wl_term_init (display);
  if (dpyinfo == 0)
    {
      if (!NILP (must_succeed))
	fatal ("Display on %s not responding.\n", SSDATA (display));
      else
	error ("Display on %s not responding.\n", SSDATA (display));
    }

  return Qnil;
}

/* Handler for signals raised during x_create_frame and
   x_create_tip_frame.  FRAME is the frame which is partially
   constructed.  */

static Lisp_Object
unwind_create_frame (Lisp_Object frame)
{
  struct frame *f = XFRAME (frame);

  /* If frame is already dead, nothing to do.  This can happen if the
     display is disconnected after the frame has become official, but
     before x_create_frame removes the unwind protect.  */
  if (!FRAME_LIVE_P (f))
    return Qnil;

  /* If frame is ``official'', nothing to do.  */
  if (NILP (Fmemq (frame, Vframe_list)))
    {
      /* pgtk_free_frame_resources (f); */
      /* free_glyphs (f); */
      return Qt;
    }

  return Qnil;
}

static void
do_unwind_create_frame (Lisp_Object frame)
{
  unwind_create_frame (frame);
}

/* Return the Wayland display structure for the display named NAME.
   Open a new connection if necessary.  */

static struct wayland_display_info *
wl_display_info_for_name (Lisp_Object name)
{
  struct wl_display_info *dpyinfo;

  CHECK_STRING (name);

  for (dpyinfo = x_display_list; dpyinfo; dpyinfo = dpyinfo->next)
    if (!NILP (Fstring_equal (XCAR (dpyinfo->name_list_element), name)))
      return dpyinfo;

  dpyinfo = wl_term_init (name);

  if (dpyinfo == 0)
    error ("Cannot connect to Wayland server %s", SDATA (name));

  return dpyinfo;
}

/* struct wl_display_info * */
/* check_x_display_info (Lisp_Object frame) */
/* { */
/*   return check_wl_display_info (frame); */
/* } */

/* Let the user specify an Wayland display with a Lisp object.
   OBJECT may be nil, a frame or a terminal object.
   nil stands for the selected frame--or, if that is not an Wayland frame,
   the first Wayland display on the list.  */

struct wl_display_info *
check_wl_display_info (Lisp_Object object)
{
  struct wayland_display_info *dpyinfo = NULL;

  if (NILP (object))
    {
      struct frame *sf = XFRAME (selected_frame);

      if (FRAME_WAYLAND_P (sf) && FRAME_LIVE_P (sf))
	dpyinfo = FRAME_DISPLAY_INFO (sf);
      else if (x_display_list != 0)
	dpyinfo = x_display_list;
      else
	error ("Wayland windows are not in use or not initialized");
    }
  else if (TERMINALP (object))
    {
      struct terminal *t = decode_live_terminal (object);

      if (t->type != output_wayland)
        error ("Terminal %d is not an Wayland display", t->id);

      dpyinfo = t->display_info.wayland;
    }
  else if (STRINGP (object))
    dpyinfo = wl_display_info_for_name (object);
  else
    {
      struct frame *f = decode_window_system_frame (object);
      dpyinfo = FRAME_DISPLAY_INFO (f);
    }

  return dpyinfo;
}

DEFUN ("x-create-frame", Fx_create_frame, Sx_create_frame,
       1, 1, 0,
       doc: /* Make a new X window, which is called a "frame" in Emacs terms.
Return an Emacs frame object.  PARMS is an alist of frame parameters.
If the parameters specify that the frame should not have a minibuffer,
and do not specify a specific minibuffer window to use, then
`default-minibuffer-frame' must be a frame whose minibuffer can be
shared by the new frame.

This function is an internal primitive--use `make-frame' instead.  */)
  (Lisp_Object parms)
{
  struct frame *f;
  Lisp_Object frame, tem;
  Lisp_Object name;
  bool minibuffer_only = false;
  bool undecorated = false, override_redirect = false;
  long window_prompting = 0;
  specpdl_ref count = SPECPDL_INDEX ();
  Lisp_Object display;
  struct wl_display_info *dpyinfo = NULL;
  Lisp_Object parent, parent_frame;
  struct kboard *kb;

  parms = Fcopy_alist (parms);

  /* Use this general default value to start with
     until we know if this frame has a specified name.  */
  Vx_resource_name = Vinvocation_name;  

  display = gui_display_get_arg (dpyinfo, parms, Qterminal, 0, 0,
                                 RES_TYPE_NUMBER);
  if (BASE_EQ (display, Qunbound))
    display = gui_display_get_arg (dpyinfo, parms, Qdisplay, 0, 0,
                                   RES_TYPE_STRING);
  if (BASE_EQ (display, Qunbound))
    display = Qnil;
  dpyinfo = check_wl_display_info (display);
  kb = dpyinfo->terminal->kboard;

  if (!dpyinfo->terminal->name)
    error ("Terminal is not live, can't create new frames on it");

  name = gui_display_get_arg (dpyinfo, parms, Qname, "name", "Name",
                              RES_TYPE_STRING);
  if (!STRINGP (name)
      && ! BASE_EQ (name, Qunbound)
      && ! NILP (name))
    error ("Invalid frame name--not a string or nil");

  if (STRINGP (name))
    Vx_resource_name = name;  

  /* See if parent window is specified.  */
  parent = gui_display_get_arg (dpyinfo, parms, Qparent_id, NULL, NULL,
                                RES_TYPE_NUMBER);
  if (BASE_EQ (parent, Qunbound))
    parent = Qnil;
  if (! NILP (parent))
    CHECK_FIXNUM (parent);

  frame = Qnil;
  tem = gui_display_get_arg (dpyinfo,
                             parms, Qminibuffer, "minibuffer", "Minibuffer",
                             RES_TYPE_SYMBOL);
  if (EQ (tem, Qnone) || NILP (tem))
    f = make_frame_without_minibuffer (Qnil, kb, display);
  else if (EQ (tem, Qonly))
    {
      f = make_minibuffer_frame ();
      minibuffer_only = true;
    }
  else if (WINDOWP (tem))
    f = make_frame_without_minibuffer (tem, kb, display);
  else
    f = make_frame (true);

  parent_frame = gui_display_get_arg (dpyinfo,
                                      parms,
                                      Qparent_frame,
                                      NULL,
                                      NULL,
                                      RES_TYPE_SYMBOL);
  /* Accept parent-frame iff parent-id was not specified.  */
  if (!NILP (parent)
      || BASE_EQ (parent_frame, Qunbound)
      || NILP (parent_frame)
      || !FRAMEP (parent_frame)
      || !FRAME_LIVE_P (XFRAME (parent_frame))
      || !FRAME_X_P (XFRAME (parent_frame)))
    parent_frame = Qnil;

  fset_parent_frame (f, parent_frame);
  store_frame_param (f, Qparent_frame, parent_frame);

  if (!NILP (tem = (gui_display_get_arg (dpyinfo,
                                         parms,
                                         Qundecorated,
                                         NULL,
                                         NULL,
                                         RES_TYPE_BOOLEAN)))
      && !(BASE_EQ (tem, Qunbound)))
    undecorated = true;

  FRAME_UNDECORATED (f) = undecorated;
  store_frame_param (f, Qundecorated, undecorated ? Qt : Qnil);

  if (!NILP (tem = (gui_display_get_arg (dpyinfo,
                                         parms,
                                         Qoverride_redirect,
                                         NULL,
                                         NULL,
                                         RES_TYPE_BOOLEAN)))
      && !(BASE_EQ (tem, Qunbound)))
    override_redirect = true;

  FRAME_OVERRIDE_REDIRECT (f) = override_redirect;
  store_frame_param (f, Qoverride_redirect, override_redirect ? Qt : Qnil);

  XSETFRAME (frame, f);

  f->terminal = dpyinfo->terminal;

  f->output_method = output_wayland;
  FRAME_OUTPUT_DATA (f) = xzalloc (sizeof *FRAME_OUTPUT_DATA (f));
  FRAME_FONTSET (f) = -1;
  /* FRAME_OUTPUT_DATA (f)->white_relief.pixel = -1; */
  /* FRAME_OUTPUT_DATA (f)->black_relief.pixel = -1; */

  fset_icon_name (f, gui_display_get_arg (dpyinfo,
                                          parms,
                                          Qicon_name,
                                          "iconName",
                                          "Title",
                                          RES_TYPE_STRING));
  if (! STRINGP (f->icon_name))
    fset_icon_name (f, Qnil);

  FRAME_DISPLAY_INFO (f) = dpyinfo;

  /* With FRAME_DISPLAY_INFO set up, this unwind-protect is safe.  */
  record_unwind_protect (do_unwind_create_frame, frame);

  /* /\* These colors will be set anyway later, but it's important */
  /*    to get the color reference counts right, so initialize them!  *\/ */
  /* { */
  /*   Lisp_Object black; */

  /*   /\* Function x_decode_color can signal an error.  Make */
  /*      sure to initialize color slots so that we won't try */
  /*      to free colors we haven't allocated.  *\/ */
  /*   FRAME_FOREGROUND_PIXEL (f) = -1; */
  /*   FRAME_BACKGROUND_PIXEL (f) = -1; */
  /*   f->output_data.x->cursor_pixel = -1; */
  /*   f->output_data.x->cursor_foreground_pixel = -1; */
  /*   f->output_data.x->border_pixel = -1; */
  /*   f->output_data.x->mouse_pixel = -1; */

  /*   black = build_string ("black"); */
  /*   FRAME_FOREGROUND_PIXEL (f) */
  /*     = x_decode_color (f, black, BLACK_PIX_DEFAULT (f)); */
  /*   FRAME_BACKGROUND_PIXEL (f) */
  /*     = x_decode_color (f, black, BLACK_PIX_DEFAULT (f)); */
  /*   f->output_data.x->cursor_pixel */
  /*     = x_decode_color (f, black, BLACK_PIX_DEFAULT (f)); */
  /*   f->output_data.x->cursor_foreground_pixel */
  /*     = x_decode_color (f, black, BLACK_PIX_DEFAULT (f)); */
  /*   f->output_data.x->border_pixel */
  /*     = x_decode_color (f, black, BLACK_PIX_DEFAULT (f)); */
  /*   f->output_data.x->mouse_pixel */
  /*     = x_decode_color (f, black, BLACK_PIX_DEFAULT (f)); */
  /* } */

  /* Specify the parent under which to make this X window.  */
  if (!NILP (parent))
    {
      FRAME_OUTPUT_DATA (f)->parent_desc = (Window) XFIXNAT (parent);
      FRAME_OUTPUT_DATA (f)->explicit_parent = true;
    }
  else
    {
      FRAME_OUTPUT_DATA (f)->parent_desc = FRAME_DISPLAY_INFO (f)->root_window;
      FRAME_OUTPUT_DATA (f)->explicit_parent = false;
    }

  /* Set the name; the functions to which we pass f expect the name to
     be set.  */
  if (BASE_EQ (name, Qunbound) || NILP (name))
    {
      /* fset_name (f, build_string (dpyinfo->x_id_name)); */
      f->explicit_name = false;
    }
  else
    {
      fset_name (f, name);
      f->explicit_name = true;
      /* Use the frame's title when getting resources for this frame.  */
      specbind (Qx_resource_name, name);
    }

/* #ifdef USE_CAIRO */
/*   register_font_driver (&ftcrfont_driver, f); */
/* #ifdef HAVE_HARFBUZZ */
/*   register_font_driver (&ftcrhbfont_driver, f); */
/* #endif	/\* HAVE_HARFBUZZ *\/ */
/* #else */
/* #ifdef HAVE_FREETYPE */
/* #ifdef HAVE_XFT */
/*   register_font_driver (&xftfont_driver, f); */
/* #ifdef HAVE_HARFBUZZ */
/*   register_font_driver (&xfthbfont_driver, f); */
/* #endif */
/* #endif	/\* not HAVE_XFT *\/ */
/* #endif	/\* HAVE_FREETYPE *\/ */
/* #endif	/\* not USE_CAIRO *\/ */
/*   register_font_driver (&xfont_driver, f); */

/* #ifdef GLYPH_DEBUG */
/*   dpyinfo_refcount = dpyinfo->reference_count; */
/* #endif /\* GLYPH_DEBUG *\/ */

  register_swash_font_driver(f);

  gui_default_parameter (f, parms, Qfont_backend, Qnil,
                         "fontBackend", "FontBackend", RES_TYPE_STRING);
  /* // We rely on Rust font-index crate to choose a generic Monospace font */
  /* gui_default_parameter (f, parms, Qfont, Qnil, */
  /*                        "font", "Monospace", RES_TYPE_STRING); */

  fprintf(stderr, "wtf");
  FRAME_RIF (f)->default_font_parameter (f, parms);

  /* /\* Extract the window parameters from the supplied values */
  /*    that are needed to determine window geometry.  *\/ */
  /* wl_default_font_parameter (f, parms); */

  /* FIXME font is invalid here */
  if (!FRAME_FONT (f))
    {
      int height = FONT_HEIGHT (FRAME_FONT (f));
      delete_frame (frame, Qnoelisp);
      error ("Invalid frame font");
    }

  gui_default_parameter (f, parms, Qborder_width, make_fixnum (0),
                         "borderWidth", "BorderWidth", RES_TYPE_NUMBER);

  /* This defaults to 1 in order to match xterm.  We recognize either
     internalBorderWidth or internalBorder (which is what xterm calls
     it).  */
  if (NILP (Fassq (Qinternal_border_width, parms)))
    {
      Lisp_Object value;

      value = gui_display_get_arg (dpyinfo, parms, Qinternal_border_width,
                                   "internalBorder", "internalBorder",
                                   RES_TYPE_NUMBER);
      if (! BASE_EQ (value, Qunbound))
	parms = Fcons (Fcons (Qinternal_border_width, value),
		       parms);
    }

  gui_default_parameter (f, parms, Qinternal_border_width, make_fixnum (1),
                         "internalBorderWidth", "internalBorderWidth",
                         RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qright_divider_width, make_fixnum (0),
                         NULL, NULL, RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qbottom_divider_width, make_fixnum (0),
                         NULL, NULL, RES_TYPE_NUMBER);

  /* Also do the stuff which must be set before the window exists.  */
  gui_default_parameter (f, parms, Qforeground_color, build_string ("black"),
                         "foreground", "Foreground", RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qbackground_color, build_string ("white"),
                         "background", "Background", RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qmouse_color, build_string ("black"),
                         "pointerColor", "Foreground", RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qcursor_color, build_string ("black"),
                         "cursorColor", "Foreground", RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qborder_color, build_string ("black"),
                         "borderColor", "BorderColor", RES_TYPE_STRING);
  gui_default_parameter (f, parms, Qno_special_glyphs, Qnil,
                         NULL, NULL, RES_TYPE_BOOLEAN);

  /* Init faces before gui_default_parameter is called for the
     scroll-bar-width parameter because otherwise we end up in
     init_iterator with a null face cache, which should not happen.  */
  init_frame_faces (f);

  gui_default_parameter (f, parms, Qinhibit_double_buffering, Qnil,
                         "inhibitDoubleBuffering", "InhibitDoubleBuffering",
                         RES_TYPE_BOOLEAN);

  gui_figure_window_size (f, parms, false, false);

  f->output_data.wayland->parent_desc = FRAME_DISPLAY_INFO (f)->root_window;

  /* x_make_gc (f); */

  gui_default_parameter (f, parms, Qauto_raise, Qnil,
                         "autoRaise", "AutoRaiseLower", RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qauto_lower, Qnil,
                         "autoLower", "AutoRaiseLower", RES_TYPE_BOOLEAN);
  gui_default_parameter (f, parms, Qcursor_type, Qbox,
                         "cursorType", "CursorType", RES_TYPE_SYMBOL);
  gui_default_parameter (f, parms, Qalpha, Qnil,
                         "alpha", "Alpha", RES_TYPE_NUMBER);
  gui_default_parameter (f, parms, Qalpha_background, Qnil,
                         "alphaBackground", "AlphaBackground", RES_TYPE_NUMBER);

  /* Add `tooltip' frame parameter's default value. */
  if (NILP (Fframe_parameter (frame, Qtooltip)))
    {
      AUTO_FRAME_ARG (arg, Qtooltip, Qt);
      Fmodify_frame_parameters (frame, arg);
    }

  /* /\* FIXME - can this be done in a similar way to normal frames? */
  /*    https://lists.gnu.org/r/emacs-devel/2007-10/msg00641.html *\/ */

  /* /\* Set the `display-type' frame parameter before setting up faces. *\/ */
  /* { */
  /*   Lisp_Object disptype; */

  /*   if (FRAME_DISPLAY_INFO (f)->n_planes == 1) */
  /*     disptype = Qmono; */
  /*   else if (FRAME_X_VISUAL_INFO (f)->class == GrayScale */
  /*            || FRAME_X_VISUAL_INFO (f)->class == StaticGray) */
  /*     disptype = Qgrayscale; */
  /*   else */
  /*     disptype = Qcolor; */

  /*   if (NILP (Fframe_parameter (frame, Qdisplay_type))) */
  /*     { */
  /* 	AUTO_FRAME_ARG (arg, Qdisplay_type, disptype); */
  /* 	Fmodify_frame_parameters (frame, arg); */
  /*     } */
  /* } */

  /* Set up faces after all frame parameters are known.  This call
     also merges in face attributes specified for new frames.

     Frame parameters may be changed if .Xdefaults contains
     specifications for the default font.  For example, if there is an
     `Emacs.default.attributeBackground: pink', the `background-color'
     attribute of the frame gets set, which let's the internal border
     of the tooltip frame appear in pink.  Prevent this.  */
  {
    Lisp_Object bg = Fframe_parameter (frame, Qbackground_color);

    call2 (Qface_set_after_frame_default, frame, Qnil);

    if (!EQ (bg, Fframe_parameter (frame, Qbackground_color)))
      {
	AUTO_FRAME_ARG (arg, Qbackground_color, bg);
	Fmodify_frame_parameters (frame, arg);
      }
  }

  f->no_split = true;

  /* Now that the frame will be official, it counts as a reference to
     its display and terminal.  */
  FRAME_DISPLAY_INFO (f)->reference_count++;
  f->terminal->reference_count++;

  /* It is now ok to make the frame official even if we get an error
     below.  And the frame needs to be on Vframe_list or making it
     visible won't work.  */
  Vframe_list = Fcons (frame, Vframe_list);
  f->can_set_window_size = true;
  adjust_frame_size (f, FRAME_TEXT_WIDTH (f), FRAME_TEXT_HEIGHT (f),
		     0, true, Qtip_frame);

  /* /\* Setting attributes of faces of the tooltip frame from resources */
  /*    and similar will set face_change, which leads to the clearing of */
  /*    all current matrices.  Since this isn't necessary here, avoid it */
  /*    by resetting face_change to the value it had before we created */
  /*    the tip frame.  *\/ */
  /* face_change = face_change_before; */

  /* Discard the unwind_protect.  */
  return unbind_to (count, frame);
}

/* Called from frame.c.  */
struct wl_display_info *
check_x_display_info (Lisp_Object frame)
{
  return check_wl_display_info (frame);
}

void
syms_of_wlfns (void)
{
  Fprovide (intern_c_string ("wayland"), Qnil);

  /* defsubr (&Sx_create_frame); */
  defsubr (&Swayland_open_connection);
  /* defsubr (&Sx_close_connection); */
  defsubr (&Sx_hide_tip);
  defsubr (&Sx_display_grayscale_p);
  defsubr (&Sxw_display_color_p);
  defsubr (&Sx_create_frame);
}
