use crate::script::{ScriptOpcode, ScriptState};
use crate::Engine;

#[inline(always)]
#[rustfmt::skip]
pub fn perform_debug_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) {
    return match code {
        ScriptOpcode::Error => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapProduction => map_production(engine, state),
        ScriptOpcode::MapLastClock => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastWorld => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastClientIn => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastNpc => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastPlayer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastLogout => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastLogin => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastZone => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastClientOut => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastCleanup => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastBandwidthIn => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastBandwidthOut => state.abort(format!("Unimplemented! {:?}", code)),
        _ => state.abort(format!("Unrecognised debug ops code: {:?}", code)),
    };
}

#[inline(always)]
fn map_production(engine: &Engine, state: &mut ScriptState) {
    state.push_int(engine.map_production() as i32);
}
