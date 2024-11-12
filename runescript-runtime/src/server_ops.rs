use crate::coord_grid::CoordGrid;
use crate::script::{ScriptExecutionState, ScriptOpcode, ScriptState};
use crate::Engine;
use rand::random;

pub struct ServerOps;

impl ServerOps {
    #[inline(always)]
    pub fn new() -> ServerOps {
        return ServerOps;
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
            ScriptOpcode::CoordX => self.coord_x(state),
            ScriptOpcode::CoordY => self.coord_y(state),
            ScriptOpcode::CoordZ => self.coord_z(state),
            ScriptOpcode::Distance => self.distance(state),
            ScriptOpcode::HuntAll => Err("Not implemented".to_string()),
            ScriptOpcode::HuntNext => Err("Not implemented".to_string()),
            ScriptOpcode::InZone => self.inzone(state),
            ScriptOpcode::LineOfSight => self.line_of_sight(engine, state),
            ScriptOpcode::LineOfWalk => self.line_of_walk(engine, state),
            ScriptOpcode::MapBlocked => self.map_blocked(engine, state),
            ScriptOpcode::MapIndoors => self.map_indoors(engine, state),
            ScriptOpcode::MapClock => self.map_clock(engine, state),
            ScriptOpcode::MapLocAddUnsafe => Err("Not implemented".to_string()),
            ScriptOpcode::MapMembers => self.map_members(engine, state),
            ScriptOpcode::MapPlayerCount => Err("Not implemented".to_string()),
            ScriptOpcode::MapFindSquare => Err("Not implemented".to_string()),
            ScriptOpcode::MoveCoord => self.movecoord(state),
            ScriptOpcode::PlayerCount => Err("Not implemented".to_string()),
            ScriptOpcode::ProjAnimMap => Err("Not implemented".to_string()),
            ScriptOpcode::ProjAnimNpc => Err("Not implemented".to_string()),
            ScriptOpcode::ProjAnimPl => Err("Not implemented".to_string()),
            ScriptOpcode::SeqLength => Err("Not implemented".to_string()),
            ScriptOpcode::SplitGet => Err("Not implemented".to_string()),
            ScriptOpcode::SplitGetAnim => Err("Not implemented".to_string()),
            ScriptOpcode::SplitInit => Err("Not implemented".to_string()),
            ScriptOpcode::SplitLineCount => Err("Not implemented".to_string()),
            ScriptOpcode::SplitPageCount => Err("Not implemented".to_string()),
            ScriptOpcode::SpotAnimMap => self.spotanim_map(engine, state),
            ScriptOpcode::StatRandom => self.stat_random(state),
            ScriptOpcode::StructParam => Err("Not implemented".to_string()),
            ScriptOpcode::WorldDelay => self.world_delay(state),
            ScriptOpcode::NpcsCount => Err("Not implemented".to_string()),
            ScriptOpcode::ZonesCount => self.zonecount(engine, state),
            ScriptOpcode::LocsCount => self.loccount(engine, state),
            ScriptOpcode::ObjsCount => self.objcount(engine, state),
            ScriptOpcode::MapMulti => Err("Not implemented".to_string()),
            _ => Err(format!("Unrecognised server ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn coord_x(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(a.x() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn coord_y(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(a.y() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn coord_z(&self, state: &mut ScriptState) -> Result<(), String> {
        let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(a.z() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn distance(&self, state: &mut ScriptState) -> Result<(), String> {
        let b: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(a.distance(b) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn inzone(&self, state: &mut ScriptState) -> Result<(), String> {
        let c: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        let b: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);

        if c.x() < a.x() || c.x() > b.x() {
            state.push_int(0);
        } else if c.y() < a.y() || c.y() > b.y() {
            state.push_int(0);
        } else if c.z() < a.z() || c.z() > b.z() {
            state.push_int(0);
        } else {
            state.push_int(1);
        }
        return Ok(());
    }

    #[inline(always)]
    fn line_of_sight(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        // state.push_int(engine.line_of_sight(a, b) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn line_of_walk(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let b: i32 = state.pop_int();
        let a: i32 = state.pop_int();
        // state.push_int(engine.line_of_walk(a, b) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn map_blocked(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let a: i32 = state.pop_int();
        // state.push_int(engine.map_blocked(a) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn map_indoors(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let a: i32 = state.pop_int();
        // state.push_int(engine.map_indoors(a) as i32);
        return Ok(());
    }

    #[inline(always)]
    fn map_clock(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        // state.push_int(engine.map_clock() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn map_members(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        // state.push_int(engine.map_members() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn movecoord(&self, state: &mut ScriptState) -> Result<(), String> {
        let z: i32 = state.pop_int();
        let y: i32 = state.pop_int();
        let x: i32 = state.pop_int();
        let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        state.push_int(coord.movecoord(x as u16, y as u8, z as u16).coord as i32);
        return Ok(());
    }

    #[inline(always)]
    fn spotanim_map(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        let delay: i32 = state.pop_int();
        let height: i32 = state.pop_int();
        let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
        let spotanim: i32 = state.pop_int();

        let x: u16 = coord.x();
        let y: u8 = coord.y();
        let z: u16 = coord.z();

        // engine.with_zone_mut(x, y, z, |zone| {
        //     zone.anim_map(x, z, spotanim as u16, height, delay as u32);
        //     engine.track_zone(engine.map_clock(), zone.index());
        // });

        return Ok(());
    }

    // https://x.com/JagexAsh/status/1110604592138670083
    #[inline(always)]
    fn stat_random(&self, state: &mut ScriptState) -> Result<(), String> {
        let high: i32 = state.pop_int();
        let low: i32 = state.pop_int();
        let level: i32 = state.pop_int();
        // wrap this?
        state.push_int(
            ((low * (99 - level) / 98) + (high * (level - 1) / 98) + 1
                > (random::<f64>() * 256.0) as i32) as i32,
        );
        return Ok(());
    }

    // https://x.com/JagexAsh/status/1730321158858276938
    // https://x.com/JagexAsh/status/1814230119411540058
    #[inline(always)]
    fn world_delay(&self, state: &mut ScriptState) -> Result<(), String> {
        // arg is popped elsewhere
        state.set_execution_state(ScriptExecutionState::WorldSuspended);
        return Ok(());
    }

    #[inline(always)]
    fn zonecount(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        // state.push_int(engine.zonecount() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn loccount(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        // state.push_int(engine.loccount() as i32);
        return Ok(());
    }

    #[inline(always)]
    fn objcount(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
        // state.push_int(engine.objcount() as i32);
        return Ok(());
    }
}
