use crate::script::{ScriptOpcode, ScriptState};
use crate::Engine;

#[inline(always)]
#[rustfmt::skip]
pub fn perform_debug_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    return match code {
        ScriptOpcode::Error => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapProduction => map_production(engine, state),
        ScriptOpcode::MapLastClock => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastWorld => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastClientIn => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastNpc => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastPlayer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastLogout => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastLogin => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastZone => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastClientOut => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastCleanup => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastBandwidthIn => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapLastBandwidthOut => Err(format!("Unimplemented! {:?}", code)),
        _ => Err(format!("Unrecognised debug ops code: {:?}", code)),
    };
}

#[inline(always)]
fn map_production(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    state.push_int(engine.map_production() as i32);
    return Ok(());
}
