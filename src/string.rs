use std::fmt::{Debug, Display};

use lean4_sys::*;

/// the ABI of lean4 string is equivalent to C's char*
/// with utf8 encoding by ending with a null byte
#[derive(Clone, Copy)]
pub struct LString {
    ptr: *mut lean_object,
}

impl LString {
    pub fn as_ptr(&self) -> *mut lean_object {
        self.ptr
    }
    pub fn as_str(&self) -> &str {
        self.into()
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
    pub fn new(s: *mut lean_object) -> Self {
        LString { ptr: s }
    }
}

impl Into<LString> for *mut lean_object {
    fn into(self) -> LString {
        LString { ptr: self }
    }
}

impl From<String> for LString {
    fn from(s: String) -> Self {
        unsafe {
            let s = std::ffi::CString::new(s).unwrap();
            let s = s.as_ptr();
            let s = lean_mk_string(s);
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
            LString { ptr: s }
        }
    }
}

impl Into<&str> for LString {
    fn into(self) -> &'static str {
        unsafe {
            let s = lean_string_cstr(self.ptr);
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

impl Into<*mut lean_object> for LString {
    fn into(self) -> *mut lean_object {
        self.ptr
    }
}
