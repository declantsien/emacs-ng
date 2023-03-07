#![doc = include_str!("../Readme.md")]

use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::*;
use std::io;
use std::path::Path;
use std::ptr::*;

/// The error type of this library, [std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html)
pub type Error = std::io::Error;

/// The result type of this library, [std::io::Result](https://doc.rust-lang.org/std/io/struct.Result.html)
pub type Result<T> = std::io::Result<T>;

/// A loaded library handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Library(*mut c_void);
unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Library {
    /// Load a library, forever.
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `LoadLibraryW(path)`
    /// | Unix      | `dlopen(path, ...)`
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        #[cfg(windows)] let handle = {
            use std::os::windows::ffi::OsStrExt;
            let filename = path.as_os_str().encode_wide().chain([0].iter().copied()).collect::<Vec<u16>>();
            unsafe { LoadLibraryW(filename.as_ptr()) }
        };

        #[cfg(unix)] let handle = {
            use std::os::unix::ffi::OsStrExt;
            let filename = path.as_os_str().as_bytes().iter().copied().chain([0].iter().copied()).collect::<Vec<u8>>();
            let _ = unsafe { dlerror() }; // clear error code
            unsafe { dlopen(filename.as_ptr() as _, RTLD_LAZY) }
        };

        if handle != null_mut() {
            Ok(Self(handle))
        } else {
            #[cfg(windows)] {
                let err = Error::last_os_error();
                match err.raw_os_error() {
                    Some(ERROR_BAD_EXE_FORMAT) => {
                        Err(io::Error::new(io::ErrorKind::Other, format!(
                            "Unable to load {path}: ERROR_BAD_EXE_FORMAT (likely tried to load a {that}-bit DLL into this {this}-bit process)",
                            path = path.display(),
                            this = if cfg!(target_arch = "x86_64") { "64" } else { "32" },
                            that = if cfg!(target_arch = "x86_64") { "32" } else { "64" },
                        )))
                    },
                    Some(ERROR_MOD_NOT_FOUND) => {
                        Err(io::Error::new(io::ErrorKind::NotFound, format!(
                            "Unable to load {path}: NotFound",
                            path = path.display(),
                        )))
                    },
                    _ => Err(err)
                }
            }
            #[cfg(unix)] {
                // dlerror already contains path info
                Err(io::Error::new(io::ErrorKind::Other, unsafe { std::ffi::CStr::from_ptr(dlerror()) }.to_string_lossy()))
            }
        }
    }

    /// Load a symbol from the library.
    /// Note that the symbol name must end with '\0'.
    /// Limiting yourself to basic ASCII is also likely wise.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., name)`
    /// | Unix      | `dlsym(..., name)`
    pub unsafe fn sym<'a, T>(&self, name: impl AsRef<str>) -> io::Result<T> {
        let name = name.as_ref();
        self.sym_opt(name).ok_or_else(||{
            io::Error::new(io::ErrorKind::InvalidInput, format!("Symbol {:?} missing from library", &name[..name.len()-1]))
        })
    }

    /// Load a symbol from the library.
    /// Note that the symbol name must end with '\0'.
    /// Limiting yourself to basic ASCII is also likely wise.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., name)`
    /// | Unix      | `dlsym(..., name)`
    pub unsafe fn sym_opt<'a, T>(&self, name: impl AsRef<str>) -> Option<T> {
        let name = name.as_ref();
        let module = self.0;
        let n = name.len();
        assert_eq!(size_of::<T>(), size_of::<*mut c_void>(), "symbol result is not pointer sized!");
        assert!(name.ends_with('\0'),           "symbol name must end with '\0'");
        assert!(!name[..n-1].contains('\0'),    "symbol name mustn't contain '\0's, except to terminate the string");

        let cname = name.as_ptr() as _;
        #[cfg(windows)] let result = GetProcAddress(module, cname);
        #[cfg(unix)] let result = dlsym(module, cname);

        if result == null_mut() {
            None
        } else {
            Some(std::ptr::read(&result as *const *mut c_void as *const T))
        }
    }

    /// Load a symbol from the library by ordinal.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    /// Additionally, DLL ordinals are typically unstable and might change between minor versions of the same DLL, breaking your imports in nastily subtle ways.
    /// If a function name is available, use it instead!
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., MAKEINTRESOURCE(ordinal))`
    /// | <strike>Unix</strike> | `Err(...)`
    pub unsafe fn sym_by_ordinal<T>(self, ordinal: u16) -> io::Result<T> {
        self.sym_opt_by_ordinal(ordinal).ok_or_else(||{
            io::Error::new(io::ErrorKind::InvalidInput, format!("Symbol @{ordinal} missing from library"))
        })
    }

    /// Load a symbol from the library by ordinal.
    ///
    /// # Safety
    ///
    /// This function implicitly transmutes!  Use extreme caution.
    /// Additionally, DLL ordinals are typically unstable and might change between minor versions of the same DLL, breaking your imports in nastily subtle ways.
    /// If a function name is available, use it instead!
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `GetProcAddress(..., MAKEINTRESOURCE(ordinal))`
    /// | <strike>Unix</strike> | `None`
    pub unsafe fn sym_opt_by_ordinal<T>(self, ordinal: u16) -> Option<T> {
        assert_eq!(size_of::<T>(), size_of::<*mut c_void>(), "symbol result is not pointer sized!");

        // SAFETY: ✔️
        //  * `hModule`     ✔️ is a valid, non-dangling, permanently loaded hmodule
        //  * `lpProcName`  ✔️ is a WORD/u16, meeting GetProcAddress's documented requirement:
        //                  "If this parameter is an ordinal value, it must be in the low-order word; the high-order word must be zero."
        #[cfg(windows)] let func = GetProcAddress(self.0, ordinal as usize as *const _);
        #[cfg(unix)] let func = null_mut::<c_void>();
        #[cfg(unix)] let _ = ordinal;

        if func.is_null() {
            None
        } else {
            // SAFETY: ✔️
            //  * `T`   ✔️ is asserted to be the same size as `*mut c_void` via assert at start of function (can't enforce this at compile time)
            //  * `T`   ✔️ is assumed compatible with `*mut c_void` per the documented safety contract of this unsafe function
            Some(std::mem::transmute_copy::<*mut c_void, T>(&func))
        }
    }

    /// Check if a symbol existing in the library.
    /// Note that the symbol name must end with '\0'.
    /// Limiting yourself to basic ASCII is also likely wise.
    ///
    /// # Platform
    ///
    /// | OS        | Behavior |
    /// | --------- | -------- |
    /// | Windows   | `!!GetProcAddress(..., name)`
    /// | Unix      | `!!dlsym(..., name)`
    pub fn has_sym(self, name: impl AsRef<str>) -> bool {
        // SAFETY: ✔️ cast to `*mut c_void` should always be safe.
        let s : Option<*mut c_void> = unsafe { self.sym_opt(name) };
        s.is_some()
    }
}

#[cfg(windows)] const ERROR_BAD_EXE_FORMAT : i32 = 0x00C1;
#[cfg(windows)] const ERROR_MOD_NOT_FOUND  : i32 = 0x007E;
#[cfg(windows)] extern "system" {
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const c_char) -> *mut c_void;
    fn LoadLibraryW(lpFileName: *const u16) -> *mut c_void;
}

#[cfg(unix)] const RTLD_LAZY : c_int = 1;
#[cfg(unix)] extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}
