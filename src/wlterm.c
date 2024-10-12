/* Wayland Communication module for terminals which understand
   the Wayland protocol.

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

/* Wayland window system support for GNU Emacs

   This file is part of the Wayland window system support for GNU Emacs.
   It contains subroutines comprising the redisplay interface, setting
   up scroll bars and widgets, and handling input.

   WAYLAND WINDOW SYSTEM

   Wayland is the next-generation display server for Unix-like systems,
   designed and built by the alumni of the venerable Xorg server, and is
   the best way to get your application windows onto your user's
   screens. Readers who have worked with X11 in the past will be
   pleasantly surprised by Wayland's improvements, and those who are new
   to graphics on Unix will find it a flexible and powerful system for
   building graphical applications and desktops.

   INPUT

   Emacs handles input by running pselect in a loop, which returns
   whenever there is input available on the connection to the X
   server.  On some systems, Emacs also arranges for any new input on
   that connection to send an asynchronous signal.  Whenever pselect
   returns, or such a signal is received and input is not blocked,
   XTread_socket is called and translates X11 events read by Xlib into
   struct input_events, which are then stored in the keyboard buffer,
   to be processed and acted upon at some later time.  The function
   handle_one_xevent is responsible for handling core events after
   they are filtered, and filtering X Input Extension events.  It also
   performs actions on some special events, such as updating the
   dimensions of a frame after a ConfigureNotify is sent by the X
   server to inform us that it changed.

   Before such events are translated, an Emacs build with
   internationalization enabled (the default since X11R6) will filter
   events through an X Input Method (XIM) or GTK, which might decide
   to intercept the event and send a different one in its place, for
   reasons such as enabling the user to insert international
   characters that aren't on his keyboard by typing a sequence of
   characters which are.  See the function x_filter_event and its
   callers for more details.

   Events that cause Emacs to quit are treated specially by the code
   that stores them in the keyboard buffer and generally cause an
   immediate interrupt.  Such an interrupt can lead to a longjmp from
   the code that stored the keyboard event, which isn't safe inside
   XTread_socket.  To avoid this problem, XTread_socket is provided a
   special event buffer named hold_quit.  When a quit event is
   encountered, it is stored inside this special buffer, which will
   cause the keyboard code that called XTread_socket to store it at a
   later time when it is safe to do so.

   handle_one_xevent will generally have to determine which frame an
   event should be attributed to.  This is not easy, because events
   can come from multiple X windows, and a frame can also have
   multiple windows.  handle_one_xevent usually calls the function
   x_any_window_to_frame, which searches for a frame by toplevel
   window and widget windows.  There are also some other functions for
   searching by specific types of window, such as
   x_top_window_to_frame (which only searches for frames by toplevel
   window), and x_menubar_window_to_frame (which will only search
   through frame menu bars).

   INPUT FOCUS

   Under X, the window where keyboard input is sent is not always
   explicitly defined.  When there is a focus window, it receives what
   is referred to as "explicit focus", but when there is none, it
   receives "implicit focus" whenever the pointer enters it, and loses
   that focus when the pointer leaves.  When the toplevel window of a
   frame receives an explicit focus event (FocusIn or FocusOut), we
   treat that frame as having the current input focus, but when there
   is no focus window, we treat each frame as having the input focus
   whenever the pointer enters it, and undo that treatment when the
   pointer leaves it.  See the callers of x_detect_focus_change for
   more details.

   REDISPLAY

   The redisplay engine communicates with X through the "redisplay
   interface", which is a structure containing pointers to functions
   which output graphics to a frame.

   Some of the functions included in the redisplay interface include
   `x_clear_frame_area', which is called by the display engine when it
   determines that a part of the display has to be cleared,
   x_draw_window_cursor, which is called to perform the calculations
   necessary to display the cursor glyph with a special "highlight"
   (more on that later) and to set the input method spot location.

   Most of the actual display is performed by the function
   `x_draw_glyph_string', also included in the redisplay interface.
   It takes a list of glyphs of the same type and face, computes the
   correct graphics context for the string through the function
   `x_set_glyph_string_gc', and draws whichever glyphs it might
   contain, along with decorations such as the box face, underline and
   overline.  That list is referred to as a "glyph string".

   GRAPHICS CONTEXTS

   A graphics context ("GC") is an X server-side object which contains
   drawing attributes such as fill style, stipple, and foreground and
   background pixel values.

   Usually, one graphics context is computed for each face when it is
   about to be displayed for the first time, and this graphics context
   is the one which is used for future X drawing operations in a glyph
   string with that face.  (See `prepare_face_for_display' in
   xfaces.c).

   However, when drawing glyph strings for special display elements
   such as the cursor, or mouse sensitive text, different GCs may be
   used.  When displaying the cursor, for example, the frame's cursor
   graphics context is used for the common case where the cursor is
   drawn with the default font, and the colors of the string's face
   are the same as the default face.  In all other cases, a temporary
   graphics context is created with the foreground and background
   colors of the cursor face adjusted to ensure that the cursor can be
   distinguished from its surroundings and that the text inside the
   cursor stays visible.

   Various graphics contexts are also calculated when the frame is
   created by the function `x_make_gcs' in xfns.c, and are adjusted
   whenever the foreground or background colors change.  The "normal"
   graphics context is used for operations performed without a face,
   and always corresponds to the foreground and background colors of
   the frame's default face, the "reverse" graphics context is used to
   draw text in inverse video, and the cursor graphics context is used
   to display the cursor in the most common case.

   N.B. that some of the other window systems supported by use an
   emulation of graphics contexts to hold the foreground and
   background colors used in a glyph string, while the some others
   ports compute those colors directly based on the colors of the
   string's face and its highlight, but only on X are graphics
   contexts a data structure inherent to the window system.

   COLOR ALLOCATION

   In (and only in) X, pixel values for colors are not guaranteed to
   correspond to their individual components.  The rules for
   converting colors into pixel values are defined by the visual class
   of each display opened by Emacs.  When a display is opened, a
   suitable visual is obtained from the X server, and a colormap is
   created based on that visual, which is then used for each frame
   created.

   The colormap is then used by the X server to convert pixel values
   from a frame created by Emacs into actual colors which are output
   onto the physical display.

   When the visual class is TrueColor, the colormap will be indexed
   based on the red, green, and blue (RGB) components of the pixel
   values, and the colormap will be statically allocated so as to
   contain linear ramps for each component.  As such, most of the
   color allocation described below is bypassed, and the pixel values
   are computed directly from the color.

   Otherwise, each time Emacs wants a pixel value that corresponds to
   a color, Emacs has to ask the X server to obtain the pixel value
   that corresponds to a "color cell" containing the color (or a close
   approximation) from the colormap.  Exactly how this is accomplished
   further depends on the visual class, since some visuals have
   immutable colormaps which contain color cells with pre-defined
   values, while others have colormaps where the color cells are
   dynamically allocated by individual X clients.

   With visuals that have a visual class of StaticColor and StaticGray
   (where the former is the case), the X server is asked to procure
   the pixel value of a color cell that contains the closest
   approximation of the color which Emacs wants.  On the other hand,
   when the visual class is DirectColor, PseudoColor, or GrayScale,
   where color cells are dynamically allocated by clients, Emacs asks
   the X server to allocate a color cell containing the desired color,
   and uses its pixel value.

   (If the color already exists, the X server returns an existing color
   cell, but increases its reference count, so it still has to be
   freed afterwards.)

   Otherwise, if no color could be allocated (due to the colormap
   being full), Emacs looks for a color cell inside the colormap
   closest to the desired color, and uses its pixel value instead.

   Since the capacity of a colormap is finite, X clients have to take
   special precautions in order to not allocate too many color cells
   that are never used.  Emacs allocates its color cells when a face
   is being realized or when a frame changes its foreground and
   background colors, and releases them alongside the face or frame.
   See calls to `unload_color' and `load_color' in xterm.c, xfaces.c
   and xfns.c for more details.

   The driving logic behind color allocation is in
   `x_alloc_nearest_color_1', while the optimization for TrueColor
   visuals is in `x_make_truecolor_pixel'.  Also see `x_query_colors`,
   which is used to determine the color values for given pixel
   values.

   In other window systems supported by Emacs, color allocation is
   handled by the window system itself, to whom Emacs simply passes 24
   (or 32-bit) RGB values.

   OPTIONAL FEATURES

   While X servers and client libraries tend to come with many
   extensions to the core X11R6 protocol, dependencies on anything
   other than the core X11R6 protocol and Xlib should be optional at
   both compile-time and runtime.  Emacs should also not crash
   regardless of what combination of X server and client-side features
   are present.  For example, if you are developing a feature that
   will need Xfixes, then add a test in configure.ac for the library
   at compile-time which defines `HAVE_XFIXES', like this:

     ### Use Xfixes (-lXfixes) if available
     HAVE_XFIXES=no
     if test "${HAVE_X11}" = "yes"; then
       XFIXES_REQUIRED=4.0.0
       XFIXES_MODULES="xfixes >= $XFIXES_REQUIRED"
       EMACS_CHECK_MODULES([XFIXES], [$XFIXES_MODULES])
       if test $HAVE_XFIXES = no; then
	 # Test old way in case pkg-config doesn't have it (older machines).
	 AC_CHECK_HEADER([X11/extensions/Xfixes.h],
	   [AC_CHECK_LIB([Xfixes], [XFixesHideCursor], [HAVE_XFIXES=yes])])
	 if test $HAVE_XFIXES = yes; then
	   XFIXES_LIBS=-lXfixes
	 fi
       fi
       if test $HAVE_XFIXES = yes; then
	 AC_DEFINE([HAVE_XFIXES], [1],
	   [Define to 1 if you have the Xfixes extension.])
       fi
     fi
     AC_SUBST([XFIXES_CFLAGS])
     AC_SUBST([XFIXES_LIBS])

  Then, make sure to adjust CFLAGS and LIBES in src/Makefile.in and
  add the new XFIXES_CFLAGS and XFIXES_LIBS variables to
  msdos/sed1v2.inp.  (The latter has to be adjusted for any new
  variables that are included in CFLAGS and LIBES even if the
  libraries are not used by the MS-DOS port.)

  Finally, add some fields in `struct x_display_info' which specify
  the major and minor versions of the extension, and whether or not to
  support them.  They (and their accessors) should be protected by the
  `HAVE_XFIXES' preprocessor conditional.  Then, these fields should
  be set in `x_term_init', and all Xfixes calls must be protected by
  not only the preprocessor conditional, but also by checks against
  those variables.

  X TOOLKIT SUPPORT

  Emacs supports being built with many different toolkits (and also no
  toolkit at all), which provide decorations such as menu bars and
  scroll bars, along with handy features like file panels, dialog
  boxes, font panels, and popup menus.  Those configurations can
  roughly be classified as belonging to one of three categories:

    - Using no toolkit at all.
    - Using the X Toolkit Intrinsics (Xt).
    - Using GTK.

  The no toolkit configuration is the simplest: no toolkit widgets are
  used, Emacs uses its own implementation of scroll bars, and the
  XMenu library that came with X11R2 and earlier versions of X is used
  for popup menus.  There is also no complicated window structure to
  speak of.

  The Xt configurations come in either the Lucid or Motif flavors.
  The former utilizes Emacs's own Xt-based Lucid widget library for
  menus, and Xaw (or derivatives such as neXTaw and Xaw3d) for dialog
  boxes and, optionally, scroll bars.  It does not support file
  panels.  The latter uses either Motif or LessTif for menu bars,
  popup menus, dialogs and file panels.

  The GTK configurations come in the GTK+ 2 or GTK 3 configurations,
  where the toolkit provides all the aforementioned decorations and
  features.  They work mostly the same, though GTK 3 has various small
  annoyances that complicate maintenance.

  All of those configurations have various special technicalities
  about event handling and the layout of windows inside a frame that
  must be kept in mind when writing X code which is run on all of
  them.

  The no toolkit configuration has no noteworthy aspects about the
  layout of windows inside a frame, since each frame has only one
  associated window aside from scroll bars.  However, in the Xt
  configurations, every widget is a separate window, and there are
  quite a few widgets.  The "outer widget", a widget of class
  ApplicationShell, is the top-level window of a frame.  Its window is
  accessed via the macro `FRAME_OUTER_WINDOW'.  The "edit widget", a
  widget class of EmacsFrame, is a child of the outer widget that
  controls the size of a frame as known to Emacs, and is the widget
  that Emacs draws to during display operations.  The "menu bar
  widget" is the widget holding the menu bar.

  Special care must be taken when performing operations on a frame.
  Properties that are used by the window manager, for example, must be
  set on the outer widget.  Drawing, on the other hand, must be done
  to the edit widget, and button press events on the menu bar widget
  must be redirected and not sent to Xt until the Lisp code is run to
  update the menu bar.

  The EmacsFrame widget is specific to Emacs and is implemented in
  widget.c.  See that file for more details.

  In the GTK configurations, GTK widgets do not necessarily correspond
  to X windows, since the toolkit might decide to keep only a
  client-side record of the widgets for performance reasons.

  Because the GtkFixed widget that holds the "edit area" might not
  correspond to an X window, drawing operations may be directly
  performed on the outer window, with special care taken to not
  overwrite the surrounding GTK widgets.  This also means that the
  only important window for most purposes is the outer window, which
  on GTK builds can usually be accessed using the macro
  `FRAME_X_WINDOW'.

  How `handle_one_xevent' is called also depends on the configuration.
  Without a toolkit, Emacs performs all event processing by itself,
  running XPending and XNextEvent in a loop whenever there is input,
  passing the event to `handle_one_xevent'.

  When using Xt, the same is performed, but `handle_one_xevent' may
  also decide to call XtDispatchEvent on an event after Emacs finishes
  processing it.

  When using GTK, however, `handle_one_xevent' is called from an event
  filter installed on the GTK event loop.  Unless the event filter
  elects to drop the event, it will be passed to GTK right after
  leaving the event filter.

  Fortunately, `handle_one_xevent' is provided a `*finish' parameter
  that abstracts away all these details.  If it is `X_EVENT_DROP',
  then the event will not be dispatched to Xt or utilized by GTK.
  Code inside `handle_one_xevent' should thus avoid making assumptions
  about the event dispatch mechanism and use that parameter
  instead.

  FRAME RESIZING

  In the following explanations "frame size" refers to the "native
  size" of a frame as reported by the (frame.h) macros
  FRAME_PIXEL_WIDTH and FRAME_PIXEL_HEIGHT.  These specify the size of
  a frame as the values passed to/received from a toolkit and the
  window manager.  The "text size" Emacs Lisp code uses in functions
  like 'set-frame-size' or sees in the ‘width’ and 'height' frame
  parameters is only loosely related to the native size.  The
  necessary translations are provided by the macros
  FRAME_TEXT_TO_PIXEL_WIDTH and FRAME_TEXT_TO_PIXEL_HEIGHT as well as
  FRAME_PIXEL_TO_TEXT_WIDTH and FRAME_PIXEL_TO_TEXT_HEIGHT (in
  frame.h).

  Lisp functions may ask for resizing a frame either explicitly, using
  one of the interfaces provided for that purpose like, for example,
  'set-frame-size' or changing the 'height' or 'width' parameter of
  that frame, or implicitly, for example, by turning off/on or
  changing the width of fringes or scroll bars for that frame.  Any
  such request passes through the routine 'adjust_frame_size' (in
  frame.c) which decides, among others, whether the native frame size
  would really change and whether it is allowed to change it at that
  moment.  Only if 'adjust_frame_size' decides that the corresponding
  terminal's 'set_window_size_hook' may be run, it will dispatch
  execution to the appropriate function which, for X builds, is
  'x_set_window_size' in this file.

  For GTK builds, 'x_set_window_size' calls 'xg_frame_set_char_size'
  in gtkutil.c if the frame has an edit widget and
  'x_set_window_size_1' in this file otherwise.  For non-GTK builds,
  'x_set_window_size' always calls 'x_set_window_size_1' directly.

  'xg_frame_set_char_size' calls the GTK function 'gtk_window_resize'
  for the frame's outer widget; x_set_window_size_1 calls the Xlib
  function 'XResizeWindow' instead.  In either case, if Emacs thinks
  that the frame is visible, it will wait for a ConfigureNotify event
  (see below) to occur within a timeout of 'x-wait-for-event-timeout'
  (the default is 0.1 seconds).  If Emacs thinks that the frame is not
  visible, it calls 'adjust_frame_size' to run 'resize_frame_windows'
  (see below) and hopes for the best.

  Note that if Emacs receives a ConfigureEvent in response to an
  earlier resize request, the sizes specified by that event are not
  necessarily the sizes Emacs requested.  Window manager and toolkit
  may override any of the requested sizes for their own reasons.

  On X, size notifications are received as ConfigureNotify events.
  The expected reaction to such an event on the Emacs side is to
  resize all Emacs windows that are on the frame referred to by the
  event.  Since resizing Emacs windows and redisplaying their buffers
  is a costly operation, Emacs may collapse several subsequent
  ConfigureNotify events into one to avoid that Emacs falls behind in
  user interactions like resizing a frame by dragging one of its
  borders with the mouse.

  Each ConfigureEvent event specifies a window, a width and a height.
  The event loop uses 'x_top_window_to_frame' to associate the window
  with its frame.  Once the frame has been identified, on GTK the
  event is dispatched to 'xg_frame_resized'.  On Motif/Lucid
  'x_window' has installed 'EmacsFrameResize' as the routine that
  handles resize events.  In either case, these routines end up
  calling the function 'change_frame_size' in dispnew.c.  On
  non-toolkit builds the effect is to call 'change_frame_size'
  directly from the event loop.  In either case, the value true is
  passed as the DELAY argument.

  'change_frame_size' is the central function to decide whether it is
  safe to process a resize request immediately or it has to be delayed
  (usually because its DELAY argument is true).  Since resizing a
  frame's windows may run arbitrary Lisp code, Emacs cannot generally
  process resize requests during redisplay and therefore has to queue
  them.  If processing the event must be delayed, the new sizes (that
  is, the ones requested by the ConfigureEvent) are stored in the
  new_width and new_height slots of the respective frame structure,
  possibly replacing ones that have been stored there upon the receipt
  of a preceding ConfigureEvent.

  Delayed size changes are applied eventually upon calls of the
  function 'do_pending_window_change' (in dispnew.c) which is called
  by the redisplay code at suitable spots where it's safe to change
  sizes.  'do_pending_window_change' calls 'change_frame_size' with
  its DELAY argument false in the hope that it is now safe to call the
  function 'resize_frame_windows' (in window.c) which is in charge of
  adjusting the sizes of all Emacs windows on the frame accordingly.
  Note that if 'resize_frame_windows' decides that the windows of a
  frame do not fit into the constraints set up by the new frame sizes,
  it will resize the windows to some minimum sizes with the effect
  that parts of the frame at the right and bottom will appear clipped
  off.

  In addition to explicitly passing width and height values in
  functions like 'gtk_window_resize' or 'XResizeWindow', Emacs also
  sets window manager size hints - a more implicit form of asking for
  the size Emacs would like its frames to assume.  Some of these hints
  only restate the size and the position explicitly requested for a
  frame.  Another hint specifies the increments in which the window
  manager should resize a frame to - either set to the default
  character size of a frame or to one pixel for a non-nil value of
  'frame-resize-pixelwise'.  See the function 'x_wm_set_size_hint' -
  in gtkutil.c for GTK and in this file for other builds - for the
  details.

  We have not discussed here a number of special issues like, for
  example, how to handle size requests and notifications for maximized
  and fullscreen frames or how to resize child frames.  Some of these
  require special treatment depending on the desktop or window manager
  used.

  One thing that might come handy when investigating problems wrt
  resizing frames is the variable 'frame-size-history'.  Setting this
  to a non-nil value, will cause Emacs to start recording frame size
  adjustments, usually specified by the function that asked for an
  adjustment, a sizes part that records the old and new values of the
  frame's width and height and maybe some additional information.  The
  internal function `frame--size-history' can then be used to display
  the value of this variable in a more readable form.

  FRAME RESIZE SYNCHRONIZATION

  The X window system operates asynchronously.  That is to say, the
  window manager and X server might think a window has been resized
  before Emacs has a chance to process the ConfigureNotify event that
  was sent.

  When a compositing manager is present, and the X server and Emacs
  both support the X synchronization extension, the semi-standard
  frame synchronization protocol can be used to notify the compositing
  manager of when Emacs has actually finished redisplaying the
  contents of a frame after a resize.  The compositing manager will
  customarily then postpone displaying the contents of the frame until
  the redisplay is complete.

  Emacs announces support for this protocol by creating an X
  server-side counter object, and setting it as the
  `_NET_WM_SYNC_REQUEST_COUNTER' property of the frame's top-level
  window.  The window manager then initiates the synchronized resize
  process by sending Emacs a ClientMessage event before the
  ConfigureNotify event where:

    type = ClientMessage
    window = the respective client window
    message_type = WM_PROTOCOLS
    format = 32
    data.l[0] = _NET_WM_SYNC_REQUEST
    data.l[1] = timestamp
    data.l[2] = low 32 bits of a provided frame counter value
    data.l[3] = high 32 bits of a provided frame counter value
    data.l[4] = 1 if the extended frame counter should be updated,
    otherwise 0

  Upon receiving such an event, Emacs constructs and saves a counter
  value from the provided low and high 32 bits.  Then, when the
  display engine tells us that a frame has been completely updated
  (presumably because of a redisplay caused by a ConfigureNotify
  event), we set the counter to the saved value, telling the
  compositing manager that the contents of the window now accurately
  reflect the new size.  The compositing manager will then display the
  contents of the window, and the window manager might also postpone
  updating the window decorations until this moment.

  DRAG AND DROP

  Drag and drop in Emacs is implemented in two ways, depending on
  which side initiated the drag-and-drop operation.  When another X
  client initiates a drag, and the user drops something on Emacs, a
  `drag-n-drop-event' is sent with the contents of the ClientMessage,
  and further processing (i.e. retrieving selection contents and
  replying to the initiating client) is performed from Lisp inside
  `x-dnd.el'.

  However, dragging contents from Emacs is implemented almost entirely
  in C.  X Windows has several competing drag-and-drop protocols, of
  which Emacs supports two on the C level: the XDND protocol (see
  https://freedesktop.org/wiki/Specifications/XDND) and the Motif drag
  and drop protocols.  These protocols are based on the initiator
  owning a special selection, specifying an action the recipient
  should perform, grabbing the mouse, and sending various different
  client messages to the toplevel window underneath the mouse as it
  moves, or when buttons are released.

  The Lisp interface to drag-and-drop is synchronous, and involves
  running a nested event loop with some global state until the drag
  finishes.  When the mouse moves, Emacs looks up the toplevel window
  underneath the pointer (the target window) either using a cache
  provided by window managers that support the
  _NET_WM_CLIENT_LIST_STACKING root window property, or by calling
  XTranslateCoordinates in a loop until a toplevel window is found,
  and sends various entry, exit, or motion events to the window
  containing a list of targets the special selection can be converted
  to, and the chosen action that the recipient should perform.  The
  recipient can then send messages in reply detailing the action it
  has actually chosen to perform.  Finally, when the mouse buttons are
  released over the recipient window, Emacs sends a "drop" message to
  the target window, waits for a reply, and returns the action
  selected by the recipient to the Lisp code that initiated the
  drag-and-drop operation.

  When a drop happens on a window not supporting any protocol
  implemented on the C level, the function inside
  `x-dnd-unsupported-drop-function' is called with some parameters of
  the drop.  If it returns non-nil, then Emacs tries to simulate a
  drop happening with the primary selection and synthetic button
  events (see `x_dnd_do_unsupported_drop').  That function implements
  the OffiX drag-and-drop protocol by default.  See
  `x-dnd-handle-unsupported-drop' in `x-dnd.el' for more details.

  DISPLAY ERROR HANDLING

  While error handling under X was originally designed solely as a
  mechanism for the X server to report fatal errors to clients, most
  clients (including Emacs) have adopted a system of "error traps" to
  handle or discard these errors as they arrive.  Discarding errors is
  usually necessary when Emacs performs an X request that might fail:
  for example, sending a message to a window that may no longer exist,
  or might not exist at all.  Handling errors is then necessary when
  the detailed error must be reported to another piece of code: for
  example, as a Lisp error.

  It is not acceptable for Emacs to crash when it is sent invalid data
  by another client, or by Lisp.  As a result, errors must be caught
  around Xlib functions generating requests containing resource
  identifiers that could potentially be invalid, such as window or
  atom identifiers provided in a client message from another program,
  or a child window ID obtained through XTranslateCoordinates that may
  refer to a window that has been deleted in the meantime.

  There are two sets of functions used to perform this "error
  trapping".  Which one should be used depends on what kind of
  processing must be done on the error.  The first consists of the
  functions `x_ignore_errors_for_next_request' and
  `x_stop_ignoring_errors', which ignore errors generated by requests
  made in between a call to the first function and a corresponding
  call to the second.  They should be used for simple asynchronous
  requests that do not require a reply from the X server: using them
  instead of the second set improves performance, as they simply
  record a range of request serials to ignore errors from, instead of
  synchronizing with the X server to handle errors.

  The second set consists of the following functions:

    - x_catch_errors_with_handler
    - x_catch_errors
    - x_uncatch_errors_after_check
    - x_uncatch_errors
    - x_check_errors
    - x_had_errors_p
    - x_clear_errors

  Callers using this set should consult the comment(s) on top of the
  aforementioned functions.  They should not be used when the requests
  being made do not require roundtrips to the X server, and obtaining
  the details of any error generated is unnecessary, as
  `x_uncatch_errors' will always synchronize with the X server, which
  is a potentially slow operation.  */

#include <config.h>

#include "lisp.h"
#include "blockinput.h"

#include <stdio.h>
#include <wayland-client.h>
#include <stdbool.h>

#include "wlterm.h"
#include "termhooks.h"
#include "keyboard.h"
#include "frame.h"

/* This is a chain of structures for all the Wayland displays currently in
   use.  */

struct wl_display_info *x_display_list;

static int
wl_read_socket (struct terminal *terminal, struct input_event *hold_quit)
{
  /* TODO */
  /* int count = 0; */
  /* bool event_found = false; */
  /* struct wl_display_info *dpyinfo = terminal->display_info.wayland; */

  /* while (wl_display_dispatch(dpyinfo->display) != -1) { */
  /*   /\* This space deliberately left blank *\/ */
  /* } */
  return 0;
};




/* Defined in wlfns.c */
extern frame_parm_handler wl_frame_parm_handlers[];

wl_default_font_parameter (struct frame *f, Lisp_Object parms)
{
  struct wl_display_info *dpyinfo = FRAME_DISPLAY_INFO (f);
  Lisp_Object font_param = gui_display_get_arg (dpyinfo, parms, Qfont, NULL, NULL,
                                                RES_TYPE_STRING);
  Lisp_Object font = Qnil;
  if (BASE_EQ (font_param, Qunbound))
    font_param = Qnil;

  /* if (NILP (font_param)) */
  /*   /\* System font should take precedence over X resources.  We */
  /*      suggest this regardless of font-use-system-font because .emacs */
  /*      may not have been read yet.  Returning a font-spec is Haiku */
  /*      specific behavior.  *\/ */
  /*   font = font_open_by_spec (f, Ffont_get_system_font ()); */

  if (NILP (font))
    font = (!NILP (font_param)
	    ? font_param
	    : gui_display_get_arg (dpyinfo, parms, Qfont,
				   "font", "Font",
				   RES_TYPE_STRING));

  if (! FONTP (font) && ! STRINGP (font))
    {
      const char **names = (const char *[]) { "monospace-12",
					      "Noto Sans Mono-12",
					      "Source Code Pro-12",
					      NULL };
      int i;

      for (i = 0; names[i]; i++)
        {
          font
            = font_open_by_name (f, build_unibyte_string (names[i]));
          if (!NILP (font))
            break;
        }
      if (NILP (font))
        error ("No suitable font was found");
    }

  gui_default_parameter (f, parms, Qfont, font, "font", "Font",
                         RES_TYPE_STRING);
}

/* Set up use of Wayland before we make the first connection.  */

static struct redisplay_interface wl_redisplay_interface = {
  wl_frame_parm_handlers,
  gui_produce_glyphs,
  gui_write_glyphs,
  gui_insert_glyphs,
  gui_clear_end_of_line,
  wr_scroll_run,
  wr_after_update_window_line,
  wr_update_window_begin,
  wr_update_window_end,
  wr_flush_display,
  gui_clear_window_mouse_face,
  gui_get_glyph_overhangs,
  gui_fix_overlapping_area,
  0, /* wr_draw_fringe_bitmap, */
  0, /* define_fringe_bitmap */
  0, /* destroy_fringe_bitmap */
  0, /* compute_glyph_string_overhangs */
  0, /* wr_draw_glyph_string, */
  0, /* wl_define_frame_cursor, */
  0, /* wr_clear_frame_area, */
  0, /* wl_clear_under_internal_border, */
  0, /* wr_draw_window_cursor, */
  0, /* wr_draw_vertical_window_border, */
  0, /* wr_draw_window_divider, */
  0, /* wl_shift_glyphs_for_insert, /\* Never called; see comment in function.  *\/   */
  0, /* wl_show_hourglass, */
  0, /* wl_hide_hourglass, */
  wl_default_font_parameter,
};


/* This function is called when the last frame on a display is deleted. */
void
wl_delete_terminal (struct terminal *terminal)
{
  /* TODO */
  /* struct wl_display_info *dpyinfo; */
  /* struct frame *f; */
  /* Lisp_Object tail, frame; */

  /* dpyinfo = terminal->display_info.wayland; */

  /* /\* Protect against recursive calls.  delete_frame in */
  /*    delete_terminal calls us back when it deletes our last frame.  *\/ */
  /* if (!terminal->name) */
  /*   return; */

  /* block_input (); */


  /* /\* No more input on this descriptor.  *\/ */
  /* delete_keyboard_wait_descriptor (dpyinfo->connection); */
  /* /\* Mark as dead. *\/ */
  /* dpyinfo->connection = -1; */

  /* x_delete_display (dpyinfo); */
  /* unblock_input (); */
}


/* Create a struct terminal, initialize it with the Wayland specific
   functions and make DISPLAY->TERMINAL point to it.  */

static struct terminal *
wl_create_terminal (struct wl_display_info *dpyinfo)
{
  struct terminal *terminal;

  terminal = create_terminal (output_wayland, &wl_redisplay_interface);

  terminal->display_info.wayland = dpyinfo;
  dpyinfo->terminal = terminal;

  /* kboard is initialized in wl_term_init. */

/*   terminal->clear_frame_hook = wl_clear_frame; */
/*   terminal->ins_del_lines_hook = x_ins_del_lines; */
/*   terminal->delete_glyphs_hook = x_delete_glyphs; */
/*   terminal->ring_bell_hook = XTring_bell; */
/*   terminal->toggle_invisible_pointer_hook = XTtoggle_invisible_pointer; */
/*   terminal->update_begin_hook = x_update_begin; */
/*   terminal->update_end_hook = x_update_end; */
  terminal->read_socket_hook = wl_read_socket;
/*   terminal->frame_up_to_date_hook = XTframe_up_to_date; */
/* #ifdef HAVE_XDBE */
/*   terminal->buffer_flipping_unblocked_hook = XTbuffer_flipping_unblocked_hook; */
/* #endif */
/*   terminal->defined_color_hook = x_defined_color; */
/*   terminal->query_frame_background_color = x_query_frame_background_color; */
/*   terminal->query_colors = x_query_colors; */
/*   terminal->mouse_position_hook = XTmouse_position; */
/*   terminal->get_focus_frame = x_get_focus_frame; */
/*   terminal->focus_frame_hook = x_focus_frame; */
/*   terminal->frame_rehighlight_hook = XTframe_rehighlight; */
/*   terminal->frame_raise_lower_hook = XTframe_raise_lower; */
/*   terminal->frame_visible_invisible_hook = x_make_frame_visible_invisible; */
/*   terminal->fullscreen_hook = XTfullscreen_hook; */
/*   terminal->iconify_frame_hook = x_iconify_frame; */
/*   terminal->set_window_size_hook = x_set_window_size; */
/*   terminal->set_frame_offset_hook = x_set_offset; */
/*   terminal->set_frame_alpha_hook = x_set_frame_alpha; */
  terminal->set_new_font_hook = wr_new_font;
/*   terminal->set_bitmap_icon_hook = x_bitmap_icon; */
/*   terminal->implicit_set_name_hook = x_implicitly_set_name; */
/*   terminal->menu_show_hook = x_menu_show; */
/* #ifdef HAVE_EXT_MENU_BAR */
/*   terminal->activate_menubar_hook = x_activate_menubar; */
/* #endif */
/* #if defined (USE_X_TOOLKIT) || defined (USE_GTK) */
/*   terminal->popup_dialog_hook = xw_popup_dialog; */
/* #endif */
/*   terminal->change_tab_bar_height_hook = x_change_tab_bar_height; */
/* #ifndef HAVE_EXT_TOOL_BAR */
/*   terminal->change_tool_bar_height_hook = x_change_tool_bar_height; */
/* #endif */
/*   terminal->set_vertical_scroll_bar_hook = XTset_vertical_scroll_bar; */
/*   terminal->set_horizontal_scroll_bar_hook = XTset_horizontal_scroll_bar; */
/*   terminal->set_scroll_bar_default_width_hook = x_set_scroll_bar_default_width; */
/*   terminal->set_scroll_bar_default_height_hook = x_set_scroll_bar_default_height; */
/*   terminal->condemn_scroll_bars_hook = XTcondemn_scroll_bars; */
/*   terminal->redeem_scroll_bar_hook = XTredeem_scroll_bar; */
/*   terminal->judge_scroll_bars_hook = XTjudge_scroll_bars; */
/*   terminal->get_string_resource_hook = x_get_string_resource; */
/*   terminal->free_pixmap = x_free_pixmap; */
/*   terminal->delete_frame_hook = x_destroy_window; */
/*   terminal->delete_terminal_hook = x_delete_terminal; */
/*   terminal->toolkit_position_hook = x_toolkit_position; */
/* #ifdef HAVE_XINPUT2 */
/*   terminal->any_grab_hook = x_have_any_grab; */
/* #endif */
  /* Other hooks are NULL by default.  */

  return terminal;
}

static void
registry_handle_global(void *data, struct wl_registry *registry,
		uint32_t name, const char *interface, uint32_t version)
{
  /* printf("interface: '%s', version: %d, name: %d\n", */
  /* 	 interface, version, name);   */

  struct wl_display_info *dpyinfo = data;

  if (strcmp(interface, wl_compositor_interface.name) == 0) {
    dpyinfo->compositor = wl_registry_bind(
					   registry, name, &wl_compositor_interface, 4);
  }
}

static void
registry_handle_global_remove(void *data, struct wl_registry *registry,
		uint32_t name)
{
	// This space deliberately left blank
}

static const struct wl_registry_listener
registry_listener = {
	.global = registry_handle_global,
	.global_remove = registry_handle_global_remove,
};

/* Test whether two display-name strings agree up to the dot that separates
   the screen number from the server number.  */
static bool
same_x_server (const char *name1, const char *name2)
{
  bool seen_colon = false;
  Lisp_Object sysname = Fsystem_name ();
  const char *system_name = SSDATA (sysname);
  ptrdiff_t system_name_length = SBYTES (sysname);
  ptrdiff_t length_until_period = 0;

  while (system_name[length_until_period] != 0
	 && system_name[length_until_period] != '.')
    length_until_period++;

  /* Treat `unix' like an empty host name.  */
  if (!strncmp (name1, "unix:", 5))
    name1 += 4;
  if (!strncmp (name2, "unix:", 5))
    name2 += 4;
  /* Treat this host's name like an empty host name.  */
  if (!strncmp (name1, system_name, system_name_length)
      && name1[system_name_length] == ':')
    name1 += system_name_length;
  if (!strncmp (name2, system_name, system_name_length)
      && name2[system_name_length] == ':')
    name2 += system_name_length;
  /* Treat this host's domainless name like an empty host name.  */
  if (!strncmp (name1, system_name, length_until_period)
      && name1[length_until_period] == ':')
    name1 += length_until_period;
  if (!strncmp (name2, system_name, length_until_period)
      && name2[length_until_period] == ':')
    name2 += length_until_period;

  for (; *name1 != '\0' && *name1 == *name2; name1++, name2++)
    {
      if (*name1 == ':')
	seen_colon = true;
      if (seen_colon && *name1 == '.')
	return true;
    }
  return (seen_colon
	  && (*name1 == '.' || *name1 == '\0')
	  && (*name2 == '.' || *name2 == '\0'));
}

/* Open a connection to Wayland display DISPLAY_NAME, and return the
   structure that describes the open display.  If obtaining the Wayland
   connection or display fails, return NULL.  Signal an error if opening
   the display itself failed.  */

struct wl_display_info *
wl_term_init (Lisp_Object display_name)
{
  struct terminal *terminal;
  struct wl_display_info *dpyinfo;
  struct wl_display *display;

  if (NILP (display_name)) {
    display = wl_display_connect(NULL);
  } else {
    CHECK_STRING (display_name);
    display = wl_display_connect(SSDATA (display_name));
  }

  if (!display)
    return NULL;

  /* We have definitely succeeded.  Record the new connection.  */

  dpyinfo = xzalloc (sizeof *dpyinfo);
  terminal = wl_create_terminal (dpyinfo);

  {
    struct wl_display_info *share;

    for (share = x_display_list; share; share = share->next)
      if (same_x_server (SSDATA (XCAR (share->name_list_element)), SSDATA (display_name)))
	break;
    if (share)
      terminal->kboard = share->terminal->kboard;
    else
      {
	terminal->kboard = allocate_kboard (Qwayland);

	/* Don't let the initial kboard remain current longer than necessary.
	   That would cause problems if a file loaded on startup tries to
	   prompt in the mini-buffer.  */
	if (current_kboard == initial_kboard)
	  current_kboard = terminal->kboard;
      }
    terminal->kboard->reference_count++;
  }

  /* binds wayland registry global and save it dpyinfo */
  struct wl_registry *registry = wl_display_get_registry(display);
  wl_registry_add_listener(registry, &registry_listener, dpyinfo);
  wl_display_roundtrip(display);

  /* Put this display on the chain.  */
  dpyinfo->next = x_display_list;
  x_display_list = dpyinfo;

  dpyinfo->name_list_element = Fcons (display_name, Qnil);
  dpyinfo->display = display;

  /* Set the name of the terminal. */
  terminal->name = xlispstrdup (display_name);

  return dpyinfo;
}

/* Move the mouse to position pixel PIX_X, PIX_Y relative to frame F.  */

void
frame_set_mouse_pixel_position (struct frame *f, int pix_x, int pix_y)
{
}

/* Convert a keysym to its name.  */

char *
get_keysym_name (int keysym)
{
  char *value;

  block_input ();
  /* value = XKeysymToString (keysym); */
  unblock_input ();

  return value;
}

void
syms_of_wlterm (void)
{
  DEFVAR_LISP ("x-keysym-table", Vx_keysym_table,
	       doc: /* Hash table of character codes indexed by X keysym codes.  */);
  Vx_keysym_table = make_hash_table (&hashtest_eql, 900, Weak_None, false);

  /* Tell Emacs about this window system.  */
  Fprovide (Qwayland, Qnil);

  DEFVAR_BOOL ("x-use-underline-position-properties",
	       x_use_underline_position_properties,
     doc: /* SKIP: real doc in xterm.c.  */);
  x_use_underline_position_properties = 1;

  DEFVAR_BOOL ("x-underline-at-descent-line",
	       x_underline_at_descent_line,
     doc: /* SKIP: real doc in xterm.c.  */);
  x_underline_at_descent_line = 0;  
}
