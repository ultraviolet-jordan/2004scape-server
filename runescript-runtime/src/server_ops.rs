use crate::coord_grid::CoordGrid;
use crate::script::{ScriptExecutionState, ScriptOpcode, ScriptState};
use crate::Engine;
use rand::random;

#[inline(always)]
#[rustfmt::skip]
pub fn perform_server_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    return match code {
        ScriptOpcode::CoordX => coord_x(state),
        ScriptOpcode::CoordY => coord_y(state),
        ScriptOpcode::CoordZ => coord_z(state),
        ScriptOpcode::Distance => distance(state),
        ScriptOpcode::HuntAll => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HuntNext => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::InZone => inzone(state),
        ScriptOpcode::LineOfSight => line_of_sight(engine, state),
        ScriptOpcode::LineOfWalk => line_of_walk(engine, state),
        ScriptOpcode::MapBlocked => map_blocked(engine, state),
        ScriptOpcode::MapIndoors => map_indoors(engine, state),
        ScriptOpcode::MapClock => map_clock(engine, state),
        ScriptOpcode::MapLocAddUnsafe => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapMembers => map_members(engine, state),
        ScriptOpcode::MapPlayerCount => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapFindSquare => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MoveCoord => movecoord(state),
        ScriptOpcode::PlayerCount => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ProjAnimMap => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ProjAnimNpc => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ProjAnimPl => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SeqLength => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitGet => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitGetAnim => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitInit => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitLineCount => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitPageCount => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SpotAnimMap => spotanim_map(engine, state),
        ScriptOpcode::StatRandom => stat_random(state),
        ScriptOpcode::StructParam => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::WorldDelay => world_delay(state),
        ScriptOpcode::NpcsCount => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ZonesCount => zonecount(engine, state),
        ScriptOpcode::LocsCount => loccount(engine, state),
        ScriptOpcode::ObjsCount => objcount(engine, state),
        ScriptOpcode::MapMulti => Err(format!("Unimplemented! {:?}", code)),
        _ => Err(format!("Unrecognised server ops code: {:?}", code)),
    };
}

#[inline(always)]
fn coord_x(state: &mut ScriptState) -> Result<(), String> {
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.x() as i32);
    return Ok(());
}

#[inline(always)]
fn coord_y(state: &mut ScriptState) -> Result<(), String> {
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.y() as i32);
    return Ok(());
}

#[inline(always)]
fn coord_z(state: &mut ScriptState) -> Result<(), String> {
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.z() as i32);
    return Ok(());
}

#[inline(always)]
fn distance(state: &mut ScriptState) -> Result<(), String> {
    let b: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.distance(b) as i32);
    return Ok(());
}

#[inline(always)]
fn inzone(state: &mut ScriptState) -> Result<(), String> {
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
fn line_of_sight(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    // state.push_int(engine.line_of_sight(a, b) as i32);
    return Ok(());
}

#[inline(always)]
fn line_of_walk(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    // state.push_int(engine.line_of_walk(a, b) as i32);
    return Ok(());
}

#[inline(always)]
fn map_blocked(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let a: i32 = state.pop_int();
    // state.push_int(engine.map_blocked(a) as i32);
    return Ok(());
}

#[inline(always)]
fn map_indoors(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let a: i32 = state.pop_int();
    // state.push_int(engine.map_indoors(a) as i32);
    return Ok(());
}

#[inline(always)]
fn map_clock(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    // state.push_int(engine.map_clock() as i32);
    return Ok(());
}

#[inline(always)]
fn map_members(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    // state.push_int(engine.map_members() as i32);
    return Ok(());
}

#[inline(always)]
fn movecoord(state: &mut ScriptState) -> Result<(), String> {
    let z: i32 = state.pop_int();
    let y: i32 = state.pop_int();
    let x: i32 = state.pop_int();
    let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(coord.movecoord(x as u16, y as u8, z as u16).coord as i32);
    return Ok(());
}

#[inline(always)]
fn spotanim_map(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
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
fn stat_random(state: &mut ScriptState) -> Result<(), String> {
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
fn world_delay(state: &mut ScriptState) -> Result<(), String> {
    // arg is popped elsewhere
    state.set_execution_state(ScriptExecutionState::WorldSuspended);
    return Ok(());
}

#[inline(always)]
fn zonecount(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    // state.push_int(engine.zonecount() as i32);
    return Ok(());
}

#[inline(always)]
fn loccount(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    // state.push_int(engine.loccount() as i32);
    return Ok(());
}

#[inline(always)]
fn objcount(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    // state.push_int(engine.objcount() as i32);
    return Ok(());
}
