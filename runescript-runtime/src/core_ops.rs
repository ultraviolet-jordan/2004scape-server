use crate::script::{ScriptExecutionState, ScriptOpcode, ScriptState};
use crate::Engine;

#[inline(always)]
#[rustfmt::skip]
pub fn perform_core_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    return match code {
        ScriptOpcode::PushConstantInt => push_constant_int(state),
        ScriptOpcode::PushVarp => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarp => Err(format!("Unimplemented! {:?}", code)),
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
fn push_varp(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
}

#[inline(always)]
fn pop_varp(_: &mut ScriptState) -> Result<(), String> {
    return Err("Not implemented".to_string());
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
