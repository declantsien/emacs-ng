# Wayland Emacs

Emacs Wayland support directly using wayland-client library.

## Using WebRender native for rendering
Like [the way](https://github.com/mozilla/gecko-dev/tree/master/gfx/webrender_bindings) Firefox is doing.

- PGTK with WebRender (instead of Cairo) already been done on emacs-ng, which can be reused with Wayland and other Window system.
Just need to move most the Rust code to C/C++ in order to upstream to GNU.
- Reuse FreeType/HarfBuzz font driver
- Less changes to WebRender itself.
