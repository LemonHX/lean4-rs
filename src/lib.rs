#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]

pub mod closure;
pub mod string;

pub use lean4_macro::Lean4;

pub use lean4_sys;
use lean4_sys::{
    b_lean_obj_arg, lean_alloc_external, lean_external_class, lean_get_external_data, lean_object,
    lean_register_external_class,
};

/// you must have #[repr(transparent)] or #[repr(C)] on your struct
/// and #[no_mangle] on your static mut lean4_object_*
/// or else you will get a **segfault**
trait Lean4Object
where
    Self: Sized,
{
    /// remember to inline this function
    fn get_registed_class() -> &'static mut *mut lean_external_class;

    /// lean4 will call this after its internal RC is dropped
    unsafe extern "C" fn finalize(s: *mut std::ffi::c_void) {
        let bx = Box::from_raw(s as *mut Self);
        drop(bx);
    }
    /// lean4 will call this when contains nested lean objects
    unsafe extern "C" fn foreach(_: *mut std::ffi::c_void, _: b_lean_obj_arg) {}

    #[inline(always)]
    fn into_lean_object_ptr(self) -> *mut lean_object {
        unsafe {
            if Self::get_registed_class().is_null() {
                *Self::get_registed_class() =
                    lean_register_external_class(Some(Self::finalize), Some(Self::foreach))
            }
            let boxed_self = Box::new(self);
            let leaked = Box::leak(boxed_self);
            let leaked_ptr_c_void = leaked as *mut _ as *mut std::ffi::c_void;
            lean_alloc_external(*Self::get_registed_class(), leaked_ptr_c_void)
        }
    }

    #[inline(always)]
    fn from_lean_object_ptr(ptr: *mut lean_object) -> &'static mut Self {
        unsafe {
            let s = lean_get_external_data(ptr) as *mut Self;
            assert!(s != std::ptr::null_mut());
            &mut *s
        }
    }
}
