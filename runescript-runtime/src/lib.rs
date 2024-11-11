use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::player_ops::{find_uid};
use crate::script::{ScriptExecutionState, ScriptOpcode, ScriptState};

pub mod script;
pub mod player_ops;

#[wasm_bindgen]
extern {
    pub fn engine_find_uid(uid: i32) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn execute(state: &mut ScriptState, code: ScriptOpcode) -> ScriptExecutionState {
    if let Err(_) = push_script(state, code) {
        state.set_execution_state(ScriptExecutionState::Aborted);
    }
    return state.get_execution_state();
}

#[wasm_bindgen]
pub fn push_script(state: &mut ScriptState, code: ScriptOpcode) -> Result<(), String> {
    // info!("{:?}", code);
    // log(format!("{:?}", code).as_str());
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
        | ScriptOpcode::EndCoreOps => Ok(()),
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
        | ScriptOpcode::MapMulti => Ok(()),
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
        | ScriptOpcode::LastCoord => find_uid(state),
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
        | ScriptOpcode::NpcArriveDelay => Err("Not implemented".to_string()),
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
        | ScriptOpcode::LocType => Err("Not implemented".to_string()),
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
        | ScriptOpcode::ObjFind => Err("Not implemented".to_string()),
        // Npc config ops (4000-4099)
        ScriptOpcode::NcCategory
        | ScriptOpcode::NcDebugname
        | ScriptOpcode::NcDesc
        | ScriptOpcode::NcName
        | ScriptOpcode::NcOp
        | ScriptOpcode::NcParam => Err("Not implemented".to_string()),
        // Loc config ops (4100-4199)
        ScriptOpcode::LcCategory
        | ScriptOpcode::LcDebugname
        | ScriptOpcode::LcDesc
        | ScriptOpcode::LcName
        | ScriptOpcode::LcOp
        | ScriptOpcode::LcParam
        | ScriptOpcode::LcWidth
        | ScriptOpcode::LcLength => Err("Not implemented".to_string()),
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
        | ScriptOpcode::OcWeight => Ok(()),
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
        | ScriptOpcode::InvTotalParamStack => Err("Not implemented".to_string()),
        // Enum ops (4400-4499)
        ScriptOpcode::Enum | ScriptOpcode::EnumGetOutputCount => {
            Err("Not implemented".to_string())
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
        | ScriptOpcode::StringIndexOfString => Ok(()),
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
        | ScriptOpcode::Abs => Ok(()),
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
        | ScriptOpcode::DbListAll => Err("Not implemented".to_string()),
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
        | ScriptOpcode::MapLastBandwidthOut => Err("Not implemented".to_string()),
    }
}