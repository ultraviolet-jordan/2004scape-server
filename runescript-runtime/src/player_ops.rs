use crate::coord_grid::CoordGrid;
use crate::script::{ScriptOpcode, ScriptState};
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
}

#[inline(always)]
#[rustfmt::skip]
pub fn perform_player_operation(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    return match code {
        ScriptOpcode::AllowDesign => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Anim => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasReadyAnim => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasRunning => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasTurnOnSpot => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkB => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkF => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkL => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BasWalkR => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BufferFull => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BuildAppearance => buildappearance(state),
        ScriptOpcode::Busy => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamLookAt => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamMoveTo => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamReset => state.protect(&ScriptState::ACTIVE_PLAYER, |state| cam_reset(state)),
        ScriptOpcode::CamShake => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearQueue => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearSoftTimer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearTimer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetTimer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Coord => state.protect(&ScriptState::ACTIVE_PLAYER, |state| coord(state)),
        ScriptOpcode::Damage => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Displayname => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::FaceSquare => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::FindUid => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Gender => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetQueue => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatAdvance => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HeadiconsGet => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HeadiconsSet => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HealEnergy => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintCoord => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintNpc => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintPlayer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HintStop => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfClose => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::TutClose => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfMultiZone => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenChat => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::TutOpen => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenMain => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenMainSide => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenSide => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetAnim => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetColour => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetHide => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetModel => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetRecol => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetNpcHead => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetObject => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetPlayerHead => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetPosition => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetResumeButtons => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetTab => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetTabActive => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::TutFlash => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfSetText => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastLoginInfo => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastCom => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastInt => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastItem => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastSlot => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastTargetSlot => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastUseItem => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastUseSlot => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LongQueue => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Mes => state.protect(&ScriptState::ACTIVE_PLAYER, |state| mes(state)),
        ScriptOpcode::MidiJingle => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MidiSong => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Name => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PApRange => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PArriveDelay => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PCountDialog => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PDelay => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PExactMove => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PFindUid => p_finduid(engine, state),
        ScriptOpcode::PLocMerge => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PLogout => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpHeld => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpLoc => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpNpc => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpNpcT => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpObj => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpPlayer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::POpPlayerT => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PPauseButton => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PStopAction => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PTeleJump => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PTeleport => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PWalk => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PlayerFindAllZone => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PlayerFindNext => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Queue => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Say => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::WalkTrigger => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetTimer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SoftTimer => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SoundSynth => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SpotAnimPl => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StaffModLevel => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Stat => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatAdd => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatBase => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatHeal => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StatSub => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::StrongQueue => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Uid => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::WeakQueue => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::IfOpenMainOverlay => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::AfkEvent => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LowMemory => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetIdkit => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PClearPendingAction => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetWalkTrigger => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Busy2 => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::FindHero => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BothHeroPoints => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetGender => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SetSkinColour => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PAnimProtect => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::RunEnergy => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Weight => Err(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::LastCoord => Err(format!("Unimplemented! {:?}", code)),
        _ => Err(format!("Unrecognised player ops code: {:?}", code)),
    };
}

#[inline(always)]
fn buildappearance(state: &mut ScriptState) -> Result<(), String> {
    let inv: i32 = state.pop_int();
    state.get_active_player()?.generate_appearance(inv);
    return Ok(());
}

#[inline(always)]
fn cam_reset(state: &mut ScriptState) -> Result<(), String> {
    state.get_active_player()?.cam_reset();
    return Ok(());
}

#[inline(always)]
fn coord(state: &mut ScriptState) -> Result<(), String> {
    let player: &Player = state.get_active_player()?;
    let coord: CoordGrid = CoordGrid::from(player.x(), player.level(), player.z());
    state.push_int(coord.coord as i32);
    return Ok(());
}

#[inline(always)]
fn mes(state: &mut ScriptState) -> Result<(), String> {
    let message: String = state.pop_string();
    state.get_active_player()?.message_game(message);
    return Ok(());
}

#[inline(always)]
fn p_finduid(engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
    let uid: i32 = state.pop_int();
    let operand: usize = state.int_operand();

    if state.pointer_get(ScriptState::PROTECTED_ACTIVE_PLAYER[operand])
        && state.get_active_player()?.uid() == uid
    {
        // script is already running on this player with protected access, no-op
        state.push_int(1);
        return Ok(());
    }

    return if let Some(player) = engine.get_player_by_uid(uid) {
        if !player.can_access() {
            state.push_int(0);
            return Ok(());
        }
        state.set_active_player(player);
        state.pointer_add(ScriptState::ACTIVE_PLAYER[operand]);
        state.pointer_add(ScriptState::PROTECTED_ACTIVE_PLAYER[operand]);
        state.push_int(1);
        Ok(())
    } else {
        state.push_int(0);
        Ok(())
    };
}
