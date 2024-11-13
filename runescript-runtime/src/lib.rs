use crate::bits::Bits;
use crate::coord_grid::CoordGrid;
use crate::core_ops::VarPlayerType;
use crate::obj_ops::{Obj, ObjType};
use crate::player_ops::Player;
use crate::script::{ScriptExecutionState, ScriptFile, ScriptOpcode, ScriptState};
use crate::trig::Trig;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod bits;
mod coord_grid;
mod core_ops;
mod debug_ops;
mod loc_ops;
mod math_ops;
mod npc_ops;
mod obj_ops;
mod player_ops;
mod script;
mod server_ops;
mod string_ops;
mod trig;

static TRIG: Lazy<Trig> = Lazy::new(Trig::new);
static BITS: Lazy<Bits> = Lazy::new(Bits::new);

#[wasm_bindgen]
#[rustfmt::skip]
extern "C" {
    pub type Engine;

    #[wasm_bindgen(method, js_name = getPlayerByUid)]
    pub fn get_player_by_uid(this: &Engine, uid: i32) -> Option<Player>;

    #[wasm_bindgen(method, js_name = getScript)]
    pub fn get_script(this: &Engine, script: usize) -> Option<ScriptFile>;

    #[wasm_bindgen(method, js_name = getObjType)]
    pub fn get_obj_type(this: &Engine, id: u16) -> Option<ObjType>;

    #[wasm_bindgen(method, js_name = getVarpType)]
    pub fn get_varp_type(this: &Engine, id: u16) -> Option<VarPlayerType>;

    #[wasm_bindgen(method, js_name = isProduction)]
    pub fn map_production(this: &Engine) -> bool;

    #[wasm_bindgen(method, js_name = isMembers)]
    pub fn map_members(this: &Engine) -> bool;

    #[wasm_bindgen(method, js_name = objAddAll)]
    pub fn obj_addall(this: &Engine, x: u16, y: u8, z: u16, id: i32, count: i32, duration: i32, stackable: bool) -> Obj;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Engine {
    #[inline(always)]
    pub fn check_coord(&self, coord: i32) -> Result<CoordGrid, String> {
        if coord < 0 {
            return Err(format!(
                "An input for a [coord] was out of range. Range should be: {} to {}. Input was {}.",
                0,
                i32::MAX,
                coord
            ));
        }
        return Ok(CoordGrid::new(coord as u32));
    }

    #[inline(always)]
    pub fn check_obj(&self, id: i32) -> Result<ObjType, String> {
        return self.get_obj_type(id as u16).ok_or(format!(
            "An input for a [obj] type was not valid to use. Input was {}.",
            id
        ));
    }

    #[inline(always)]
    pub fn check_obj_stack(&self, count: i32) -> Result<i32, String> {
        if count < 1 {
            return Err(format!(
                "An input for a [objstack] was out of range. Range should be: {} to {}. Input was {}.",
                1,
                i32::MAX,
                count
            ));
        }
        return Ok(count);
    }

    #[inline(always)]
    pub fn check_duration(&self, duration: i32) -> Result<i32, String> {
        if duration < 1 {
            return Err(format!(
                "An input for a [duration] was out of range. Range should be: {} to {}. Input was {}.",
                1,
                i32::MAX,
                duration
            ));
        }
        return Ok(duration);
    }

    #[inline(always)]
    pub fn check_varp(&self, id: i32) -> Result<VarPlayerType, String> {
        return self.get_varp_type(id as u16).ok_or(format!(
            "An input for a [varp] type was not valid to use. Input was {}.",
            id
        ));
    }
}

#[wasm_bindgen]
impl ScriptState {
    pub fn execute(&mut self, engine: &Engine) {
        self.set_execution_state(ScriptExecutionState::Running);
        while self.get_execution_state() == ScriptExecutionState::Running {
            let pc: isize = self.get_pc();
            let opcount: i32 = self.get_opcount();
            let codes: &Vec<u16> = self.opcodes();
            let len: usize = codes.len();
            if pc >= len as isize || pc < -1 {
                self.abort(format!(
                    "Invalid program counter: {}, max expected: {}",
                    pc, len
                ));
                return;
            }
            if opcount > 500_000 {
                self.abort(String::from("Too many instructions!"));
                return;
            }
            let pc: isize = self.get_pc() + 1;
            let code: u16 = codes[pc as usize];
            self.set_opcount(opcount + 1);
            self.set_pc(pc);
            execute_command(engine, self, ScriptOpcode::from(code))
        }
    }
}

pub fn execute_command(engine: &Engine, state: &mut ScriptState, code: ScriptOpcode) {
    // info!("{:?}", code);
    // log(format!("{:?}", code).as_str());
    // log(format!("{:?}", state.get_script().name()).as_str());
    match code {
        // Core language ops (0-99)
        ScriptOpcode::PushConstantInt => core_ops::push_constant_int(state),
        ScriptOpcode::PushVarp => core_ops::push_varp(engine, state),
        ScriptOpcode::PopVarp => core_ops::pop_varp(engine, state),
        ScriptOpcode::PushConstantString => core_ops::push_constant_string(state),
        ScriptOpcode::PushVarn => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarn => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Branch => core_ops::branch(state),
        ScriptOpcode::BranchNot => core_ops::branch_not(state),
        ScriptOpcode::BranchEquals => core_ops::branch_equals(state),
        ScriptOpcode::BranchLessThan => core_ops::branch_less_than(state),
        ScriptOpcode::BranchGreaterThan => core_ops::branch_greater_than(state),
        ScriptOpcode::PushVars => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVars => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Return => core_ops::_return(state),
        ScriptOpcode::GoSub => core_ops::gosub(engine, state),
        ScriptOpcode::Jump => core_ops::jump(engine, state),
        ScriptOpcode::Switch => core_ops::switch(state),
        ScriptOpcode::PushVarbit => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarbit => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::BranchLessThanOrEquals => core_ops::branch_less_than_or_equals(state),
        ScriptOpcode::BranchGreaterThanOrEquals => core_ops::branch_greater_than_or_equals(state),
        ScriptOpcode::PushIntLocal => core_ops::push_int_local(state),
        ScriptOpcode::PopIntLocal => core_ops::pop_int_local(state),
        ScriptOpcode::PushStringLocal => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopStringLocal => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::JoinString => core_ops::join_string(state),
        ScriptOpcode::PopIntDiscard => core_ops::pop_int_discard(state),
        ScriptOpcode::PopStringDiscard => core_ops::pop_string_discard(state),
        ScriptOpcode::GoSubWithParams => core_ops::gosub_with_params(engine, state),
        ScriptOpcode::JumpWithParams => core_ops::jump_with_params(engine, state),
        ScriptOpcode::PushVarcInt => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopVarcInt => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::DefineArray => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PushArrayInt => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PopArrayInt => state.abort(format!("Unimplemented! {:?}", code)),
        // Server ops (1000-1999)
        ScriptOpcode::CoordX => server_ops::coord_x(state),
        ScriptOpcode::CoordY => server_ops::coord_y(state),
        ScriptOpcode::CoordZ => server_ops::coord_z(state),
        ScriptOpcode::Distance => server_ops::distance(state),
        ScriptOpcode::HuntAll => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::HuntNext => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::InZone => server_ops::inzone(state),
        ScriptOpcode::LineOfSight => server_ops::line_of_sight(engine, state),
        ScriptOpcode::LineOfWalk => server_ops::line_of_walk(engine, state),
        ScriptOpcode::MapBlocked => server_ops::map_blocked(engine, state),
        ScriptOpcode::MapIndoors => server_ops::map_indoors(engine, state),
        ScriptOpcode::MapClock => server_ops::map_clock(engine, state),
        ScriptOpcode::MapLocAddUnsafe => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapMembers => server_ops::map_members(engine, state),
        ScriptOpcode::MapPlayerCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapFindSquare => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MoveCoord => server_ops::movecoord(state),
        ScriptOpcode::PlayerCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ProjAnimMap => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ProjAnimNpc => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ProjAnimPl => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SeqLength => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitGet => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitGetAnim => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitInit => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitLineCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SplitPageCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::SpotAnimMap => server_ops::spotanim_map(engine, state),
        ScriptOpcode::StatRandom => server_ops::stat_random(state),
        ScriptOpcode::StructParam => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::WorldDelay => server_ops::world_delay(state),
        ScriptOpcode::NpcsCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ZonesCount => server_ops::zonecount(engine, state),
        ScriptOpcode::LocsCount => server_ops::loccount(engine, state),
        ScriptOpcode::ObjsCount => server_ops::objcount(engine, state),
        ScriptOpcode::MapMulti => state.abort(format!("Unimplemented! {:?}", code)),
        // Player ops (2000-2499)
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
        ScriptOpcode::BuildAppearance => player_ops::buildappearance(state),
        ScriptOpcode::Busy => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamLookAt => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamMoveTo => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::CamReset => state.protect(&ScriptState::ACTIVE_PLAYER, |state| {
            player_ops::cam_reset(state)
        }),
        ScriptOpcode::CamShake => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearQueue => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearSoftTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ClearTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::GetTimer => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Coord => state.protect(&ScriptState::ACTIVE_PLAYER, |state| {
            player_ops::coord(state)
        }),
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
        ScriptOpcode::Mes => {
            state.protect(&ScriptState::ACTIVE_PLAYER, |state| player_ops::mes(state))
        }
        ScriptOpcode::MidiJingle => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MidiSong => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::Name => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PApRange => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PArriveDelay => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PCountDialog => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PDelay => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PExactMove => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::PFindUid => player_ops::p_finduid(engine, state),
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
        ScriptOpcode::SoftTimer => state.protect(&ScriptState::ACTIVE_PLAYER, |state| {
            player_ops::softtimer(engine, state)
        }),
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
        // Npc ops (2500-2999)
        ScriptOpcode::NpcAdd
        | ScriptOpcode::NpcAnim
        | ScriptOpcode::NpcBaseStat
        | ScriptOpcode::NpcCategory
        | ScriptOpcode::NpcChangeType
        | ScriptOpcode::NpcCoord
        | ScriptOpcode::NpcDamage
        | ScriptOpcode::NpcDel
        | ScriptOpcode::NpcDelay
        | ScriptOpcode::NpcFaceSquare
        | ScriptOpcode::NpcFind
        | ScriptOpcode::NpcFindAllAny
        | ScriptOpcode::NpcFindAll
        | ScriptOpcode::NpcFindExact
        | ScriptOpcode::NpcFindHero
        | ScriptOpcode::NpcFindAllZone
        | ScriptOpcode::NpcFindNext
        | ScriptOpcode::NpcFindUid
        | ScriptOpcode::NpcGetMode
        | ScriptOpcode::NpcHeroPoints
        | ScriptOpcode::NpcName
        | ScriptOpcode::NpcParam
        | ScriptOpcode::NpcQueue
        | ScriptOpcode::NpcRange
        | ScriptOpcode::NpcSay
        | ScriptOpcode::NpcHuntAll
        | ScriptOpcode::NpcHuntNext
        | ScriptOpcode::NpcSetHunt
        | ScriptOpcode::NpcSetHuntMode
        | ScriptOpcode::NpcSetMode
        | ScriptOpcode::NpcWalkTrigger
        | ScriptOpcode::NpcSetTimer
        | ScriptOpcode::NpcStat
        | ScriptOpcode::NpcStatAdd
        | ScriptOpcode::NpcStatHeal
        | ScriptOpcode::NpcStatSub
        | ScriptOpcode::NpcTele
        | ScriptOpcode::NpcType
        | ScriptOpcode::NpcUid
        | ScriptOpcode::SpotAnimNpc
        | ScriptOpcode::NpcWalk
        | ScriptOpcode::NpcAttackRange
        | ScriptOpcode::NpcHasOp
        | ScriptOpcode::NpcArriveDelay => state.abort(format!("Unimplemented! {:?}", code)),
        // Loc ops (3000-3499)
        ScriptOpcode::LocAdd
        | ScriptOpcode::LocAngle
        | ScriptOpcode::LocAnim
        | ScriptOpcode::LocCategory
        | ScriptOpcode::LocChange
        | ScriptOpcode::LocCoord
        | ScriptOpcode::LocDel
        | ScriptOpcode::LocFind
        | ScriptOpcode::LocFindAllZone
        | ScriptOpcode::LocFindNext
        | ScriptOpcode::LocName
        | ScriptOpcode::LocParam
        | ScriptOpcode::LocShape
        | ScriptOpcode::LocType => state.abort(format!("Unimplemented! {:?}", code)),
        // Obj ops (3500-4000)
        ScriptOpcode::ObjAdd => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjAddAll => obj_ops::obj_addall(engine, state),
        ScriptOpcode::ObjCoord => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjCount => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjDel => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjName => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjParam => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjTakeItem => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjType => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::ObjFind => state.abort(format!("Unimplemented! {:?}", code)),
        // Npc config ops (4000-4099)
        ScriptOpcode::NcCategory
        | ScriptOpcode::NcDebugname
        | ScriptOpcode::NcDesc
        | ScriptOpcode::NcName
        | ScriptOpcode::NcOp
        | ScriptOpcode::NcParam => state.abort(format!("Unimplemented! {:?}", code)),
        // Loc config ops (4100-4199)
        ScriptOpcode::LcCategory
        | ScriptOpcode::LcDebugname
        | ScriptOpcode::LcDesc
        | ScriptOpcode::LcName
        | ScriptOpcode::LcOp
        | ScriptOpcode::LcParam
        | ScriptOpcode::LcWidth
        | ScriptOpcode::LcLength => state.abort(format!("Unimplemented! {:?}", code)),
        // Obj config ops (4200-4299)
        ScriptOpcode::OcCategory
        | ScriptOpcode::OcCert
        | ScriptOpcode::OcCost
        | ScriptOpcode::OcDebugname
        | ScriptOpcode::OcDesc
        | ScriptOpcode::OcIop
        | ScriptOpcode::OcMembers
        | ScriptOpcode::OcName
        | ScriptOpcode::OcOp
        | ScriptOpcode::OcParam
        | ScriptOpcode::OcStackable
        | ScriptOpcode::OcTradeable
        | ScriptOpcode::OcUncert
        | ScriptOpcode::OcWearPos2
        | ScriptOpcode::OcWearPos3
        | ScriptOpcode::OcWearPos
        | ScriptOpcode::OcWeight => state.abort(format!("Unimplemented! {:?}", code)),
        // Inventory ops (4300-4399)
        ScriptOpcode::InvAllStock
        | ScriptOpcode::InvSize
        | ScriptOpcode::InvStockBase
        | ScriptOpcode::InvAdd
        | ScriptOpcode::InvChangeSlot
        | ScriptOpcode::InvClear
        | ScriptOpcode::InvDel
        | ScriptOpcode::InvDelSlot
        | ScriptOpcode::InvDropItem
        | ScriptOpcode::InvDropSlot
        | ScriptOpcode::InvFreespace
        | ScriptOpcode::InvGetNum
        | ScriptOpcode::InvGetObj
        | ScriptOpcode::InvItemSpace
        | ScriptOpcode::InvItemSpace2
        | ScriptOpcode::InvMoveFromSlot
        | ScriptOpcode::InvMoveToSlot
        | ScriptOpcode::BothMoveInv
        | ScriptOpcode::InvMoveItem
        | ScriptOpcode::InvMoveItemCert
        | ScriptOpcode::InvMoveItemUncert
        | ScriptOpcode::InvSetSlot
        | ScriptOpcode::InvTotal
        | ScriptOpcode::InvTotalCat
        | ScriptOpcode::InvTransmit
        | ScriptOpcode::InvOtherTransmit
        | ScriptOpcode::InvStopTransmit
        | ScriptOpcode::BothDropSlot
        | ScriptOpcode::InvDropAll
        | ScriptOpcode::InvTotalParam
        | ScriptOpcode::InvTotalParamStack => state.abort(format!("Unimplemented! {:?}", code)),
        // Enum ops (4400-4499)
        ScriptOpcode::Enum | ScriptOpcode::EnumGetOutputCount => {
            state.abort(format!("Unimplemented! {:?}", code))
        }
        // String ops (4500-4599)
        ScriptOpcode::AppendNum => string_ops::append_num(state),
        ScriptOpcode::Append => string_ops::append(state),
        ScriptOpcode::AppendSignNum => string_ops::append_signnum(state),
        ScriptOpcode::Lowercase => string_ops::lowercase(state),
        ScriptOpcode::TextGender => state.protect(&ScriptState::ACTIVE_PLAYER, |state| {
            string_ops::text_gender(state)
        }),
        ScriptOpcode::ToString => string_ops::to_string(state),
        ScriptOpcode::Compare => string_ops::compare(state),
        ScriptOpcode::TextSwitch => string_ops::text_switch(state),
        ScriptOpcode::AppendChar => string_ops::append_char(state),
        ScriptOpcode::StringLength => string_ops::string_length(state),
        ScriptOpcode::SubString => string_ops::substring(state),
        ScriptOpcode::StringIndexOfChar => string_ops::string_indexof_char(state),
        ScriptOpcode::StringIndexOfString => string_ops::string_indexof_string(state),
        // Number ops (4600-4699)
        ScriptOpcode::Add => math_ops::add(state),
        ScriptOpcode::Sub => math_ops::sub(state),
        ScriptOpcode::Multiply => math_ops::multiply(state),
        ScriptOpcode::Divide => math_ops::divide(state),
        ScriptOpcode::Random => math_ops::random(state),
        ScriptOpcode::RandomInc => math_ops::randominc(state),
        ScriptOpcode::Interpolate => math_ops::interpolate(state),
        ScriptOpcode::AddPercent => math_ops::addpercent(state),
        ScriptOpcode::SetBit => math_ops::setbit(state),
        ScriptOpcode::ClearBit => math_ops::clearbit(state),
        ScriptOpcode::TestBit => math_ops::testbit(state),
        ScriptOpcode::Modulo => math_ops::modulo(state),
        ScriptOpcode::Pow => math_ops::pow(state),
        ScriptOpcode::InvPow => math_ops::invpow(state),
        ScriptOpcode::And => math_ops::and(state),
        ScriptOpcode::Or => math_ops::or(state),
        ScriptOpcode::Min => math_ops::min(state),
        ScriptOpcode::Max => math_ops::max(state),
        ScriptOpcode::Scale => math_ops::scale(state),
        ScriptOpcode::BitCount => math_ops::bitcount(state),
        ScriptOpcode::ToggleBit => math_ops::togglebit(state),
        ScriptOpcode::SetBitRange => math_ops::setbit_range(state),
        ScriptOpcode::ClearBitRange => math_ops::clearbit_range(state),
        ScriptOpcode::GetBitRange => math_ops::getbit_range(state),
        ScriptOpcode::SetBitRangeToInt => math_ops::setbit_range_toint(state),
        ScriptOpcode::SinDeg => math_ops::sin_deg(state),
        ScriptOpcode::CosDeg => math_ops::cos_deg(state),
        ScriptOpcode::Atan2Deg => math_ops::atan2_deg(state),
        ScriptOpcode::Abs => math_ops::abs(state),
        // DB ops (7500-7599)
        ScriptOpcode::DbFindWithCount
        | ScriptOpcode::DbFindNext
        | ScriptOpcode::DbGetField
        | ScriptOpcode::DbGetFieldCount
        | ScriptOpcode::DbListAllWithCount
        | ScriptOpcode::DbGetRowTable
        | ScriptOpcode::DbFindByIndex
        | ScriptOpcode::DbFindRefineWithCount
        | ScriptOpcode::DbFind
        | ScriptOpcode::DbFindRefine
        | ScriptOpcode::DbListAll => state.abort(format!("Unimplemented! {:?}", code)),
        // Debug ops (10000-11000)
        ScriptOpcode::Error => state.abort(format!("Unimplemented! {:?}", code)),
        ScriptOpcode::MapProduction => debug_ops::map_production(engine, state),
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
        _ => state.abort(format!("Unrecognised ops code: {:?}", code)),
    }
}
