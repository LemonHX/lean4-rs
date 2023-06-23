use lean4_sys::{lean_box, lean_io_result_mk_error, lean_io_result_mk_ok};

use crate::Lean4Obj;

pub enum LIO {
    Ok(Lean4Obj),
    Err(Lean4Obj),
}

impl LIO {
    pub fn ok_unit() -> Self {
        LIO::Ok(unsafe { Lean4Obj(lean_box(0)) })
    }
}

impl Into<Lean4Obj> for LIO {
    fn into(self) -> Lean4Obj {
        match self {
            LIO::Ok(obj) => unsafe { Lean4Obj(lean_io_result_mk_ok(obj.into())) },
            LIO::Err(obj) => unsafe { Lean4Obj(lean_io_result_mk_error(obj.into())) },
        }
    }
}
