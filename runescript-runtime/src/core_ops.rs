use crate::script::{ScriptExecutionState, ScriptOpcode, ScriptState};
use crate::Engine;

pub struct CoreOps;

impl CoreOps {
    #[inline(always)]
    pub fn new() -> CoreOps {
        return CoreOps;
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
            ScriptOpcode::PushConstantInt => self.push_constant_int(state),
            ScriptOpcode::PushVarp => self.push_varp(state),
            ScriptOpcode::PopVarp => self.pop_varp(state),
            ScriptOpcode::PushConstantString => self.push_constant_string(state),
            ScriptOpcode::PushVarn => self.push_varn(state),
            ScriptOpcode::PopVarn => self.pop_varn(state),
            ScriptOpcode::Branch => self.branch(state),
            ScriptOpcode::BranchNot => self.branch_not(state),
            ScriptOpcode::BranchEquals => self.branch_equals(state),
            ScriptOpcode::BranchLessThan => self.branch_less_than(state),
            ScriptOpcode::BranchGreaterThan => self.branch_greater_than(state),
            ScriptOpcode::PushVars => self.push_vars(state),
            ScriptOpcode::PopVars => self.pop_vars(state),
            ScriptOpcode::Return => self._return(state),
            ScriptOpcode::GoSub => self.gosub(engine, state),
            ScriptOpcode::Jump => self.jump(engine, state),
            ScriptOpcode::Switch => self.switch(state),
            ScriptOpcode::PushVarbit => self.push_varbit(state),
            ScriptOpcode::PopVarbit => self.pop_varbit(state),
            ScriptOpcode::BranchLessThanOrEquals => self.branch_less_than_or_equals(state),
            ScriptOpcode::BranchGreaterThanOrEquals => self.branch_greater_than_or_equals(state),
            ScriptOpcode::PushIntLocal => self.push_int_local(state),
            ScriptOpcode::PopIntLocal => self.pop_int_local(state),
            ScriptOpcode::PushStringLocal => self.push_string_local(state),
            ScriptOpcode::PopStringLocal => self.pop_string_local(state),
            ScriptOpcode::JoinString => self.join_string(state),
            ScriptOpcode::PopIntDiscard => self.pop_int_discard(state),
            ScriptOpcode::PopStringDiscard => self.pop_string_discard(state),
            ScriptOpcode::GoSubWithParams => self.gosub_with_params(engine, state),
            ScriptOpcode::JumpWithParams => self.jump_with_params(engine, state),
            ScriptOpcode::PushVarcInt => self.push_varc(state),
            ScriptOpcode::PopVarcInt => self.pop_varc(state),
            ScriptOpcode::DefineArray => self.define_array(state),
            ScriptOpcode::PushArrayInt => self.push_array_int(state),
            ScriptOpcode::PopArrayInt => self.pop_array_int(state),
            _ => Err(format!("Unrecognised core ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn push_constant_int(&self, state: &mut ScriptState) -> Result<(), String> {
        state.push_int(state.int_operand() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn push_varp(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_varp(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn push_constant_string(&self, state: &mut ScriptState) -> Result<(), String> {
        state.push_string(state.string_operand());
        return Ok(());
    }

    #[inline(always)]
    fn push_varn(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_varn(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn branch(&self, state: &mut ScriptState) -> Result<(), String> {
        state.set_pc(state.get_pc() + state.int_operand() as isize);
        return Ok(());
    }

    #[inline(always)]
    fn branch_not(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a != b {
            state.set_pc(state.get_pc() + state.int_operand() as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn branch_equals(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a == b {
            state.set_pc(state.get_pc() + state.int_operand() as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn branch_less_than(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a < b {
            state.set_pc(state.get_pc() + state.int_operand() as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn branch_greater_than(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a > b {
            state.set_pc(state.get_pc() + state.int_operand() as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn push_vars(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_vars(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn _return(&self, state: &mut ScriptState) -> Result<(), String> {
        if state.get_fp() == 0 {
            state.set_execution_state(ScriptExecutionState::Finished);
            return Ok(());
        }
        state.pop_frame();
        return Ok(());
    }

    #[inline(always)]
    fn gosub(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
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
    fn jump(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let script: i32 = state.pop_int();
        if let Some(script) = engine.get_script(script as usize) {
            state.goto_frame(script);
            return Ok(());
        }
        return Err(format!("[jump] script {} not found!", script));
    }

    #[inline(always)]
    fn switch(&self, state: &mut ScriptState) -> Result<(), String> {
        let key: i32 = state.pop_int();
        if let Ok(Some(result)) = state.get_switch_table(key, state.int_operand()) {
            state.set_pc(state.get_pc() + result as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn push_varbit(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_varbit(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn branch_less_than_or_equals(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a <= b {
            state.set_pc(state.get_pc() + state.int_operand() as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn branch_greater_than_or_equals(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        if a >= b {
            state.set_pc(state.get_pc() + state.int_operand() as isize);
        }
        return Ok(());
    }

    #[inline(always)]
    fn push_int_local(&self, state: &mut ScriptState) -> Result<(), String> {
        let operand: usize = state.int_operand();
        let local: i32 = unsafe { *state.get_int_locals().as_ptr().add(operand) };
        state.push_int(local);
        return Ok(());
    }

    #[inline(always)]
    fn pop_int_local(&self, state: &mut ScriptState) -> Result<(), String> {
        let operand: usize = state.int_operand();
        state.get_int_locals()[operand] = state.pop_int();
        return Ok(());
    }

    #[inline(always)]
    fn push_string_local(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_string_local(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn join_string(&self, state: &mut ScriptState) -> Result<(), String> {
        let count: usize = state.int_operand();
        let mut result: String = String::new();
        for _ in 0..count {
            result = state.pop_string() + &result;
        }
        state.push_string(result);
        return Ok(());
    }

    #[inline(always)]
    fn pop_int_discard(&self, state: &mut ScriptState) -> Result<(), String> {
        state.set_isp(state.get_isp() - 1);
        return Ok(());
    }

    #[inline(always)]
    fn pop_string_discard(&self, state: &mut ScriptState) -> Result<(), String> {
        state.set_ssp(state.get_ssp() - 1);
        return Ok(());
    }

    #[inline(always)]
    fn gosub_with_params(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
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
    fn jump_with_params(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let script: usize = state.int_operand();
        if let Some(script) = engine.get_script(script) {
            state.goto_frame(script);
            return Ok(());
        }
        return Err(format!("[jump_with_params] script {} not found!", script));
    }

    #[inline(always)]
    fn push_varc(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_varc(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn define_array(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn push_array_int(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }

    #[inline(always)]
    fn pop_array_int(&self, _: &mut ScriptState) -> Result<(), String> {
        return Err("Not implemented".to_string());
    }
}
