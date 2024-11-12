use crate::player_ops::Player;
use crate::script::{ScriptExecutionState, ScriptOpcode, ScriptState};
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
#[rustfmt::skip]
pub fn perform_core_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    return match code {
        ScriptOpcode::PushConstantInt => push_constant_int(state),
        ScriptOpcode::PushVarp => push_varp(engine, state),
        ScriptOpcode::PopVarp => pop_varp(engine, state),
        ScriptOpcode::PushConstantString => push_constant_string(state),
        ScriptOpcode::PushVarn => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarn => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Branch => branch(state),
        ScriptOpcode::BranchNot => branch_not(state),
        ScriptOpcode::BranchEquals => branch_equals(state),
        ScriptOpcode::BranchLessThan => branch_less_than(state),
        ScriptOpcode::BranchGreaterThan => branch_greater_than(state),
        ScriptOpcode::PushVars => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVars => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Return => _return(state),
        ScriptOpcode::GoSub => gosub(engine, state),
        ScriptOpcode::Jump => jump(engine, state),
        ScriptOpcode::Switch => switch(state),
        ScriptOpcode::PushVarbit => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarbit => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BranchLessThanOrEquals => branch_less_than_or_equals(state),
        ScriptOpcode::BranchGreaterThanOrEquals => branch_greater_than_or_equals(state),
        ScriptOpcode::PushIntLocal => push_int_local(state),
        ScriptOpcode::PopIntLocal => pop_int_local(state),
        ScriptOpcode::PushStringLocal => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopStringLocal => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::JoinString => join_string(state),
        ScriptOpcode::PopIntDiscard => pop_int_discard(state),
        ScriptOpcode::PopStringDiscard => pop_string_discard(state),
        ScriptOpcode::GoSubWithParams => gosub_with_params(engine, state),
        ScriptOpcode::JumpWithParams => jump_with_params(engine, state),
        ScriptOpcode::PushVarcInt => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarcInt => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::DefineArray => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PushArrayInt => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopArrayInt => Err(format!("Unimplemented! {:?}", code)),
        _ => Err(format!("Unrecognised core ops code: {:?}", code)),
    };
}

#[inline(always)]
fn push_constant_int(state: &mut ScriptState) -> Result<(), String> {
    state.push_int(state.int_operand() as i32);
    return Ok(());
}

#[inline(always)]
fn push_varp(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let secondary: usize = state.int_operand() >> 16 & 0x1;
    if secondary == 1 && state.get_active_player2().is_null() {
        return Err(String::from("No secondary active_player."));
    } else if secondary == 0 && state.get_active_player1().is_null() {
        return Err(String::from("No active_player."));
    }

    let varp_type: VarPlayerType = engine.check_varp((state.int_operand() & 0xffff) as i32)?;
    let player: &Player = if secondary == 1 {
        state.get_active_player2()
    } else {
        state.get_active_player1()
    };
    if varp_type.is_string() {
        if let Some(str) = player.get_var(varp_type.id()).as_string() {
            state.push_string(str);
        } else {
            return Err(String::from("Expected a string varp value."));
        }
    } else {
        if let Some(num) = player.get_var(varp_type.id()).as_f64() {
            state.push_int(num as i32);
        } else {
            return Err(String::from("Expected a numeric varp value."));
        }
    }
    return Ok(());
}

#[inline(always)]
fn pop_varp(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let secondary: usize = state.int_operand() >> 16 & 0x1;
    if secondary == 1 && state.get_active_player2().is_null() {
        return Err(String::from("No secondary active_player."));
    } else if secondary == 0 && state.get_active_player1().is_null() {
        return Err(String::from("No active_player."));
    }

    let varp_type: VarPlayerType = engine.check_varp((state.int_operand() & 0xffff) as i32)?;
    if !state.pointer_get(ScriptState::PROTECTED_ACTIVE_PLAYER[secondary]) && varp_type.protect() {
        return Err(format!(
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
    return Ok(());
}

#[inline(always)]
fn push_constant_string(state: &mut ScriptState) -> Result<(), String> {
    state.push_string(state.string_operand());
    return Ok(());
}

#[inline(always)]
fn push_varn(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_varn(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn branch(state: &mut ScriptState) -> Result<(), String> {
    state.set_pc(state.get_pc() + state.int_operand() as isize);
    return Ok(());
}

#[inline(always)]
fn branch_not(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a != b {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
    }
    return Ok(());
}

#[inline(always)]
fn branch_equals(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a == b {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
    }
    return Ok(());
}

#[inline(always)]
fn branch_less_than(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a < b {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
    }
    return Ok(());
}

#[inline(always)]
fn branch_greater_than(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a > b {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
    }
    return Ok(());
}

#[inline(always)]
fn push_vars(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_vars(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn _return(state: &mut ScriptState) -> Result<(), String> {
    if state.get_fp() == 0 {
        state.set_execution_state(ScriptExecutionState::Finished);
        return Ok(());
    }
    return state.pop_frame();
}

#[inline(always)]
fn gosub(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    if state.get_fp() >= 50 {
        return Err("stack overflow!".to_string());
    }
    let script: i32 = state.pop_int();
    if let Some(script) = engine.get_script(script as usize) {
        state.gosub_frame(script);
        return Ok(());
    }
    return Err(format!("[gosub] script {} not found!", script));
}

#[inline(always)]
fn jump(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let script: i32 = state.pop_int();
    if let Some(script) = engine.get_script(script as usize) {
        state.goto_frame(script);
        return Ok(());
    }
    return Err(format!("[jump] script {} not found!", script));
}

#[inline(always)]
fn switch(state: &mut ScriptState) -> Result<(), String> {
    let key: i32 = state.pop_int();
    if let Ok(Some(result)) = state.get_switch_table(key, state.int_operand()) {
        state.set_pc(state.get_pc() + result as isize);
    }
    return Ok(());
}

#[inline(always)]
fn push_varbit(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_varbit(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn branch_less_than_or_equals(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a <= b {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
    }
    return Ok(());
}

#[inline(always)]
fn branch_greater_than_or_equals(state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    if a >= b {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
    }
    return Ok(());
}

#[inline(always)]
fn push_int_local(state: &mut ScriptState) -> Result<(), String> {
    let operand: usize = state.int_operand();
    let local: i32 = unsafe { *state.get_int_locals().as_ptr().add(operand) };
    state.push_int(local);
    return Ok(());
}

#[inline(always)]
fn pop_int_local(state: &mut ScriptState) -> Result<(), String> {
    let operand: usize = state.int_operand();
    state.get_int_locals()[operand] = state.pop_int();
    return Ok(());
}

#[inline(always)]
fn push_string_local(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_string_local(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn join_string(state: &mut ScriptState) -> Result<(), String> {
    let count: usize = state.int_operand();
    let mut result: String = String::new();
    for _ in 0..count {
        result = state.pop_string() + &result;
    }
    state.push_string(result);
    return Ok(());
}

#[inline(always)]
fn pop_int_discard(state: &mut ScriptState) -> Result<(), String> {
    state.pop_int();
    return Ok(());
}

#[inline(always)]
fn pop_string_discard(state: &mut ScriptState) -> Result<(), String> {
    state.pop_string();
    return Ok(());
}

#[inline(always)]
fn gosub_with_params(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    if state.get_fp() >= 50 {
        return Err("stack overflow!".to_string());
    }
    let script: usize = state.int_operand();
    if let Some(script) = engine.get_script(script) {
        state.gosub_frame(script);
        return Ok(());
    }
    return Err(format!("[gosub_with_params] script {} not found!", script));
}

#[inline(always)]
fn jump_with_params(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let script: usize = state.int_operand();
    if let Some(script) = engine.get_script(script) {
        state.goto_frame(script);
        return Ok(());
    }
    return Err(format!("[jump_with_params] script {} not found!", script));
}

#[inline(always)]
fn push_varc(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_varc(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn define_array(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn push_array_int(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_array_int(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}
