//! This module contains Rust definitions whose C equivalents live in
//! lisp.h.

use core::ptr::NonNull;
use std::ffi::CString;
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;

use libc::c_void;
use libc::intptr_t;
use libc::uintptr_t;

use crate::bindings::build_string;
use crate::bindings::make_string_from_utf8;
use crate::bindings::pI;
use crate::bindings::valid_lisp_object_p;
use crate::bindings::Aligned_Lisp_Subr;
use crate::bindings::Fprin1_to_string;
use crate::bindings::Lisp_Bits;
use crate::bindings::Lisp_Type;
use crate::bindings::USER_PTRP;
use crate::bindings::XLI;
use crate::bindings::XUSER_PTR;
use crate::definitions::EmacsInt;
use crate::definitions::EmacsUint;
use crate::definitions::USE_LSB_TAG;
use crate::globals::Qexternal_debugging_output;
use crate::globals::Qnil;
use crate::globals::Qt;
use crate::globals::Quser_ptrp;
use crate::multibyte::LispStringRef;
use crate::sys::VALMASK;

// TODO: tweak Makefile to rebuild C files if this changes.

/// Emacs values are represented as tagged pointers. A few bits are
/// used to represent the type, and the remaining bits are either used
/// to store the value directly (e.g. integers) or the address of a
/// more complex data type (e.g. a cons cell).
///
/// TODO: example representations
///
/// `EmacsInt` represents an integer big enough to hold our tagged
/// pointer representation.
///
/// In Emacs C, this is `EMACS_INT`.
///
/// `EmacsUint` represents the unsigned equivalent of `EmacsInt`.
/// In Emacs C, this is `EMACS_UINT`.
///
/// Their definition are determined in a way consistent with Emacs C.
/// Under casual systems, they're the type isize and usize respectively.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(pub EmacsInt);

impl fmt::Debug for LispObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let valid = unsafe { valid_lisp_object_p(*self) };
        if valid > 0 {
            let loutput = unsafe { Fprin1_to_string(*self, Qexternal_debugging_output, Qnil) };
            let output: String = loutput.into();
            return write!(f, "{}", output);
        } else {
            let n = unsafe { XLI(*self) };
            let prefix = {
                if valid == 0 {
                    "INVALID"
                } else {
                    "SOME"
                }
            };
            write!(
                f,
                "#<{prefix}_LISP_OBJECT 0x{:08}{}x>\r\n",
                n,
                std::str::from_utf8(pI).unwrap()
            )
        }
    }
}

impl LispObject {
    pub const fn from_C(n: EmacsInt) -> Self {
        Self(n)
    }

    pub fn from_C_unsigned(n: EmacsUint) -> Self {
        Self::from_C(n as EmacsInt)
    }

    pub const fn to_C(self) -> EmacsInt {
        self.0
    }

    pub const fn to_C_unsigned(self) -> EmacsUint {
        self.0 as EmacsUint
    }
}

impl From<EmacsInt> for LispObject {
    fn from(o: EmacsInt) -> Self {
        LispObject::from_C(o)
    }
}

impl From<LispObject> for EmacsInt {
    fn from(o: LispObject) -> Self {
        LispObject::to_C(o)
    }
}

impl From<()> for LispObject {
    fn from(_v: ()) -> Self {
        Qnil
    }
}

impl From<LispObject> for bool {
    fn from(o: LispObject) -> Self {
        o.is_not_nil()
    }
}

impl From<bool> for LispObject {
    fn from(v: bool) -> Self {
        if v {
            Qt
        } else {
            Qnil
        }
    }
}

impl From<usize> for LispObject {
    fn from(v: usize) -> Self {
        Self::from_natnum(v as EmacsUint)
    }
}

impl From<i32> for LispObject {
    fn from(v: i32) -> Self {
        Self::from_fixnum(EmacsInt::from(v))
    }
}

impl LispObject {
    pub fn is_nil(self) -> bool {
        self == Qnil
    }

    pub fn is_not_nil(self) -> bool {
        self != Qnil
    }

    pub fn is_t(self) -> bool {
        self == Qt
    }

    pub fn eq(self, other: impl Into<Self>) -> bool {
        self == other.into()
    }

    pub fn base_eq(self, other: impl Into<Self>) -> bool {
        unsafe { crate::bindings::BASE_EQ(self, other.into()) }
    }
}

impl LispObject {
    pub fn get_type(self) -> Lisp_Type {
        let raw = self.to_C_unsigned();
        let res = (if USE_LSB_TAG {
            raw & (!VALMASK as EmacsUint)
        } else {
            raw >> Lisp_Bits::VALBITS
        }) as u32;
        unsafe { mem::transmute(res) }
    }

    pub fn tag_ptr<T>(external: ExternalPtr<T>, ty: Lisp_Type) -> Self {
        let raw = external.as_ptr() as intptr_t;
        let res = if USE_LSB_TAG {
            let ptr = raw as intptr_t;
            let tag = ty as intptr_t;
            (ptr + tag) as EmacsInt
        } else {
            let ptr = raw as EmacsUint as uintptr_t;
            let tag = ty as EmacsUint as uintptr_t;
            ((tag << Lisp_Bits::VALBITS) + ptr) as EmacsInt
        };

        Self::from_C(res)
    }

    pub fn get_untaggedptr(self) -> *mut c_void {
        (self.to_C() & VALMASK) as intptr_t as *mut c_void
    }
}

// ExternalPtr

#[repr(transparent)]
pub struct ExternalPtr<T>(NonNull<T>);

impl<T> Copy for ExternalPtr<T> {}

// Derive fails for this type so do it manually
impl<T> Clone for ExternalPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ExternalPtr<T> {
    // pub const fn null() -> Self {
    //     Self(None)
    // }

    pub const fn new(p: *mut T) -> Option<Self> {
        let nonnull = NonNull::new(p);
        match nonnull {
            Some(nonnull) => Some(Self(nonnull)),
            None => None,
        }
    }

    pub const fn new_unchecked(p: *mut T) -> Self {
        let nonnull = unsafe { NonNull::new_unchecked(p) };
        Self(nonnull)
    }

    pub fn is_null(self) -> bool {
        false
    }

    pub const fn as_ptr(self) -> *const T {
        self.0.as_ptr()
    }

    pub fn as_mut(&mut self) -> *mut T {
        unsafe { self.0.as_mut() }
    }

    pub fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        let ptr: Option<*mut T> = unsafe { ptr.as_ref().map(|p| mem::transmute(p)) };
        ptr.and_then(|p| NonNull::new(p)).map(|p| Self(p))
    }

    pub fn cast<U>(mut self) -> ExternalPtr<U> {
        ExternalPtr::<U>(self.0.cast::<U>())
    }
}

impl<T> Deref for ExternalPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T> DerefMut for ExternalPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

// impl<T> From<*mut T> for ExternalPtr<T> {
//     fn from(o: *mut T) -> Option<Self> {
//         Self::new(o)
//     }
// }

impl<T> PartialEq for ExternalPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl<T> PartialOrd for ExternalPtr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.as_ptr().cmp(&other.as_ptr()))
    }
}

/// Copies a Rust str into a new Lisp string
impl<'a> From<&'a str> for LispObject {
    fn from(s: &str) -> Self {
        let cs = CString::new(s).unwrap();
        unsafe { build_string(cs.as_ptr()) }
    }
}

// Lisp_Subr support

pub type LispSubrRef = ExternalPtr<Aligned_Lisp_Subr>;

/// Used to denote functions that have no limit on the maximum number
/// of arguments.
pub const MANY: i16 = -2;

impl LispObject {
    pub fn is_user_ptr(self) -> bool {
        unsafe { USER_PTRP(self) }
    }

    pub unsafe fn as_userdata_ref<T>(&self) -> &T {
        if self.is_user_ptr() {
            let p = XUSER_PTR(*self);
            &(*((*p).p as *const T))
        } else {
            wrong_type!(Quser_ptrp, *self);
        }
    }
}

// extra Rust types conversation
impl From<LispObject> for i32 {
    fn from(o: LispObject) -> Self {
        let val: EmacsInt = o.into();
        val as i32
    }
}

impl From<LispObject> for u32 {
    fn from(o: LispObject) -> Self {
        let val: EmacsInt = o.into();
        val as u32
    }
}

impl From<u32> for LispObject {
    fn from(v: u32) -> Self {
        Self::from_fixnum(EmacsInt::from(v))
    }
}

impl From<i16> for LispObject {
    fn from(v: i16) -> Self {
        Self::from_fixnum(EmacsInt::from(v))
    }
}

impl From<u16> for LispObject {
    fn from(v: u16) -> Self {
        Self::from_fixnum(EmacsInt::from(v))
    }
}

impl From<String> for LispObject {
    fn from(s: String) -> Self {
        let len = s.len();
        let c_content = CString::new(s)
            .map_err(|e| e.to_string())
            .expect("Failed to create string for intern function call");
        unsafe { make_string_from_utf8(c_content.as_ptr(), len.try_into().unwrap()) }
    }
}

impl From<LispObject> for String {
    fn from(val: LispObject) -> Self {
        let val = val.as_symbol_or_string();
        let string_ref: LispStringRef = val.into();
        string_ref.to_utf8()
    }
}
