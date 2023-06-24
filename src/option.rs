use lean4_sys::{lean_alloc_ctor, lean_ctor_get, lean_ctor_set, lean_obj_tag};

use crate::Lean4Obj;

#[repr(transparent)]
pub struct LOption {
    pub ptr: Lean4Obj,
}

impl LOption {
    pub fn none() -> Self {
        unsafe {
            let mem = lean_alloc_ctor(0, 0, 0);
            LOption { ptr: Lean4Obj(mem) }
        }
    }

    pub fn some(obj: Lean4Obj) -> Self {
        unsafe {
            let mem = lean_alloc_ctor(1, 1, 0);
            lean_ctor_set(mem, 0, obj.0);
            LOption { ptr: Lean4Obj(mem) }
        }
    }

    pub fn is_some(&self) -> bool {
        unsafe {
            let tag = lean_obj_tag(self.ptr.0);
            tag == 1
        }
    }

    pub fn is_none(&self) -> bool {
        unsafe {
            let tag = lean_obj_tag(self.ptr.0);
            tag == 0
        }
    }
}

impl From<Lean4Obj> for LOption {
    fn from(obj: Lean4Obj) -> Self {
        Self { ptr: obj }
    }
}

impl Into<Lean4Obj> for LOption {
    fn into(self) -> Lean4Obj {
        self.ptr
    }
}

impl From<Option<Lean4Obj>> for LOption {
    fn from(obj: Option<Lean4Obj>) -> Self {
        match obj {
            Some(obj) => Self::some(obj),
            None => Self::none(),
        }
    }
}

impl Into<Option<Lean4Obj>> for LOption {
    fn into(self) -> Option<Lean4Obj> {
        if self.is_some() {
            Some(unsafe { Lean4Obj(lean_ctor_get(self.ptr.0, 0)) })
        } else {
            None
        }
    }
}
