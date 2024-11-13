use crate::bits::Bits;
use crate::coord_grid::CoordGrid;
use crate::core_ops::{perform_core_operation, VarPlayerType};
use crate::debug_ops::perform_debug_operation;
use crate::math_ops::perform_math_operation;
use crate::obj_ops::{perform_obj_operation, Obj, ObjType};
use crate::player_ops::{perform_player_operation, Player};
use crate::script::{ScriptExecutionState, ScriptFile, ScriptOpcode, ScriptState};
use crate::server_ops::perform_server_operation;
use crate::string_ops::perform_string_operation;
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
                throw_error(
                    self,
                    format!("Invalid program counter: {}, max expected: {}", pc, len).as_str(),
                );
                return;
            }
            if opcount > 500_000 {
                throw_error(self, "Too many instructions!");
                return;
            }
            let pc: isize = self.get_pc() + 1;
            let code: u16 = codes[pc as usize];
            self.set_opcount(opcount + 1);
            self.set_pc(pc);
            if let Err(error) = unsafe { push_script(engine, self, ScriptOpcode::from(code)) } {
                throw_error(self, error.as_str());
                return;
            }
        }
    }
}

pub fn throw_error(state: &mut ScriptState, message: &str) {
    let file_name = state.get_script_file_name();
    let script_name = state.get_script_name();
    let line_number = state.script_line_number(state.get_pc());

    state.set_execution_state(ScriptExecutionState::Aborted);
    state.set_error(format!(
        r#"
        Script Error: {message}
        File: {file_name}

        1. {name} - {file_name}:{line}"#,
        message = message,
        file_name = file_name,
        name = script_name,
        line = line_number
    ));
}

#[inline(always)]
pub unsafe fn push_script(
    engine: &Engine,
    state: &mut ScriptState,
    code: ScriptOpcode,
) -> Result<(), String> {
    // info!("{:?}", code);
    log(format!("{:?}", code).as_str());
    log(format!("{:?}", state.get_script().name()).as_str());
    match code {
        // Core language ops (0-99)
        ScriptOpcode::PushConstantInt
        | ScriptOpcode::PushVarp
        | ScriptOpcode::PopVarp
        | ScriptOpcode::PushConstantString
        | ScriptOpcode::PushVarn
        | ScriptOpcode::PopVarn
        | ScriptOpcode::Branch
        | ScriptOpcode::BranchNot
        | ScriptOpcode::BranchEquals
        | ScriptOpcode::BranchLessThan
        | ScriptOpcode::BranchGreaterThan
        | ScriptOpcode::PushVars
        | ScriptOpcode::PopVars
        | ScriptOpcode::Return
        | ScriptOpcode::GoSub
        | ScriptOpcode::Jump
        | ScriptOpcode::Switch
        | ScriptOpcode::PushVarbit
        | ScriptOpcode::PopVarbit
        | ScriptOpcode::BranchLessThanOrEquals
        | ScriptOpcode::BranchGreaterThanOrEquals
        | ScriptOpcode::PushIntLocal
        | ScriptOpcode::PopIntLocal
        | ScriptOpcode::PushStringLocal
        | ScriptOpcode::PopStringLocal
        | ScriptOpcode::JoinString
        | ScriptOpcode::PopIntDiscard
        | ScriptOpcode::PopStringDiscard
        | ScriptOpcode::GoSubWithParams
        | ScriptOpcode::JumpWithParams
        | ScriptOpcode::PushVarcInt
        | ScriptOpcode::PopVarcInt
        | ScriptOpcode::DefineArray
        | ScriptOpcode::PushArrayInt
        | ScriptOpcode::PopArrayInt
        | ScriptOpcode::EndCoreOps => perform_core_operation(engine, state, code),
        // Server ops (1000-1999)
        ScriptOpcode::CoordX
        | ScriptOpcode::CoordY
        | ScriptOpcode::CoordZ
        | ScriptOpcode::Distance
        | ScriptOpcode::HuntAll
        | ScriptOpcode::HuntNext
        | ScriptOpcode::InZone
        | ScriptOpcode::LineOfSight
        | ScriptOpcode::LineOfWalk
        | ScriptOpcode::MapBlocked
        | ScriptOpcode::MapIndoors
        | ScriptOpcode::MapClock
        | ScriptOpcode::MapLocAddUnsafe
        | ScriptOpcode::MapMembers
        | ScriptOpcode::MapPlayerCount
        | ScriptOpcode::MapFindSquare
        | ScriptOpcode::MoveCoord
        | ScriptOpcode::PlayerCount
        | ScriptOpcode::ProjAnimMap
        | ScriptOpcode::ProjAnimNpc
        | ScriptOpcode::ProjAnimPl
        | ScriptOpcode::SeqLength
        | ScriptOpcode::SplitGet
        | ScriptOpcode::SplitGetAnim
        | ScriptOpcode::SplitInit
        | ScriptOpcode::SplitLineCount
        | ScriptOpcode::SplitPageCount
        | ScriptOpcode::SpotAnimMap
        | ScriptOpcode::StatRandom
        | ScriptOpcode::StructParam
        | ScriptOpcode::WorldDelay
        | ScriptOpcode::NpcsCount
        | ScriptOpcode::ZonesCount
        | ScriptOpcode::LocsCount
        | ScriptOpcode::ObjsCount
        | ScriptOpcode::MapMulti => perform_server_operation(engine, state, code),
        // Player ops (2000-2499)
        ScriptOpcode::AllowDesign
        | ScriptOpcode::Anim
        | ScriptOpcode::BasReadyAnim
        | ScriptOpcode::BasRunning
        | ScriptOpcode::BasTurnOnSpot
        | ScriptOpcode::BasWalkB
        | ScriptOpcode::BasWalkF
        | ScriptOpcode::BasWalkL
        | ScriptOpcode::BasWalkR
        | ScriptOpcode::BufferFull
        | ScriptOpcode::BuildAppearance
        | ScriptOpcode::Busy
        | ScriptOpcode::CamLookAt
        | ScriptOpcode::CamMoveTo
        | ScriptOpcode::CamReset
        | ScriptOpcode::CamShake
        | ScriptOpcode::ClearQueue
        | ScriptOpcode::ClearSoftTimer
        | ScriptOpcode::ClearTimer
        | ScriptOpcode::GetTimer
        | ScriptOpcode::Coord
        | ScriptOpcode::Damage
        | ScriptOpcode::Displayname
        | ScriptOpcode::FaceSquare
        | ScriptOpcode::FindUid
        | ScriptOpcode::Gender
        | ScriptOpcode::GetQueue
        | ScriptOpcode::StatAdvance
        | ScriptOpcode::HeadiconsGet
        | ScriptOpcode::HeadiconsSet
        | ScriptOpcode::HealEnergy
        | ScriptOpcode::HintCoord
        | ScriptOpcode::HintNpc
        | ScriptOpcode::HintPlayer
        | ScriptOpcode::HintStop
        | ScriptOpcode::IfClose
        | ScriptOpcode::TutClose
        | ScriptOpcode::IfMultiZone
        | ScriptOpcode::IfOpenChat
        | ScriptOpcode::TutOpen
        | ScriptOpcode::IfOpenMain
        | ScriptOpcode::IfOpenMainSide
        | ScriptOpcode::IfOpenSide
        | ScriptOpcode::IfSetAnim
        | ScriptOpcode::IfSetColour
        | ScriptOpcode::IfSetHide
        | ScriptOpcode::IfSetModel
        | ScriptOpcode::IfSetRecol
        | ScriptOpcode::IfSetNpcHead
        | ScriptOpcode::IfSetObject
        | ScriptOpcode::IfSetPlayerHead
        | ScriptOpcode::IfSetPosition
        | ScriptOpcode::IfSetResumeButtons
        | ScriptOpcode::IfSetTab
        | ScriptOpcode::IfSetTabActive
        | ScriptOpcode::TutFlash
        | ScriptOpcode::IfSetText
        | ScriptOpcode::LastLoginInfo
        | ScriptOpcode::LastCom
        | ScriptOpcode::LastInt
        | ScriptOpcode::LastItem
        | ScriptOpcode::LastSlot
        | ScriptOpcode::LastTargetSlot
        | ScriptOpcode::LastUseItem
        | ScriptOpcode::LastUseSlot
        | ScriptOpcode::LongQueue
        | ScriptOpcode::Mes
        | ScriptOpcode::MidiJingle
        | ScriptOpcode::MidiSong
        | ScriptOpcode::Name
        | ScriptOpcode::PApRange
        | ScriptOpcode::PArriveDelay
        | ScriptOpcode::PCountDialog
        | ScriptOpcode::PDelay
        | ScriptOpcode::PExactMove
        | ScriptOpcode::PFindUid
        | ScriptOpcode::PLocMerge
        | ScriptOpcode::PLogout
        | ScriptOpcode::POpHeld
        | ScriptOpcode::POpLoc
        | ScriptOpcode::POpNpc
        | ScriptOpcode::POpNpcT
        | ScriptOpcode::POpObj
        | ScriptOpcode::POpPlayer
        | ScriptOpcode::POpPlayerT
        | ScriptOpcode::PPauseButton
        | ScriptOpcode::PStopAction
        | ScriptOpcode::PTeleJump
        | ScriptOpcode::PTeleport
        | ScriptOpcode::PWalk
        | ScriptOpcode::PlayerFindAllZone
        | ScriptOpcode::PlayerFindNext
        | ScriptOpcode::Queue
        | ScriptOpcode::Say
        | ScriptOpcode::WalkTrigger
        | ScriptOpcode::SetTimer
        | ScriptOpcode::SoftTimer
        | ScriptOpcode::SoundSynth
        | ScriptOpcode::SpotAnimPl
        | ScriptOpcode::StaffModLevel
        | ScriptOpcode::Stat
        | ScriptOpcode::StatAdd
        | ScriptOpcode::StatBase
        | ScriptOpcode::StatHeal
        | ScriptOpcode::StatSub
        | ScriptOpcode::StrongQueue
        | ScriptOpcode::Uid
        | ScriptOpcode::WeakQueue
        | ScriptOpcode::IfOpenMainOverlay
        | ScriptOpcode::AfkEvent
        | ScriptOpcode::LowMemory
        | ScriptOpcode::SetIdkit
        | ScriptOpcode::PClearPendingAction
        | ScriptOpcode::GetWalkTrigger
        | ScriptOpcode::Busy2
        | ScriptOpcode::FindHero
        | ScriptOpcode::BothHeroPoints
        | ScriptOpcode::SetGender
        | ScriptOpcode::SetSkinColour
        | ScriptOpcode::PAnimProtect
        | ScriptOpcode::RunEnergy
        | ScriptOpcode::Weight
        | ScriptOpcode::LastCoord => perform_player_operation(engine, state, code),
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
        | ScriptOpcode::NpcArriveDelay => Err(format!("Unimplemented! {:?}", code)),
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
        | ScriptOpcode::LocType => Err(format!("Unimplemented! {:?}", code)),
        // Obj ops (3500-4000)
        ScriptOpcode::ObjAdd
        | ScriptOpcode::ObjAddAll
        | ScriptOpcode::ObjCoord
        | ScriptOpcode::ObjCount
        | ScriptOpcode::ObjDel
        | ScriptOpcode::ObjName
        | ScriptOpcode::ObjParam
        | ScriptOpcode::ObjTakeItem
        | ScriptOpcode::ObjType
        | ScriptOpcode::ObjFind => perform_obj_operation(engine, state, code),
        // Npc config ops (4000-4099)
        ScriptOpcode::NcCategory
        | ScriptOpcode::NcDebugname
        | ScriptOpcode::NcDesc
        | ScriptOpcode::NcName
        | ScriptOpcode::NcOp
        | ScriptOpcode::NcParam => Err(format!("Unimplemented! {:?}", code)),
        // Loc config ops (4100-4199)
        ScriptOpcode::LcCategory
        | ScriptOpcode::LcDebugname
        | ScriptOpcode::LcDesc
        | ScriptOpcode::LcName
        | ScriptOpcode::LcOp
        | ScriptOpcode::LcParam
        | ScriptOpcode::LcWidth
        | ScriptOpcode::LcLength => Err(format!("Unimplemented! {:?}", code)),
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
        | ScriptOpcode::OcWeight => Err(format!("Unimplemented! {:?}", code)),
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
        | ScriptOpcode::InvTotalParamStack => Err(format!("Unimplemented! {:?}", code)),
        // Enum ops (4400-4499)
        ScriptOpcode::Enum | ScriptOpcode::EnumGetOutputCount => {
            Err(format!("Unimplemented! {:?}", code))
        }
        // String ops (4500-4599)
        ScriptOpcode::AppendNum
        | ScriptOpcode::Append
        | ScriptOpcode::AppendSignNum
        | ScriptOpcode::Lowercase
        | ScriptOpcode::TextGender
        | ScriptOpcode::ToString
        | ScriptOpcode::Compare
        | ScriptOpcode::TextSwitch
        | ScriptOpcode::AppendChar
        | ScriptOpcode::StringLength
        | ScriptOpcode::SubString
        | ScriptOpcode::StringIndexOfChar
        | ScriptOpcode::StringIndexOfString => perform_string_operation(state, code),
        // Number ops (4600-4699)
        ScriptOpcode::Add
        | ScriptOpcode::Sub
        | ScriptOpcode::Multiply
        | ScriptOpcode::Divide
        | ScriptOpcode::Random
        | ScriptOpcode::RandomInc
        | ScriptOpcode::Interpolate
        | ScriptOpcode::AddPercent
        | ScriptOpcode::SetBit
        | ScriptOpcode::ClearBit
        | ScriptOpcode::TestBit
        | ScriptOpcode::Modulo
        | ScriptOpcode::Pow
        | ScriptOpcode::InvPow
        | ScriptOpcode::And
        | ScriptOpcode::Or
        | ScriptOpcode::Min
        | ScriptOpcode::Max
        | ScriptOpcode::Scale
        | ScriptOpcode::BitCount
        | ScriptOpcode::ToggleBit
        | ScriptOpcode::SetBitRange
        | ScriptOpcode::ClearBitRange
        | ScriptOpcode::GetBitRange
        | ScriptOpcode::SetBitRangeToInt
        | ScriptOpcode::SinDeg
        | ScriptOpcode::CosDeg
        | ScriptOpcode::Atan2Deg
        | ScriptOpcode::Abs => perform_math_operation(state, code),
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
        | ScriptOpcode::DbListAll => Err(format!("Unimplemented! {:?}", code)),
        // Debug ops (10000-11000)
        ScriptOpcode::Error
        | ScriptOpcode::MapProduction
        | ScriptOpcode::MapLastClock
        | ScriptOpcode::MapLastWorld
        | ScriptOpcode::MapLastClientIn
        | ScriptOpcode::MapLastNpc
        | ScriptOpcode::MapLastPlayer
        | ScriptOpcode::MapLastLogout
        | ScriptOpcode::MapLastLogin
        | ScriptOpcode::MapLastZone
        | ScriptOpcode::MapLastClientOut
        | ScriptOpcode::MapLastCleanup
        | ScriptOpcode::MapLastBandwidthIn
        | ScriptOpcode::MapLastBandwidthOut => perform_debug_operation(engine, state, code),
    }
}
