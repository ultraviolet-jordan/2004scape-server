use crate::script::{ScriptOpcode, ScriptState};
use crate::Engine;

pub struct DebugOps;

impl DebugOps {
    #[inline(always)]
    pub fn new() -> DebugOps {
        return DebugOps;
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
            ScriptOpcode::Error => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapProduction => self.map_production(engine, state),
            ScriptOpcode::MapLastClock => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastWorld => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastClientIn => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastNpc => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastPlayer => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastLogout => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastLogin => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastZone => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastClientOut => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastCleanup => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastBandwidthIn => Err(String::from("Unimplemented!")),
            ScriptOpcode::MapLastBandwidthOut => Err(String::from("Unimplemented!")),
            _ => Err(format!("Unrecognised debug ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn map_production(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        state.push_int(engine.map_production() as i32);
        return Ok(());
    }
}
