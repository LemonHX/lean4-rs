use futures::future::select_all;
use std::future::Future;
use tokio::sync::oneshot::{Receiver, Sender};

use crate::Lean4Object;
use lean4_macro::Lean4;
use lean4_sys::{lean_box, lean_dec, lean_external_class};

use crate::{array::LArray, closure::Closure, io::LIO, option::LOption, Lean4Obj};

extern "C" {
    fn lean_initialize_runtime_module();
}

lazy_static::lazy_static! {
    /// yeah, I think let process to collect this runtime is perfectly fine
    /// background process? it will be killed by OS for sure
    pub static ref GLOBAL_TOKIO_RT : tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(|| {
            unsafe {
                // initialize lean runtime on each thread
                lean_initialize_runtime_module();
            }
        })
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");
}

#[repr(C)]
#[derive(Lean4)]
pub struct TokioTask {
    receiver: Receiver<Lean4Obj>,
}

impl Drop for TokioTask {
    fn drop(&mut self) {
        println!("one future drops");
    }
}

impl Future for TokioTask {
    type Output = Result<Lean4Obj, String>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let self_ptr = self.get_mut() as *mut TokioTask;
        let receiver = unsafe { &mut (*self_ptr).receiver };
        let receiver = std::pin::Pin::new(receiver);
        let res = receiver.poll(cx).map(|x| x.map_err(|e| e.to_string()));

        // free self pointer
        let s = unsafe { Box::from_raw(self_ptr) };
        drop(s);
        res
    }
}

impl TokioTask {
    pub fn new_native() -> (Lean4Obj, Sender<Lean4Obj>) {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let r = Self { receiver: receiver };
        let r = r.into_lean_object_ptr();
        (r, sender)
    }

    pub fn new_lean(result: Lean4Obj) -> Lean4Obj {
        let (r, cx) = Self::new_native();
        cx.send(result).unwrap();
        r
    }
}

#[no_mangle]
pub extern "C" fn tokio_task_lean_closure(_: Lean4Obj, c: Lean4Obj) -> Lean4Obj {
    TokioTask::new_lean(Closure::from(c)(unsafe { Lean4Obj(lean_box(0)) }))
}

#[no_mangle]
pub extern "C" fn tokio_task_get(lean_self: Lean4Obj) -> Lean4Obj {
    let task = TokioTask::from_lean_object_ptr(lean_self);
    GLOBAL_TOKIO_RT.block_on(async move {
        let res = task.await.unwrap();
        LIO::Ok(res).into()
    })
}

#[no_mangle]
pub extern "C" fn tokio_task_try_get(lean_self: Lean4Obj) -> Lean4Obj {
    println!("🦀: tokio_task_try_get");
    let task = TokioTask::from_lean_object_ptr(lean_self);
    let receiver = &mut task.receiver;

    let ret = receiver.try_recv().ok();
    println!("🦀: tokio_task_try_get {:?}", ret);
    match ret {
        Some(res) => LOption::some(res).into(),
        None => LOption::none().into(),
    }
}

#[no_mangle]
pub extern "C" fn tokio_task_bind(_: Lean4Obj, lean_self: Lean4Obj, lean_f: Lean4Obj) -> Lean4Obj {
    let task = TokioTask::from_lean_object_ptr(lean_self);
    let (r, sender) = TokioTask::new_native();
    GLOBAL_TOKIO_RT.spawn(async move {
        let res = task.await.unwrap();
        let f = Closure::from(lean_f);
        let r = f(res);
        let r = TokioTask::from_lean_object_ptr(r);
        let r = r.await.unwrap();
        sender.send(r).unwrap();
    });
    r
}

#[no_mangle]
pub extern "C" fn tokio_task_map(_: Lean4Obj, lean_f: Lean4Obj, lean_task_a: Lean4Obj) -> Lean4Obj {
    let taska = TokioTask::from_lean_object_ptr(lean_task_a);
    let (r, sender) = TokioTask::new_native();
    GLOBAL_TOKIO_RT.spawn(async move {
        let res = taska.await.unwrap();
        let f = Closure::from(lean_f);
        let r = f(res);
        sender.send(r).unwrap();
    });
    r
}

#[no_mangle]
pub extern "C" fn tokio_task_seq_left(
    _: Lean4Obj,
    _: Lean4Obj,
    lean_task_a: Lean4Obj,
    lean_task_b: Lean4Obj,
) -> Lean4Obj {
    let taska = TokioTask::from_lean_object_ptr(lean_task_a);
    let cls = Closure::from(lean_task_b);
    cls(unsafe { Lean4Obj(lean_box(0)) });
    let (r, sender) = TokioTask::new_native();
    GLOBAL_TOKIO_RT.spawn(async move {
        let res = taska.await.unwrap();
        sender.send(res).unwrap();
    });
    r
}

#[no_mangle]
pub extern "C" fn tokio_task_seq_right(
    _1: Lean4Obj,
    _2: Lean4Obj,
    _a: Lean4Obj,
    lean_task_b: Lean4Obj,
) -> Lean4Obj {
    let cls = Closure::from(lean_task_b);
    let taskb = cls(unsafe { Lean4Obj(lean_box(0)) });
    let taskb = TokioTask::from_lean_object_ptr(taskb);
    let (r, sender) = TokioTask::new_native();
    GLOBAL_TOKIO_RT.spawn(async move {
        let res = taskb.await.unwrap();
        sender.send(res).unwrap();
    });
    r
}

#[no_mangle]
pub extern "C" fn tokio_task_select_all(_: Lean4Obj, lean_tasks: Lean4Obj) -> Lean4Obj {
    let larray = LArray::from(lean_tasks);
    let (r, sender) = TokioTask::new_native();
    GLOBAL_TOKIO_RT.block_on(async move {
        let iter = larray.as_slice_mut().iter().map(|x| {
            let task = TokioTask::from_lean_object_ptr(*x);
            let receiver = &mut task.receiver;
            receiver
        });
        let (res, _idx, _rest) = select_all(iter).await;
        let res = res.unwrap();
        unsafe { lean_dec(lean_tasks.0) };
        sender.send(res).unwrap();
    });
    r
}
