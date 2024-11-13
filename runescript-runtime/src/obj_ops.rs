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
) {
    return match code {
        ScriptOpcode::ObjAdd => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjAddAll => obj_addall(engine, state),
        ScriptOpcode::ObjCoord => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjDel => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjName => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjParam => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjTakeItem => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjType => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjFind => state.abort(format!("Unimplemented! {:?}", code)),
        _ => state.abort(format!("Unrecognised obj ops code: {:?}", code)),
    }
}

#[inline(always)]
fn obj_addall(engine: &Engine, state: &mut ScriptState) {
    let duration: i32 = state.pop_int();
    let count: i32 = state.pop_int();
    let id: i32 = state.pop_int();
    let coord: i32 = state.pop_int();

    let duration: i32 = match engine.check_duration(duration) {
        Ok(duration) => duration,
        Err(err) => return state.abort(err),
    };

    let coord: CoordGrid = match engine.check_coord(coord) {
        Ok(coord) => coord,
        Err(err) => return state.abort(err),
    };

    if id == -1 || count == -1 {
        return;
    }

    if let Err(err) = engine.check_obj_stack(count) {
        return state.abort(err);
    }

    let obj_type: ObjType = match engine.check_obj(id) {
        Ok(obj_type) => obj_type,
        Err(err) => return state.abort(err),
    };

    if obj_type.dummyitem() != 0 {
        return state.abort(format!(
            "Attempted to add dummy item: {}",
            obj_type.debugname()
        ));
    }

    if obj_type.members() && !engine.map_members() {
        return;
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
}
