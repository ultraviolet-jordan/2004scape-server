use crate::player_ops::Player;
use crate::script::{ScriptExecutionState, ScriptState};
use crate::Engine;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

pub enum VarValue {
    String(String),
    Int(i32),
}

#[wasm_bindgen]
extern "C" {
    pub type VarPlayerType;

    #[wasm_bindgen(method, getter = id)]
    pub fn id(this: &VarPlayerType) -> u16;

    #[wasm_bindgen(method, getter = debugname)]
    pub fn debugname(this: &VarPlayerType) -> String;

    #[wasm_bindgen(method, getter = protect)]
    pub fn protect(this: &VarPlayerType) -> bool;

    #[wasm_bindgen(method, js_name = isString)]
    pub fn is_string(this: &VarPlayerType) -> bool;
}

#[inline(always)]
pub(crate) fn push_constant_int(state: &mut ScriptState) {
    state.push_int(state.int_operand() as i32);
}

#[inline(always)]
pub(crate) fn push_varp(engine: &Engine, state: &mut ScriptState) {
    let secondary: usize = state.int_operand() >> 16 & 0x1;
    if secondary == 1 && state.get_active_player2().is_null() {
        return state.abort(String::from("No active_player."));
    } else if secondary == 0 && state.get_active_player1().is_null() {
        return state.abort(String::from("No active_player."));
    }

    let varp_type: VarPlayerType = match engine.check_varp((state.int_operand() & 0xffff) as i32) {
        Ok(varp) => varp,
        Err(err) => return state.abort(err),
    };

    let player: &Player = if secondary == 1 {
        state.get_active_player2()
    } else {
        state.get_active_player1()
    };

    if varp_type.is_string() {
        if let Some(str) = player.get_var(varp_type.id()).as_string() {
            state.push_string(str);
            return;
        }
        state.abort(String::from("Expected a string varp value."));
    } else {
        if let Some(num) = player.get_var(varp_type.id()).as_f64() {
            state.push_int(num as i32);
            return;
        }
        state.abort(String::from("Expected a numeric varp value."));
    }
}

#[inline(always)]
pub(crate) fn pop_varp(engine: &Engine, state: &mut ScriptState) {
    let secondary: usize = state.int_operand() >> 16 & 0x1;
    if secondary == 1 && state.get_active_player2().is_null() {
        state.abort(String::from("No secondary active_player."));
        return;
    } else if secondary == 0 && state.get_active_player1().is_null() {
        state.abort(String::from("No active_player."));
        return;
    }

    let varp_type: VarPlayerType = match engine.check_varp((state.int_operand() & 0xffff) as i32) {
        Ok(varp) => varp,
        Err(err) => return state.abort(err),
    };

    if !state.pointer_get(ScriptState::PROTECTED_ACTIVE_PLAYER[secondary]) && varp_type.protect() {
        return state.abort(format!(
            "&{} requires protected access",
            varp_type.debugname()
        ));
    }

    if varp_type.is_string() {
        let value: String = state.pop_string();
        if secondary == 1 {
            state
                .get_active_player2()
                .set_var(varp_type.id(), JsValue::from(value));
        } else {
            state
                .get_active_player1()
                .set_var(varp_type.id(), JsValue::from(value));
        }
    } else {
        let value: i32 = state.pop_int();
        if secondary == 1 {
            state
                .get_active_player2()
                .set_var(varp_type.id(), JsValue::from(value));
        } else {
            state
                .get_active_player1()
                .set_var(varp_type.id(), JsValue::from(value));
        }
    }
}

#[inline(always)]
pub(crate) fn push_constant_string(state: &mut ScriptState) {
    state.push_string(state.string_operand());
}

#[inline(always)]
pub(crate) fn push_varn(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn pop_varn(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn branch(state: &mut ScriptState) {
    state.branch(state.int_operand() as isize);
}

#[inline(always)]
pub(crate) fn branch_not(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a != b {
        state.branch(state.int_operand() as isize);
    }
}

#[inline(always)]
pub(crate) fn branch_equals(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a == b {
        state.branch(state.int_operand() as isize);
    }
}

#[inline(always)]
pub(crate) fn branch_less_than(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a < b {
        state.branch(state.int_operand() as isize);
    }
}

#[inline(always)]
pub(crate) fn branch_greater_than(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a > b {
        state.branch(state.int_operand() as isize);
    }
}

#[inline(always)]
pub(crate) fn push_vars(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn pop_vars(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn _return(state: &mut ScriptState) {
    if state.get_fp() == 0 {
        state.set_execution_state(ScriptExecutionState::Finished);
    } else if let Err(err) = state.pop_frame() {
        state.abort(err);
    }
}

#[inline(always)]
pub(crate) fn gosub(engine: &Engine, state: &mut ScriptState) {
    if state.get_fp() >= 50 {
        return state.abort(String::from("[gosub] stack overflow!"));
    }
    let script: i32 = state.pop_int();
    match engine.get_script(script as usize) {
        None => state.abort(format!("[gosub] proc {} not found!", script)),
        Some(proc) => state.gosub_frame(proc),
    };
}

#[inline(always)]
pub(crate) fn jump(engine: &Engine, state: &mut ScriptState) {
    let script: i32 = state.pop_int();
    match engine.get_script(script as usize) {
        None => state.abort(format!("[jump] label {} not found!", script)),
        Some(label) => state.goto_frame(label),
    };
}

#[inline(always)]
pub(crate) fn switch(state: &mut ScriptState) {
    let key: i32 = state.pop_int();
    if let Ok(Some(result)) = state.get_switch_table(key, state.int_operand()) {
        state.branch(result as isize);
    }
}

#[inline(always)]
pub(crate) fn push_varbit(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn pop_varbit(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn branch_less_than_or_equals(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a <= b {
        state.branch(state.int_operand() as isize);
    }
}

#[inline(always)]
pub(crate) fn branch_greater_than_or_equals(state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a >= b {
        state.branch(state.int_operand() as isize);
    }
}

#[inline(always)]
pub(crate) fn push_int_local(state: &mut ScriptState) {
    let operand: usize = state.int_operand();
    let local: i32 = unsafe { *state.get_int_locals().as_ptr().add(operand) };
    state.push_int(local);
}

#[inline(always)]
pub(crate) fn pop_int_local(state: &mut ScriptState) {
    let operand: usize = state.int_operand();
    let local: i32 = state.pop_int();
    unsafe { *state.get_int_locals().as_mut_ptr().add(operand) = local }
}

#[inline(always)]
pub(crate) fn push_string_local(state: &mut ScriptState) {
    let operand: usize = state.int_operand();
    let local: String = unsafe { (*state.get_string_locals().as_ptr().add(operand)).clone() };
    state.push_string(local);
}

#[inline(always)]
pub(crate) fn pop_string_local(state: &mut ScriptState) {
    let operand: usize = state.int_operand();
    let local: String = state.pop_string();
    unsafe { *state.get_string_locals().as_mut_ptr().add(operand) = local }
}

#[inline(always)]
pub(crate) fn join_string(state: &mut ScriptState) {
    let count: usize = state.int_operand();
    let mut result: String = String::new();
    for _ in 0..count {
        result = state.pop_string() + &result;
    }
    state.push_string(result);
}

#[inline(always)]
pub(crate) fn pop_int_discard(state: &mut ScriptState) {
    state.set_isp(state.get_isp() - 1);
}

#[inline(always)]
pub(crate) fn pop_string_discard(state: &mut ScriptState) {
    state.set_ssp(state.get_ssp() - 1);
}

#[inline(always)]
pub(crate) fn gosub_with_params(engine: &Engine, state: &mut ScriptState) {
    if state.get_fp() >= 50 {
        return state.abort(String::from("[gosub_with_params] stack overflow!"));
    }
    let script: usize = state.int_operand();
    match engine.get_script(script) {
        None => state.abort(format!("[gosub_with_params] proc {} not found!", script)),
        Some(proc) => state.gosub_frame(proc),
    };
}

#[inline(always)]
pub(crate) fn jump_with_params(engine: &Engine, state: &mut ScriptState) {
    let script: usize = state.int_operand();
    match engine.get_script(script) {
        None => state.abort(format!("[jump_with_params] label {} not found!", script)),
        Some(label) => state.goto_frame(label),
    };
}

#[inline(always)]
pub(crate) fn push_varc(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn pop_varc(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn define_array(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn push_array_int(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}

#[inline(always)]
pub(crate) fn pop_array_int(state: &mut ScriptState) {
    state.abort(String::from("Unimplemented!"));
}
