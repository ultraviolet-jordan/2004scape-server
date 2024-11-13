use crate::coord_grid::CoordGrid;
use crate::script::{ScriptFile, ScriptState};
use crate::Engine;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    pub type Player;

    // ---- properties

    #[wasm_bindgen(method, getter = uid)]
    pub fn uid(this: &Player) -> i32;

    #[wasm_bindgen(method, setter = activeScript)]
    pub fn active_script(this: &Player, state: ScriptState);

    #[wasm_bindgen(method, setter = protect)]
    pub fn protect(this: &Player, protect: bool);

    #[wasm_bindgen(method, getter = gender)]
    pub fn gender(this: &Player) -> u8;

    #[wasm_bindgen(method, getter = x)]
    pub fn x(this: &Player) -> u16;

    #[wasm_bindgen(method, getter = z)]
    pub fn z(this: &Player) -> u16;

    #[wasm_bindgen(method, getter = level)]
    pub fn level(this: &Player) -> u8;

    // ---- functions

    #[wasm_bindgen(method, js_name = generateAppearance)]
    pub fn generate_appearance(this: &Player, inv: i32);

    #[wasm_bindgen(method, js_name = canAccess)]
    pub fn can_access(this: &Player) -> bool;

    #[wasm_bindgen(method, js_name = messageGame)]
    pub fn message_game(this: &Player, msg: String);

    #[wasm_bindgen(method, js_name = camReset)]
    pub fn cam_reset(this: &Player);

    #[wasm_bindgen(method, js_name = setVar)]
    pub fn set_var(this: &Player, id: u16, value: JsValue);

    #[wasm_bindgen(method, js_name = getVar)]
    pub fn get_var(this: &Player, id: u16) -> JsValue;

    #[wasm_bindgen(method, js_name = setTimer)]
    pub fn set_timer(
        thi: &Player,
        timer: PlayerTimerType,
        script: ScriptFile,
        args: Vec<JsValue>,
        interval: i32,
    );
}

#[repr(u16)]
#[wasm_bindgen]
pub enum PlayerTimerType {
    Normal,
    Soft,
}

#[inline(always)]
pub(crate) fn buildappearance(state: &mut ScriptState) {
    let inv: i32 = state.pop_int();
    match state.get_active_player() {
        Ok(player) => player.generate_appearance(inv),
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
pub(crate) fn cam_reset(state: &mut ScriptState) {
    match state.get_active_player() {
        Ok(player) => player.cam_reset(),
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
pub(crate) fn coord(state: &mut ScriptState) {
    match state.get_active_player() {
        Ok(player) => {
            let coord: CoordGrid = CoordGrid::from(player.x(), player.level(), player.z());
            state.push_int(coord.coord as i32);
        }
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
pub(crate) fn mes(state: &mut ScriptState) {
    let message: String = state.pop_string();
    match state.get_active_player() {
        Ok(player) => player.message_game(message),
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
pub(crate) fn p_finduid(engine: &Engine, state: &mut ScriptState) {
    let uid: i32 = state.pop_int();
    let operand: usize = state.int_operand();

    if state.pointer_get(ScriptState::PROTECTED_ACTIVE_PLAYER[operand]) {
        match state.get_active_player() {
            Ok(player) if player.uid() == uid => {
                // script is already running on this player with protected access, no-op
                state.push_int(1);
                return;
            }
            Err(err) => return state.abort(err),
            _ => {}
        }
    }

    return if let Some(player) = engine.get_player_by_uid(uid) {
        if !player.can_access() {
            state.push_int(0);
            return;
        }
        state.set_active_player(player);
        state.pointer_add(ScriptState::ACTIVE_PLAYER[operand]);
        state.pointer_add(ScriptState::PROTECTED_ACTIVE_PLAYER[operand]);
        state.push_int(1);
    } else {
        state.push_int(0);
    };
}

#[inline(always)]
pub(crate) fn softtimer(engine: &Engine, state: &mut ScriptState) {
    let args: Vec<JsValue> = state.pop_args();
    let interval: i32 = state.pop_int();
    let timer_id: i32 = state.pop_int();
    let script: ScriptFile = match engine.get_script(timer_id as usize) {
        None => return state.abort(format!("Unable to find timer script: {}", timer_id)),
        Some(script) => script,
    };
    match state.get_active_player() {
        Ok(player) => player.set_timer(PlayerTimerType::Soft, script, args, interval),
        Err(err) => state.abort(err),
    }
}
