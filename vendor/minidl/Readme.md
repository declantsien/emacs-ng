# minidl

[![Crates.io](https://img.shields.io/crates/v/minidl.svg)](https://crates.io/crates/minidl)
[![Docs](https://docs.rs/minidl/badge.svg)](https://docs.rs/minidl/)
[![unsafe: yes](https://img.shields.io/github/search/MaulingMonkey/minidl/unsafe%2bextension%3Ars?color=yellow&label=unsafe)](https://github.com/MaulingMonkey/minidl/search?q=unsafe+extension%3Ars)
[![rust: stable](https://img.shields.io/badge/rust-stable-yellow.svg)](https://gist.github.com/MaulingMonkey/c81a9f18811079f19326dac4daa5a359#minimum-supported-rust-versions-msrv)
[![License](https://img.shields.io/crates/l/minidl.svg)](https://github.com/MaulingMonkey/minidl)
<br>
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/minidl.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/minidl)
[![Build Status](https://github.com/MaulingMonkey/minidl/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/minidl/actions?query=workflow%3Arust)
[![Open issues](https://img.shields.io/github/issues-raw/MaulingMonkey/minidl.svg)](https://github.com/MaulingMonkey/minidl/issues)
[![dependency status](https://deps.rs/repo/github/MaulingMonkey/minidl/status.svg)](https://deps.rs/repo/github/MaulingMonkey/minidl)

Extremely lean cross platform library for loading symbols.

* No dependencies (minimal build times)
* No macros (minimal build times)
* No safety (ABI mismatches would be unsound anyways)
* No unloading (would be unsound if the library spawned threads anyways)

## Quick Start

```rust
use minidl::*;
use std::os::raw::*;

struct Example {
    OutputDebugStringA: unsafe extern "system" fn (_: *const c_char),
    Invalid_Optional:   Option<unsafe extern "system" fn (_: *const c_char)>,
}

impl Example {
    pub fn new() -> Result<Self> {
        let lib = Library::load("kernel32.dll")?;
        unsafe{Ok(Self{
            OutputDebugStringA: lib.sym("OutputDebugStringA\0")?,
            Invalid_Optional:   lib.sym_opt("Invalid_Optional\0"),
        })}
    }
}
```

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->
