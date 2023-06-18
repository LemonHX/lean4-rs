use lean4_sys::*;

use crate::Lean4Obj;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Closure {
    ptr: Lean4Obj,
}

impl Closure {
    pub fn new(ptr: Lean4Obj) -> Self {
        Self { ptr }
    }
    pub fn get_func_ptr(self) -> *mut std::ffi::c_void {
        unsafe { lean_closure_fun(self.ptr.into()) }
    }
    pub fn get_arity(self) -> usize {
        unsafe { lean_closure_arity(self.ptr.into()) as usize }
    }
    pub fn get_num_fixed(self) -> usize {
        unsafe { lean_closure_num_fixed(self.ptr.into()) as usize }
    }
    pub fn get_args_ptr(self) -> &'static mut [Lean4Obj] {
        let pptr = unsafe { lean_closure_arg_cptr(self.ptr.into()) };
        let arity = self.get_arity();
        unsafe { std::slice::from_raw_parts_mut(std::mem::transmute(pptr), arity) }
    }
    /// `f` is the function pointer
    /// `arity` is the number of arguments expected by `f`
    /// `num_fixed` is the number of arguments that have been already fixed
    pub fn alloc(f: *mut std::ffi::c_void, arity: usize, num_fixed: usize) -> Lean4Obj {
        unsafe { lean_alloc_closure(f, arity as u32, num_fixed as u32).into() }
    }

    pub fn get_arg(self, i: usize) -> Lean4Obj {
        unsafe { lean_closure_get(self.ptr.into(), i as u32).into() }
    }

    pub fn set_arg(self, i: usize, arg: Lean4Obj) {
        unsafe { lean_closure_set(self.ptr.into(), i as u32, arg.into()) }
    }
}

// for better user experience
// ---------- call ----------
// 1
impl std::ops::FnOnce<(Lean4Obj,)> for Closure {
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(self, args: (Lean4Obj,)) -> Self::Output {
        unsafe { lean_apply_1(self.ptr.into(), args.0.into()).into() }
    }
}

// 2
impl std::ops::FnOnce<(Lean4Obj, Lean4Obj)> for Closure {
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(self, args: (Lean4Obj, Lean4Obj)) -> Self::Output {
        unsafe { lean_apply_2(self.ptr.into(), args.0.into(), args.1.into()).into() }
    }
}

// 3
impl std::ops::FnOnce<(Lean4Obj, Lean4Obj, Lean4Obj)> for Closure {
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(self, args: (Lean4Obj, Lean4Obj, Lean4Obj)) -> Self::Output {
        unsafe { lean_apply_3(self.ptr.into(), args.0.into(), args.1.into(), args.2.into()).into() }
    }
}

// 4
impl std::ops::FnOnce<(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj)> for Closure {
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj),
    ) -> Self::Output {
        unsafe {
            lean_apply_4(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
            )
            .into()
        }
    }
}

// 5
impl std::ops::FnOnce<(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj)> for Closure {
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj),
    ) -> Self::Output {
        unsafe {
            lean_apply_5(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
            )
            .into()
        }
    }
}

// 6
impl std::ops::FnOnce<(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj)> for Closure {
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj),
    ) -> Self::Output {
        unsafe {
            lean_apply_6(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
            )
            .into()
        }
    }
}

// 7
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_7(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
            )
            .into()
        }
    }
}

// 8
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_8(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
            )
            .into()
        }
    }
}

// 9
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_9(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
            )
            .into()
        }
    }
}

// 10
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_10(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
            )
            .into()
        }
    }
}

// 11
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_11(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
                args.10.into(),
            )
            .into()
        }
    }
}

// 12
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_12(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
                args.10.into(),
                args.11.into(),
            )
            .into()
        }
    }
}

// 13
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_13(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
                args.10.into(),
                args.11.into(),
                args.12.into(),
            )
            .into()
        }
    }
}

// 14
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_14(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
                args.10.into(),
                args.11.into(),
                args.12.into(),
                args.13.into(),
            )
            .into()
        }
    }
}

// 15
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_15(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
                args.10.into(),
                args.11.into(),
                args.12.into(),
                args.13.into(),
                args.14.into(),
            )
            .into()
        }
    }
}

// 16
impl
    std::ops::FnOnce<(
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
        Lean4Obj,
    )> for Closure
{
    type Output = Lean4Obj;

    extern "rust-call" fn call_once(
        self,
        args: (
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_16(
                self.ptr.into(),
                args.0.into(),
                args.1.into(),
                args.2.into(),
                args.3.into(),
                args.4.into(),
                args.5.into(),
                args.6.into(),
                args.7.into(),
                args.8.into(),
                args.9.into(),
                args.10.into(),
                args.11.into(),
                args.12.into(),
                args.13.into(),
                args.14.into(),
                args.15.into(),
            )
            .into()
        }
    }
}

//---------- from ----------
impl From<Lean4Obj> for Closure {
    fn from(ptr: Lean4Obj) -> Self {
        Self::new(ptr)
    }
}

// 1
impl From<extern "C" fn(Lean4Obj) -> Lean4Obj> for Closure {
    fn from(f: extern "C" fn(Lean4Obj) -> Lean4Obj) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 1, 0))
    }
}

// 2
impl From<extern "C" fn(Lean4Obj, Lean4Obj) -> Lean4Obj> for Closure {
    fn from(f: extern "C" fn(Lean4Obj, Lean4Obj) -> Lean4Obj) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 2, 0))
    }
}

// 3
impl From<extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj> for Closure {
    fn from(f: extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 3, 0))
    }
}

// 4
impl From<extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj> for Closure {
    fn from(f: extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 4, 0))
    }
}

// 5
impl From<extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj> for Closure {
    fn from(
        f: extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 5, 0))
    }
}

// 6
impl From<extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj>
    for Closure
{
    fn from(
        f: extern "C" fn(Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj, Lean4Obj) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 6, 0))
    }
}

// 7
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 7, 0))
    }
}

// 8
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 8, 0))
    }
}

// 9
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 9, 0))
    }
}

// 10
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 10, 0))
    }
}

// 11
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 11, 0))
    }
}

// 12
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 12, 0))
    }
}

// 13
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 13, 0))
    }
}

// 14
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 14, 0))
    }
}

// 15
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 15, 0))
    }
}

// 16
impl
    From<
        extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
            Lean4Obj,
        ) -> Lean4Obj,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 16, 0))
    }
}
