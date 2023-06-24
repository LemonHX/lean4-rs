use std::fmt::{Debug, Display};

use lean4_sys::*;

use crate::Lean4Obj;

/// the ABI of lean4 string is equivalent to C's char*
/// with utf8 encoding by ending with a null byte
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct LString {
    pub ptr: Lean4Obj,
}

impl LString {
    pub fn as_str(&self) -> &str {
        self.into()
    }
}

impl From<Lean4Obj> for LString {
    fn from(obj: Lean4Obj) -> Self {
        LString { ptr: obj.into() }
    }
}

impl Into<Lean4Obj> for LString {
    fn into(self) -> Lean4Obj {
        self.ptr
    }
}

impl Debug for LString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.into();
        write!(f, "lean_string <{:?}>:{:?}", self.ptr, s)
    }
}

impl Display for LString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.into();
        write!(f, "{}", s)
    }
}

impl LString {
    pub fn new(s: Lean4Obj) -> Self {
        LString { ptr: s }
    }
}

impl From<String> for LString {
    fn from(s: String) -> Self {
        unsafe {
            let s = std::ffi::CString::new(s).unwrap();
            let s = s.as_ptr();
            let s = lean_mk_string(s);
            let s = Lean4Obj(s);
            LString { ptr: s }
        }
    }
}

impl From<&str> for LString {
    fn from(s: &str) -> Self {
        unsafe {
            let s = std::ffi::CString::new(s).unwrap();
            let s = s.as_ptr();
            let s = lean_mk_string(s);
            let s = Lean4Obj(s);
            LString { ptr: s }
        }
    }
}

impl Into<&str> for LString {
    fn into(self) -> &'static str {
        unsafe {
            let s = lean_string_cstr(self.ptr.0);
            let s = std::ffi::CStr::from_ptr(s);
            s.to_str().unwrap()
        }
    }
}

impl Into<&str> for &LString {
    fn into(self) -> &'static str {
        (*self).into()
    }
}
