use crate::coord_grid::CoordGrid;
use crate::script::{ScriptOpcode, ScriptState};
use crate::Engine;
use std::rc::Rc;
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
}

pub struct PlayerOps;

impl PlayerOps {
    #[inline(always)]
    pub fn new() -> PlayerOps {
        return PlayerOps;
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
            ScriptOpcode::AllowDesign => Err("Not implemented".to_string()),
            ScriptOpcode::Anim => Err("Not implemented".to_string()),
            ScriptOpcode::BasReadyAnim => Err("Not implemented".to_string()),
            ScriptOpcode::BasRunning => Err("Not implemented".to_string()),
            ScriptOpcode::BasTurnOnSpot => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkB => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkF => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkL => Err("Not implemented".to_string()),
            ScriptOpcode::BasWalkR => Err("Not implemented".to_string()),
            ScriptOpcode::BufferFull => Err("Not implemented".to_string()),
            ScriptOpcode::BuildAppearance => self.buildappearance(state),
            ScriptOpcode::Busy => Err("Not implemented".to_string()),
            ScriptOpcode::CamLookAt => Err("Not implemented".to_string()),
            ScriptOpcode::CamMoveTo => Err("Not implemented".to_string()),
            ScriptOpcode::CamReset => Err("Not implemented".to_string()),
            ScriptOpcode::CamShake => Err("Not implemented".to_string()),
            ScriptOpcode::ClearQueue => Err("Not implemented".to_string()),
            ScriptOpcode::ClearSoftTimer => Err("Not implemented".to_string()),
            ScriptOpcode::ClearTimer => Err("Not implemented".to_string()),
            ScriptOpcode::GetTimer => Err("Not implemented".to_string()),
            ScriptOpcode::Coord => state.protect(&ScriptState::ACTIVE_PLAYER, |state| self.coord(state)),
            ScriptOpcode::Damage => Err("Not implemented".to_string()),
            ScriptOpcode::Displayname => Err("Not implemented".to_string()),
            ScriptOpcode::FaceSquare => Err("Not implemented".to_string()),
            ScriptOpcode::FindUid => Err("Not implemented".to_string()),
            ScriptOpcode::Gender => Err("Not implemented".to_string()),
            ScriptOpcode::GetQueue => Err("Not implemented".to_string()),
            ScriptOpcode::StatAdvance => Err("Not implemented".to_string()),
            ScriptOpcode::HeadiconsGet => Err("Not implemented".to_string()),
            ScriptOpcode::HeadiconsSet => Err("Not implemented".to_string()),
            ScriptOpcode::HealEnergy => Err("Not implemented".to_string()),
            ScriptOpcode::HintCoord => Err("Not implemented".to_string()),
            ScriptOpcode::HintNpc => Err("Not implemented".to_string()),
            ScriptOpcode::HintPlayer => Err("Not implemented".to_string()),
            ScriptOpcode::HintStop => Err("Not implemented".to_string()),
            ScriptOpcode::IfClose => Err("Not implemented".to_string()),
            ScriptOpcode::TutClose => Err("Not implemented".to_string()),
            ScriptOpcode::IfMultiZone => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenChat => Err("Not implemented".to_string()),
            ScriptOpcode::TutOpen => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenMain => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenMainSide => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenSide => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetAnim => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetColour => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetHide => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetModel => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetRecol => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetNpcHead => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetObject => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetPlayerHead => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetPosition => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetResumeButtons => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetTab => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetTabActive => Err("Not implemented".to_string()),
            ScriptOpcode::TutFlash => Err("Not implemented".to_string()),
            ScriptOpcode::IfSetText => Err("Not implemented".to_string()),
            ScriptOpcode::LastLoginInfo => Err("Not implemented".to_string()),
            ScriptOpcode::LastCom => Err("Not implemented".to_string()),
            ScriptOpcode::LastInt => Err("Not implemented".to_string()),
            ScriptOpcode::LastItem => Err("Not implemented".to_string()),
            ScriptOpcode::LastSlot => Err("Not implemented".to_string()),
            ScriptOpcode::LastTargetSlot => Err("Not implemented".to_string()),
            ScriptOpcode::LastUseItem => Err("Not implemented".to_string()),
            ScriptOpcode::LastUseSlot => Err("Not implemented".to_string()),
            ScriptOpcode::LongQueue => Err("Not implemented".to_string()),
            ScriptOpcode::Mes => state.protect(&ScriptState::ACTIVE_PLAYER, |state| self.mes(state)),
            ScriptOpcode::MidiJingle => Err("Not implemented".to_string()),
            ScriptOpcode::MidiSong => Err("Not implemented".to_string()),
            ScriptOpcode::Name => Err("Not implemented".to_string()),
            ScriptOpcode::PApRange => Err("Not implemented".to_string()),
            ScriptOpcode::PArriveDelay => Err("Not implemented".to_string()),
            ScriptOpcode::PCountDialog => Err("Not implemented".to_string()),
            ScriptOpcode::PDelay => Err("Not implemented".to_string()),
            ScriptOpcode::PExactMove => Err("Not implemented".to_string()),
            ScriptOpcode::PFindUid => self.p_finduid(engine, state),
            ScriptOpcode::PLocMerge => Err("Not implemented".to_string()),
            ScriptOpcode::PLogout => Err("Not implemented".to_string()),
            ScriptOpcode::POpHeld => Err("Not implemented".to_string()),
            ScriptOpcode::POpLoc => Err("Not implemented".to_string()),
            ScriptOpcode::POpNpc => Err("Not implemented".to_string()),
            ScriptOpcode::POpNpcT => Err("Not implemented".to_string()),
            ScriptOpcode::POpObj => Err("Not implemented".to_string()),
            ScriptOpcode::POpPlayer => Err("Not implemented".to_string()),
            ScriptOpcode::POpPlayerT => Err("Not implemented".to_string()),
            ScriptOpcode::PPauseButton => Err("Not implemented".to_string()),
            ScriptOpcode::PStopAction => Err("Not implemented".to_string()),
            ScriptOpcode::PTeleJump => Err("Not implemented".to_string()),
            ScriptOpcode::PTeleport => Err("Not implemented".to_string()),
            ScriptOpcode::PWalk => Err("Not implemented".to_string()),
            ScriptOpcode::PlayerFindAllZone => Err("Not implemented".to_string()),
            ScriptOpcode::PlayerFindNext => Err("Not implemented".to_string()),
            ScriptOpcode::Queue => Err("Not implemented".to_string()),
            ScriptOpcode::Say => Err("Not implemented".to_string()),
            ScriptOpcode::WalkTrigger => Err("Not implemented".to_string()),
            ScriptOpcode::SetTimer => Err("Not implemented".to_string()),
            ScriptOpcode::SoftTimer => Err("Not implemented".to_string()),
            ScriptOpcode::SoundSynth => Err("Not implemented".to_string()),
            ScriptOpcode::SpotAnimPl => Err("Not implemented".to_string()),
            ScriptOpcode::StaffModLevel => Err("Not implemented".to_string()),
            ScriptOpcode::Stat => Err("Not implemented".to_string()),
            ScriptOpcode::StatAdd => Err("Not implemented".to_string()),
            ScriptOpcode::StatBase => Err("Not implemented".to_string()),
            ScriptOpcode::StatHeal => Err("Not implemented".to_string()),
            ScriptOpcode::StatSub => Err("Not implemented".to_string()),
            ScriptOpcode::StrongQueue => Err("Not implemented".to_string()),
            ScriptOpcode::Uid => Err("Not implemented".to_string()),
            ScriptOpcode::WeakQueue => Err("Not implemented".to_string()),
            ScriptOpcode::IfOpenMainOverlay => Err("Not implemented".to_string()),
            ScriptOpcode::AfkEvent => Err("Not implemented".to_string()),
            ScriptOpcode::LowMemory => Err("Not implemented".to_string()),
            ScriptOpcode::SetIdkit => Err("Not implemented".to_string()),
            ScriptOpcode::PClearPendingAction => Err("Not implemented".to_string()),
            ScriptOpcode::GetWalkTrigger => Err("Not implemented".to_string()),
            ScriptOpcode::Busy2 => Err("Not implemented".to_string()),
            ScriptOpcode::FindHero => Err("Not implemented".to_string()),
            ScriptOpcode::BothHeroPoints => Err("Not implemented".to_string()),
            ScriptOpcode::SetGender => Err("Not implemented".to_string()),
            ScriptOpcode::SetSkinColour => Err("Not implemented".to_string()),
            ScriptOpcode::PAnimProtect => Err("Not implemented".to_string()),
            ScriptOpcode::RunEnergy => Err("Not implemented".to_string()),
            ScriptOpcode::Weight => Err("Not implemented".to_string()),
            ScriptOpcode::LastCoord => Err("Not implemented".to_string()),
            _ => Err(format!("Unrecognised player ops code: {:?}", code)),
        };
    }

    #[inline(always)]
    fn buildappearance(&self, state: &mut ScriptState) -> Result<(), String> {
        let inv: i32 = state.pop_int();
        state.get_active_player()?.generate_appearance(inv);
        return Ok(());
    }

    #[inline(always)]
    fn coord(&self, state: &mut ScriptState) -> Result<(), String> {
        let player: Rc<Player> = state.get_active_player()?;
        let coord: CoordGrid = CoordGrid::from(player.x(), player.level(), player.z());
        state.push_int(coord.coord as i32);
        return Ok(());
    }

    #[inline(always)]
    fn mes(&self, state: &mut ScriptState) -> Result<(), String> {
        let message: String = state.pop_string();
        state.get_active_player()?.message_game(message);
        return Ok(());
    }

    #[inline(always)]
    fn p_finduid(&self, engine: &Engine, state: &mut ScriptState) -> Result<(), String> {
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
}
