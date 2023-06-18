//! lean4 object are packed struct in C
//! so we can directly cast it to a rust #[repr(pack)] struct

use lean4_sys::lean_ctor_obj_cptr;

use crate::Lean4Obj;

impl Lean4Obj {
    /// only works for structure in Lean4
    pub fn into_packed_struct<T: Sized>(self) -> &'static mut T {
        unsafe { &mut *(lean_ctor_obj_cptr(self.0) as *mut T) }
    }
}
