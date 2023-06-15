use lean4_sys::*;

#[derive(Clone, Copy, Debug)]
pub struct Closure {
    ptr: *mut lean_object,
}

impl Closure {
    pub fn new(ptr: *mut lean_object) -> Self {
        Self { ptr }
    }
    pub fn get_func_ptr(self) -> *mut std::ffi::c_void {
        unsafe { lean_closure_fun(self.ptr) }
    }
    pub fn get_arity(self) -> usize {
        unsafe { lean_closure_arity(self.ptr) as usize }
    }
    pub fn get_num_fixed(self) -> usize {
        unsafe { lean_closure_num_fixed(self.ptr) as usize }
    }
    pub fn get_args_ptr(self) -> &'static mut [*mut lean_object] {
        let pptr = unsafe { lean_closure_arg_cptr(self.ptr) };
        let arity = self.get_arity();
        unsafe { std::slice::from_raw_parts_mut(pptr, arity) }
    }
    /// `f` is the function pointer
    /// `arity` is the number of arguments expected by `f`
    /// `num_fixed` is the number of arguments that have been already fixed
    pub fn alloc(f: *mut std::ffi::c_void, arity: usize, num_fixed: usize) -> *mut lean_object {
        unsafe { lean_alloc_closure(f, arity as u32, num_fixed as u32) }
    }

    pub fn get_arg(self, i: usize) -> *mut lean_object {
        unsafe { lean_closure_get(self.ptr, i as u32) }
    }

    pub fn set_arg(self, i: usize, arg: *mut lean_object) {
        unsafe { lean_closure_set(self.ptr, i as u32, arg) }
    }
}

// for better user experience
// ---------- call ----------
// 1
impl std::ops::FnOnce<(*mut lean_object,)> for Closure {
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(self, args: (*mut lean_object,)) -> Self::Output {
        unsafe { lean_apply_1(self.ptr, args.0) }
    }
}

// 2
impl std::ops::FnOnce<(*mut lean_object, *mut lean_object)> for Closure {
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (*mut lean_object, *mut lean_object),
    ) -> Self::Output {
        unsafe { lean_apply_2(self.ptr, args.0, args.1) }
    }
}

// 3
impl std::ops::FnOnce<(*mut lean_object, *mut lean_object, *mut lean_object)> for Closure {
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (*mut lean_object, *mut lean_object, *mut lean_object),
    ) -> Self::Output {
        unsafe { lean_apply_3(self.ptr, args.0, args.1, args.2) }
    }
}

// 4
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe { lean_apply_4(self.ptr, args.0, args.1, args.2, args.3) }
    }
}

// 5
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe { lean_apply_5(self.ptr, args.0, args.1, args.2, args.3, args.4) }
    }
}

// 6
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe { lean_apply_6(self.ptr, args.0, args.1, args.2, args.3, args.4, args.5) }
    }
}

// 7
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_7(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6,
            )
        }
    }
}

// 8
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_8(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7,
            )
        }
    }
}

// 9
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_9(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
            )
        }
    }
}

// 10
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_10(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9,
            )
        }
    }
}

// 11
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_11(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9, args.10,
            )
        }
    }
}

// 12
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_12(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9, args.10, args.11,
            )
        }
    }
}

// 13
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_13(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9, args.10, args.11, args.12,
            )
        }
    }
}

// 14
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_14(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9, args.10, args.11, args.12, args.13,
            )
        }
    }
}

// 15
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_15(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9, args.10, args.11, args.12, args.13, args.14,
            )
        }
    }
}

// 16
impl
    std::ops::FnOnce<(
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
        *mut lean_object,
    )> for Closure
{
    type Output = *mut lean_object;

    extern "rust-call" fn call_once(
        self,
        args: (
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ),
    ) -> Self::Output {
        unsafe {
            lean_apply_16(
                self.ptr, args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
                args.9, args.10, args.11, args.12, args.13, args.14, args.15,
            )
        }
    }
}

//---------- from ----------
impl From<*mut lean_object> for Closure {
    fn from(ptr: *mut lean_object) -> Self {
        Self::new(ptr)
    }
}

// 1
impl From<extern "C" fn(*mut lean_object) -> *mut lean_object> for Closure {
    fn from(f: extern "C" fn(*mut lean_object) -> *mut lean_object) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 1, 0))
    }
}

// 2
impl From<extern "C" fn(*mut lean_object, *mut lean_object) -> *mut lean_object> for Closure {
    fn from(f: extern "C" fn(*mut lean_object, *mut lean_object) -> *mut lean_object) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 2, 0))
    }
}

// 3
impl From<extern "C" fn(*mut lean_object, *mut lean_object, *mut lean_object) -> *mut lean_object>
    for Closure
{
    fn from(
        f: extern "C" fn(*mut lean_object, *mut lean_object, *mut lean_object) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 3, 0))
    }
}

// 4
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 4, 0))
    }
}

// 5
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 5, 0))
    }
}

// 6
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 6, 0))
    }
}

// 7
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 7, 0))
    }
}

// 8
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 8, 0))
    }
}

// 9
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 9, 0))
    }
}

// 10
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 10, 0))
    }
}

// 11
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 11, 0))
    }
}

// 12
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 12, 0))
    }
}

// 13
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 13, 0))
    }
}

// 14
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 14, 0))
    }
}

// 15
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 15, 0))
    }
}

// 16
impl
    From<
        extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    > for Closure
{
    fn from(
        f: extern "C" fn(
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
            *mut lean_object,
        ) -> *mut lean_object,
    ) -> Self {
        Self::new(Self::alloc(f as *mut std::ffi::c_void, 16, 0))
    }
}
