use crate::script::{ScriptOpcode, ScriptState};
use crate::Engine;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Obj;
}

pub struct ObjOps;

impl ObjOps {
    #[inline(always)]
    pub fn new() -> ObjOps {
        return ObjOps;
    }

    #[inline(always)]
    #[rustfmt::skip]
    pub fn push(
        &self,
        engine: &Engine,
        state: &mut ScriptState,
        code: ScriptOpcode,
    ) -> Result<(), String> {
        return match code {
            ScriptOpcode::ObjAdd => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjAddAll => self.obj_addall(engine, state),
            ScriptOpcode::ObjCoord => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjCount => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjDel => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjName => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjParam => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjTakeItem => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjType => Err(String::from("Unimplemented!")),
            ScriptOpcode::ObjFind => Err(String::from("Unimplemented!")),
            _ => Err(format!("Unrecognised server ops code: {:?}", code)),
        }
    }

    #[inline(always)]
    fn obj_addall(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let duration: i32 = state.pop_int();
        let count: i32 = state.pop_int();
        let id: i32 = state.pop_int();
        let coord: i32 = state.pop_int();

        if id == -1 || count == -1 {
            return Ok(());
        }
        if let Some(obj) = engine.obj_addall(coord, id, count, duration) {
            state.set_active_obj(obj);
            state.pointer_add(ScriptState::ACTIVE_OBJ[state.int_operand()]);
            return Ok(());
        }
        return Ok(());
    }
}
