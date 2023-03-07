# `gl_loader`: an OpenGL function pointer loader based on Glad

This (small) package aim to do *only one thing*: Provide a `get_proc_address` function to load OpenGL function pointer.

It will not provide any form of window or OpenGL context creation. You will have to handle them by yourself.

The code is simply a binding to the pointer loading part of the glad library: [glad](https://glad.dav1d.de/)

## Explanation

When using the OpenGL's headers, the function are just declaration, empty shell. 
Trying to call them will result in whatever happen when you try to use a null pointer as a function.

To really be able to use those functions, you need to load their pointer.
The OS will provide them for you by usually exposing a `getProcAddress` function (or equivalent) which take the OpenGL function name and output its pointer.

Therefor, this crate aim to provide a OS-independant way to load the OpenGL function pointer.

## Why 

The main idea is to be able to make a OpenGL-based engine that is complitely agnostic from the GUI/Windowing library used.
Let's say for exemple that you want to make an editor that use libGUI-X and package your final application with libWindow-Y. 
Both these system should handle the window and context creation but might have a way to load function pointer which is uncompatible or require you to do dirty hacks in your engine to handle both case.

By deffering the function pointer loading to this small library, you should be able to make your engine complitely agnostic as long as a proper OpenGL-context has been created and made current on the same thread.

## How to use

First you will need the following requirement:
1 - An OpenGL context has been created and made current
2 - You are calling the `get_proc_address` from *the same thread* has the one which owns the current context

Then, using the [gl](https://crates.io/crates/gl) crate, you should be able to just do:
```
extern crate gl_loader;

fn load_gl_symbol() {
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}
```

Your function pointer should be loaded and you should be able to use OpenGL.

### NOTE:

You *need* to call `gl_loader::init_gl()` *before* calling `gl_loader::get_proc_address(...)`, `init_gl()` will make sure that the OpenGL Library/Framework are properly loaded.

## Goal

Right now, some of the dependencies are relying on the std.
The final goal is to make the package fully `#![no_std]` compatible.

## Credit

**glutin contributors**: The original code was based on the glutin source code.

**glad contributors**: The current code is a binding to the specific part of Glad that handle function pointer loading.

**[Fizzer](http://www.pouet.net/user.php?who=82805) and [Ferris](http://www.pouet.net/user.php?who=16820)**: For giving me the idea and explaining some of the specific to me