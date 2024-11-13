use crate::coord_grid::CoordGrid;
use crate::script::{ScriptExecutionState, ScriptState};
use crate::Engine;
use rand::random;

#[inline(always)]
pub(crate) fn coord_x(state: &mut ScriptState) {
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.x() as i32);
}

#[inline(always)]
pub(crate) fn coord_y(state: &mut ScriptState) {
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.y() as i32);
}

#[inline(always)]
pub(crate) fn coord_z(state: &mut ScriptState) {
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.z() as i32);
}

#[inline(always)]
pub(crate) fn distance(state: &mut ScriptState) {
    let b: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    let a: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(a.distance(b) as i32);
}

#[inline(always)]
pub(crate) fn inzone(state: &mut ScriptState) {
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
}

#[inline(always)]
pub(crate) fn line_of_sight(engine: &Engine, state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    // state.push_int(engine.line_of_sight(a, b) as i32);
}

#[inline(always)]
pub(crate) fn line_of_walk(engine: &Engine, state: &mut ScriptState) {
    let b: i32 = state.pop_int();
    let a: i32 = state.pop_int();
    // state.push_int(engine.line_of_walk(a, b) as i32);
}

#[inline(always)]
pub(crate) fn map_blocked(engine: &Engine, state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    // state.push_int(engine.map_blocked(a) as i32);
}

#[inline(always)]
pub(crate) fn map_indoors(engine: &Engine, state: &mut ScriptState) {
    let a: i32 = state.pop_int();
    // state.push_int(engine.map_indoors(a) as i32);
}

#[inline(always)]
pub(crate) fn map_clock(engine: &Engine, state: &mut ScriptState) {
    // state.push_int(engine.map_clock() as i32);
}

#[inline(always)]
pub(crate) fn map_members(engine: &Engine, state: &mut ScriptState) {
    // state.push_int(engine.map_members() as i32);
}

#[inline(always)]
pub(crate) fn movecoord(state: &mut ScriptState) {
    let z: i32 = state.pop_int();
    let y: i32 = state.pop_int();
    let x: i32 = state.pop_int();
    let coord: CoordGrid = CoordGrid::new(state.pop_int() as u32);
    state.push_int(coord.movecoord(x as u16, y as u8, z as u16).coord as i32);
}

#[inline(always)]
pub(crate) fn spotanim_map(engine: &Engine, state: &mut ScriptState) {
    let delay: i32 = state.pop_int();
    let height: i32 = state.pop_int();
    let coord: i32 = state.pop_int();
    let spotanim: i32 = state.pop_int();

    let coord: CoordGrid = CoordGrid::new(coord as u32);
    let x: u16 = coord.x();
    let y: u8 = coord.y();
    let z: u16 = coord.z();

    // engine.with_zone_mut(x, y, z, |zone| {
    //     zone.anim_map(x, z, spotanim as u16, height, delay as u32);
    //     engine.track_zone(engine.map_clock(), zone.index());
    // });
}

// https://x.com/JagexAsh/status/1110604592138670083
#[inline(always)]
pub(crate) fn stat_random(state: &mut ScriptState) {
    let high: i32 = state.pop_int();
    let low: i32 = state.pop_int();
    let level: i32 = state.pop_int();
    // wrap this?
    state.push_int(
        ((low * (99 - level) / 98) + (high * (level - 1) / 98) + 1
            > (random::<f64>() * 256.0) as i32) as i32,
    );
}

// https://x.com/JagexAsh/status/1730321158858276938
// https://x.com/JagexAsh/status/1814230119411540058
#[inline(always)]
pub(crate) fn world_delay(state: &mut ScriptState) {
    // arg is popped elsewhere
    state.set_execution_state(ScriptExecutionState::WorldSuspended);
}

#[inline(always)]
pub(crate) fn zonecount(engine: &Engine, state: &mut ScriptState) {
    // state.push_int(engine.zonecount() as i32);
}

#[inline(always)]
pub(crate) fn loccount(engine: &Engine, state: &mut ScriptState) {
    // state.push_int(engine.loccount() as i32);
}

#[inline(always)]
pub(crate) fn objcount(engine: &Engine, state: &mut ScriptState) {
    // state.push_int(engine.objcount() as i32);
}
