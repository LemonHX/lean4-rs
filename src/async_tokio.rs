use std::sync::atomic::AtomicUsize;

use futures::future::select_all;
use lean4_sys::{lean_alloc_object, lean_box, lean_dec, lean_object};
use scc::HashMap;
use tokio::sync::oneshot::{Receiver, Sender};

use crate::{array::LArray, closure::Closure, io::LIO, option::LOption, Lean4Obj};

extern "C" {
    fn lean_initialize_runtime_module();
}

pub struct System {
    all_tasks: HashMap<usize, &'static TokioTask>,
}

impl System {
    fn new() -> Self {
        Self {
            all_tasks: HashMap::new(),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_SYSTEM : System = System::new();
    pub static ref GLOBAL_TOKIO_RT : tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(|| {
            unsafe {
                lean_initialize_runtime_module();
            }
        })
        .enable_all()
        .build()
        .unwrap();
}

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[repr(C)]
pub struct TokioTask {
    header: lean_object,
    id: usize,
    receiver: Receiver<Lean4Obj>,
}

impl std::fmt::Debug for TokioTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TokioTask {{ id: {} }}", self.id)
    }
}

impl TokioTask {
    pub fn from_lean_object_ptr(ptr: Lean4Obj) -> &'static mut Self {
        unsafe {
            let ptr = ptr.0 as *mut TokioTask;
            &mut *ptr
        }
    }

    pub fn new_raw() -> &'static mut Self {
        unsafe {
            let mem = lean_alloc_object(std::mem::size_of::<TokioTask>());
            let ptr = mem as *mut TokioTask;
            let r = &mut *ptr;
            r.header.set_m_tag(114);
            r
        }
    }

    pub fn new_native() -> (&'static mut Self, Sender<Lean4Obj>) {
        let id = GLOBAL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let r = Self::new_raw();
        let (sender, receiver) = tokio::sync::oneshot::channel();
        r.id = id;
        unsafe {
            std::ptr::write(&mut r.receiver, receiver);
        }
        GLOBAL_SYSTEM
            .all_tasks
            .insert(id, unsafe { &mut *(r as *mut TokioTask) })
            .unwrap();
        (r, sender)
    }

    pub fn new_lean(result: Lean4Obj) -> &'static mut Self {
        let r: &mut TokioTask = Self::new_raw();
        let (cx, rx) = tokio::sync::oneshot::channel();
        cx.send(result).unwrap();
        r.receiver = rx;
        r
    }

    pub fn into_lean_object_ptr(&'static mut self) -> Lean4Obj {
        let ptr = self as *mut TokioTask;
        Lean4Obj(ptr as *mut lean_object)
    }
}

#[no_mangle]
pub extern "C" fn tokio_task_mk(c: Lean4Obj) -> Lean4Obj {
    TokioTask::new_lean(c).into_lean_object_ptr()
}

#[no_mangle]
pub extern "C" fn tokio_task_get(lean_self: Lean4Obj) -> Lean4Obj {
    let task = TokioTask::from_lean_object_ptr(lean_self);
    GLOBAL_TOKIO_RT.block_on(async move {
        let receiver = &mut task.receiver;
        let res = receiver.await.unwrap();
        LIO::Ok(res).into()
    })
}

#[no_mangle]
pub extern "C" fn tokio_task_try_get(lean_self: Lean4Obj) -> Lean4Obj {
    println!("🦀: tokio_task_try_get");
    let task = TokioTask::from_lean_object_ptr(lean_self);
    let receiver = &mut task.receiver;

    std::thread::sleep(std::time::Duration::from_millis(1000));

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
        let receiver = &mut task.receiver;
        let res = receiver.await.unwrap();
        let f = Closure::from(lean_f);
        let r = f(res);
        let r = TokioTask::from_lean_object_ptr(r);
        let receiver = &mut r.receiver;
        let r = receiver.await.unwrap();
        sender.send(r).unwrap();
    });
    r.into_lean_object_ptr()
}

#[no_mangle]
pub extern "C" fn tokio_task_map(_: Lean4Obj, lean_f: Lean4Obj, lean_task_a: Lean4Obj) -> Lean4Obj {
    let taska = TokioTask::from_lean_object_ptr(lean_task_a);
    let (r, sender) = TokioTask::new_native();
    GLOBAL_TOKIO_RT.spawn(async move {
        let receiver = &mut taska.receiver;
        let res = receiver.await.unwrap();
        let f = Closure::from(lean_f);
        let r = f(res);
        sender.send(r).unwrap();
    });
    r.into_lean_object_ptr()
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
        let receiver = &mut taska.receiver;
        let res = receiver.await.unwrap();
        sender.send(res).unwrap();
    });
    r.into_lean_object_ptr()
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
        let receiver = &mut taskb.receiver;
        let res = receiver.await.unwrap();
        sender.send(res).unwrap();
    });
    r.into_lean_object_ptr()
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
        let (res, _, _) = select_all(iter).await;
        let res = res.unwrap();
        unsafe { lean_dec(lean_tasks.0) };
        sender.send(res).unwrap();
    });
    r.into_lean_object_ptr()
}
