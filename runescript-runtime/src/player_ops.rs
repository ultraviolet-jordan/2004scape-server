use crate::coord_grid::CoordGrid;
use crate::script::{ScriptFile, ScriptOpcode, ScriptState};
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
#[rustfmt::skip]
pub fn perform_player_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) {
    return match code {
        ScriptOpcode::AllowDesign => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Anim => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasReadyAnim => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasRunning => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasTurnOnSpot => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkB => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkF => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkL => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkR => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BufferFull => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BuildAppearance => buildappearance(state),
        ScriptOpcode::Busy => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamLookAt => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamMoveTo => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamReset => state.protect(&ScriptState::ACTIVE_PLAYER, |state| cam_reset(state)),
        ScriptOpcode::CamShake => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearQueue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearSoftTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Coord => state.protect(&ScriptState::ACTIVE_PLAYER, |state| coord(state)),
        ScriptOpcode::Damage => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Displayname => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::FaceSquare => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::FindUid => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Gender => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetQueue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatAdvance => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HeadiconsGet => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HeadiconsSet => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HealEnergy => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintCoord => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintNpc => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintPlayer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintStop => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfClose => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::TutClose => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfMultiZone => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenChat => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::TutOpen => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenMain => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenMainSide => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenSide => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetAnim => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetColour => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetHide => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetModel => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetRecol => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetNpcHead => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetObject => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetPlayerHead => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetPosition => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetResumeButtons => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetTab => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetTabActive => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::TutFlash => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetText => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastLoginInfo => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastCom => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastInt => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastItem => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastSlot => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastTargetSlot => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastUseItem => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastUseSlot => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LongQueue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Mes => state.protect(&ScriptState::ACTIVE_PLAYER, |state| mes(state)),
        ScriptOpcode::MidiJingle => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MidiSong => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Name => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PApRange => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PArriveDelay => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PCountDialog => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PDelay => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PExactMove => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PFindUid => p_finduid(engine, state),
        ScriptOpcode::PLocMerge => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PLogout => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpHeld => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpLoc => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpNpc => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpNpcT => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpObj => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpPlayer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpPlayerT => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PPauseButton => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PStopAction => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PTeleJump => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PTeleport => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PWalk => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PlayerFindAllZone => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PlayerFindNext => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Queue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Say => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::WalkTrigger => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SoftTimer => state.protect(&ScriptState::ACTIVE_PLAYER, |state| softtimer(engine, state)),
        ScriptOpcode::SoundSynth => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SpotAnimPl => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StaffModLevel => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Stat => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatAdd => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatBase => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatHeal => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatSub => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StrongQueue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Uid => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::WeakQueue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenMainOverlay => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::AfkEvent => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LowMemory => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetIdkit => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PClearPendingAction => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetWalkTrigger => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Busy2 => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::FindHero => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BothHeroPoints => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetGender => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetSkinColour => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PAnimProtect => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::RunEnergy => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Weight => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastCoord => state.abort(format!("Unimplemented! {:?}", code)),
        _ => state.abort(format!("Unrecognised player ops code: {:?}", code)),
    };
}

#[inline(always)]
fn buildappearance(state: &mut ScriptState) {
    let inv: i32 = state.pop_int();
    match state.get_active_player() {
        Ok(player) => player.generate_appearance(inv),
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
fn cam_reset(state: &mut ScriptState) {
    match state.get_active_player() {
        Ok(player) => player.cam_reset(),
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
fn coord(state: &mut ScriptState) {
    match state.get_active_player() {
        Ok(player) => {
            let coord: CoordGrid = CoordGrid::from(player.x(), player.level(), player.z());
            state.push_int(coord.coord as i32);
        }
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
fn mes(state: &mut ScriptState) {
    let message: String = state.pop_string();
    match state.get_active_player() {
        Ok(player) => player.message_game(message),
        Err(err) => state.abort(err),
    };
}

#[inline(always)]
fn p_finduid(engine: &Engine, state: &mut ScriptState) {
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
fn softtimer(engine: &Engine, state: &mut ScriptState) {
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
