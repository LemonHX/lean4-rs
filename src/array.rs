use lean4_sys::{lean_array_cptr, lean_array_size};

use crate::Lean4Obj;

#[derive(Clone, Copy)]
pub struct LArray {
    pub ptr: Lean4Obj,
}

unsafe impl Send for LArray {}
unsafe impl Sync for LArray {}

impl From<Lean4Obj> for LArray {
    fn from(obj: Lean4Obj) -> Self {
        Self { ptr: obj }
    }
}

impl Into<Lean4Obj> for LArray {
    fn into(self) -> Lean4Obj {
        self.ptr
    }
}

impl Into<&'static mut [Lean4Obj]> for LArray {
    fn into(self) -> &'static mut [Lean4Obj] {
        unsafe {
            let ptr = lean_array_cptr(self.ptr.0);
            let len = lean_array_size(self.ptr.0);
            std::mem::transmute(std::slice::from_raw_parts_mut(ptr, len))
        }
    }
}

impl LArray {
    pub fn len(&self) -> usize {
        unsafe { lean_array_size(self.ptr.0) }
    }
    pub fn as_slice_mut(self) -> &'static mut [Lean4Obj] {
        self.into()
    }
}

impl std::fmt::Debug for LArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &'static mut [Lean4Obj] = self.clone().into();
        write!(f, "lean_array <{:?}>:{:?}", self.ptr, s)
    }
}
