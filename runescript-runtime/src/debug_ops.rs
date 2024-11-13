use crate::script::ScriptState;
use crate::Engine;

#[inline(always)]
pub(crate) fn map_production(engine: &Engine, state: &mut ScriptState) {
    state.push_int(engine.map_production() as i32);
}
