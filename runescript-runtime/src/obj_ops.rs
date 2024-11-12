use crate::coord_grid::CoordGrid;
use crate::script::{ScriptOpcode, ScriptState};
use crate::Engine;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    pub type Obj;
    pub type ObjType;

    #[wasm_bindgen(method, getter = debugname)]
    pub fn debugname(this: &ObjType) -> String;

    #[wasm_bindgen(method, getter = dummyitem)]
    pub fn dummyitem(this: &ObjType) -> i32;

    #[wasm_bindgen(method, getter = members)]
    pub fn members(this: &ObjType) -> bool;

    #[wasm_bindgen(method, getter = stackable)]
    pub fn stackable(this: &ObjType) -> bool;
}

#[inline(always)]
#[rustfmt::skip]
pub fn perform_obj_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    return match code {
        ScriptOpcode::ObjAdd => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjAddAll => obj_addall(engine, state),
        ScriptOpcode::ObjCoord => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjCount => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjDel => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjName => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjParam => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjTakeItem => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjType => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjFind => Err(format!("Unimplemented! {:?}", code)),
        _ => Err(format!("Unrecognised obj ops code: {:?}", code)),
    }
}

#[inline(always)]
fn obj_addall(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let duration: i32 = engine.check_duration(state.pop_int())?;
    let count: i32 = state.pop_int();
    let id: i32 = state.pop_int();
    let coord: CoordGrid = engine.check_coord(state.pop_int())?;

    if id == -1 || count == -1 {
        return Ok(());
    }

    engine.check_obj_stack(count)?;
    let obj_type: ObjType = engine.check_obj(id)?;

    if obj_type.dummyitem() != 0 {
        return Err(format!(
            "Attempted to add dummy item: {}",
            obj_type.debugname()
        ));
    }

    if obj_type.members() && !engine.map_members() {
        return Ok(());
    }

    let obj: Obj = engine.obj_addall(
        coord.x(),
        coord.y(),
        coord.z(),
        id,
        count,
        duration,
        obj_type.stackable(),
    );
    state.set_active_obj(obj);
    state.pointer_add(ScriptState::ACTIVE_OBJ[state.int_operand()]);
    return Ok(());
}
