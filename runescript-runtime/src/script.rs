use crate::loc_ops::Loc;
use crate::npc_ops::Npc;
use crate::obj_ops::Obj;
use crate::player_ops::Player;
use js_sys::{Array, Map};
use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[repr(u16)]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum ScriptOpcode {
    // Core language ops (0-99)
    PushConstantInt = 0,    // official, see cs2
    PushVarp = 1,           // official, see cs2
    PopVarp = 2,            // official, see cs2
    PushConstantString = 3, // official, see cs2
    PushVarn = 4,
    PopVarn = 5,
    Branch = 6,             // official, see cs2
    BranchNot = 7,          // official, see cs2
    BranchEquals = 8,       // official, see cs2
    BranchLessThan = 9,     // official, see cs2
    BranchGreaterThan = 10, // official, see cs2
    PushVars = 11,
    PopVars = 12,
    Return = 21, // official, see cs2
    GoSub = 22,
    Jump = 23,
    Switch = 24,
    PushVarbit = 25,
    PopVarbit = 26,
    BranchLessThanOrEquals = 31,    // official, see cs2
    BranchGreaterThanOrEquals = 32, // official, see cs2
    PushIntLocal = 33,              // official, see cs2
    PopIntLocal = 34,               // official, see cs2
    PushStringLocal = 35,           // official, see cs2
    PopStringLocal = 36,            // official, see cs2
    JoinString = 37,                // official, see cs2
    PopIntDiscard = 38,             // official, see cs2
    PopStringDiscard = 39,          // official, see cs2
    GoSubWithParams = 40,           // official, see cs2
    JumpWithParams = 41,            // official, see cs2
    PushVarcInt = 42,
    PopVarcInt = 43,
    DefineArray = 44,  // official, see cs2
    PushArrayInt = 45, // official, see cs2
    PopArrayInt = 46,  // official, see cs2
    EndCoreOps = 100,
    // Server ops (1000-1999)
    CoordX = 1000, // official, see cs2
    CoordY = 1001, // official, see cs2
    CoordZ = 1002, // official, see cs2
    Distance = 1003,
    HuntAll = 1004,
    HuntNext = 1005, // official
    InZone = 1006,   // official
    LineOfSight = 1007,
    LineOfWalk = 1008,
    MapBlocked = 1009, // official
    MapIndoors = 1010,
    MapClock = 1011,        // official
    MapLocAddUnsafe = 1012, // official
    MapMembers = 1013,      // official
    MapPlayerCount = 1014,  // official, see giant dwarf cutscene
    MapFindSquare = 1015,   // official
    MoveCoord = 1016,       // official
    PlayerCount = 1017,
    ProjAnimMap = 1018,
    ProjAnimNpc = 1019,
    ProjAnimPl = 1020,
    SeqLength = 1021, // official
    SplitGet = 1022,
    SplitGetAnim = 1023,
    SplitInit = 1024, // official
    SplitLineCount = 1025,
    SplitPageCount = 1026, // official
    SpotAnimMap = 1027,
    StatRandom = 1028,
    StructParam = 1029,
    WorldDelay = 1030, // official
    NpcsCount = 1031,
    ZonesCount = 1032,
    LocsCount = 1033,
    ObjsCount = 1034,
    MapMulti = 1035,
    // Player ops (2000-2499)
    AllowDesign = 2000,
    Anim = 2001,
    BasReadyAnim = 2002,
    BasRunning = 2003,
    BasTurnOnSpot = 2004,
    BasWalkB = 2005,
    BasWalkF = 2006,
    BasWalkL = 2007,
    BasWalkR = 2008,
    BufferFull = 2009,      // official
    BuildAppearance = 2010, // official
    Busy = 2011,            // official
    CamLookAt = 2012,       // official
    CamMoveTo = 2013,       // official
    CamReset = 2014,        // official
    CamShake = 2015,        // official, see server packets
    ClearQueue = 2016,      // official
    ClearSoftTimer = 2017,
    ClearTimer = 2018,
    GetTimer = 2019,
    Coord = 2020, // official
    Damage = 2021,
    Displayname = 2022, // official, joke reply
    FaceSquare = 2023,  // official
    FindUid = 2024,     // official
    Gender = 2025,
    GetQueue = 2026, // official
    StatAdvance = 2027,
    HeadiconsGet = 2028,
    HeadiconsSet = 2029,
    HealEnergy = 2030, // official
    HintCoord = 2031,
    HintNpc = 2032,
    HintPlayer = 2033,
    HintStop = 2034,
    IfClose = 2035, // official
    TutClose = 2036,
    IfMultiZone = 2037,
    IfOpenChat = 2038,
    TutOpen = 2039,
    IfOpenMain = 2040,
    IfOpenMainSide = 2041,
    IfOpenSide = 2042,
    IfSetAnim = 2043,   // official
    IfSetColour = 2044, // official
    IfSetHide = 2045,   // official
    IfSetModel = 2046,  // official
    IfSetRecol = 2047,
    IfSetNpcHead = 2048,    // official
    IfSetObject = 2049,     // official
    IfSetPlayerHead = 2050, // official
    IfSetPosition = 2051,   // official
    IfSetResumeButtons = 2052,
    IfSetTab = 2053,
    IfSetTabActive = 2054,
    TutFlash = 2055,
    IfSetText = 2056, // official
    LastLoginInfo = 2057,
    LastCom = 2058,
    LastInt = 2059, // official
    LastItem = 2060,
    LastSlot = 2061, // official
    LastTargetSlot = 2062,
    LastUseItem = 2063,  // official
    LastUseSlot = 2064,  // official
    LongQueue = 2065,    // official
    Mes = 2066,          // official
    MidiJingle = 2067,   // official, see cs2
    MidiSong = 2068,     // official, see cs2
    Name = 2069,         // official, joke reply
    PApRange = 2070,     // official
    PArriveDelay = 2071, // official
    PCountDialog = 2072, // official
    PDelay = 2073,       // official
    PExactMove = 2074,   // official
    PFindUid = 2075,     // official
    PLocMerge = 2076,    // official
    PLogout = 2077,
    POpHeld = 2078, // official
    POpLoc = 2079,  // official
    POpNpc = 2080,  // official
    POpNpcT = 2081, // official
    POpObj = 2082,
    POpPlayer = 2083,
    POpPlayerT = 2084,   // official
    PPauseButton = 2085, // official
    PStopAction = 2086,  // official
    PTeleJump = 2087,    // official
    PTeleport = 2088,
    PWalk = 2089,             // official
    PlayerFindAllZone = 2090, // todo: replace with huntall
    PlayerFindNext = 2091,    // todo: replace with huntnext
    Queue = 2092,             // official
    Say = 2093,               // official
    WalkTrigger = 2094,       // official
    SetTimer = 2095,
    SoftTimer = 2096,  // official
    SoundSynth = 2097, // official, newspost
    SpotAnimPl = 2098,
    StaffModLevel = 2099, // official
    Stat = 2100,          // official
    StatAdd = 2101,
    StatBase = 2102, // official
    StatHeal = 2103, // official
    StatSub = 2104,
    StrongQueue = 2105,
    Uid = 2106,       // official
    WeakQueue = 2107, // official
    IfOpenMainOverlay = 2108,
    AfkEvent = 2109,
    LowMemory = 2110,
    SetIdkit = 2111,
    PClearPendingAction = 2112, // official
    GetWalkTrigger = 2113,      // official
    Busy2 = 2114,               // official
    FindHero = 2115,            // official
    BothHeroPoints = 2116,      // official
    SetGender = 2117,
    SetSkinColour = 2118,
    PAnimProtect = 2119,
    RunEnergy = 2120,
    Weight = 2121,
    LastCoord = 2122,
    // Npc ops (2500-2999)
    NpcAdd = 2500,      // official
    NpcAnim = 2501,     // official, newspost
    NpcBaseStat = 2502, // official
    NpcCategory = 2503, // official
    NpcChangeType = 2504,
    NpcCoord = 2505, // official
    NpcDamage = 2506,
    NpcDel = 2507,        // official
    NpcDelay = 2508,      // official
    NpcFaceSquare = 2509, // official
    NpcFind = 2510,       // official
    NpcFindAllAny = 2511, // official
    NpcFindAll = 2512,
    NpcFindExact = 2513, // official
    NpcFindHero = 2514,  // official
    NpcFindAllZone = 2515,
    NpcFindNext = 2516,
    NpcFindUid = 2517,
    NpcGetMode = 2518,
    NpcHeroPoints = 2519, // official
    NpcName = 2520,
    NpcParam = 2521,   // official
    NpcQueue = 2522,   // official
    NpcRange = 2523,   // official
    NpcSay = 2524,     // official
    NpcHuntAll = 2525, // official
    NpcHuntNext = 2526,
    NpcSetHunt = 2527,     // official
    NpcSetHuntMode = 2528, // official
    NpcSetMode = 2529,     // official
    NpcWalkTrigger = 2530, // official
    NpcSetTimer = 2531,
    NpcStat = 2532,
    NpcStatAdd = 2533,
    NpcStatHeal = 2534, // official
    NpcStatSub = 2535,
    NpcTele = 2536,
    NpcType = 2537, // official
    NpcUid = 2538,
    SpotAnimNpc = 2539,
    NpcWalk = 2540,
    NpcAttackRange = 2541, // official
    NpcHasOp = 2542,       // official
    NpcArriveDelay = 2543,
    // Loc ops (3000-3499)
    LocAdd = 3000,      // official
    LocAngle = 3001,    // official
    LocAnim = 3002,     // official
    LocCategory = 3003, // official
    LocChange = 3004,
    LocCoord = 3005,       // official
    LocDel = 3006,         // official
    LocFind = 3007,        // official
    LocFindAllZone = 3008, // official
    LocFindNext = 3009,    // official
    LocName = 3010,
    LocParam = 3011, // official
    LocShape = 3012,
    LocType = 3013, // official
    // Obj ops (3500-4000)
    ObjAdd = 3500,
    ObjAddAll = 3501,
    ObjCoord = 3502,
    ObjCount = 3503,
    ObjDel = 3504,
    ObjName = 3505,
    ObjParam = 3506,
    ObjTakeItem = 3507,
    ObjType = 3508,
    ObjFind = 3509,
    // Npc config ops (4000-4099)
    NcCategory = 4000,
    NcDebugname = 4001,
    NcDesc = 4002,
    NcName = 4003,
    NcOp = 4004,
    NcParam = 4005,
    // Loc config ops (4100-4199)
    LcCategory = 4100,
    LcDebugname = 4101,
    LcDesc = 4102,
    LcName = 4103,
    LcOp = 4104,
    LcParam = 4105,
    LcWidth = 4106,
    LcLength = 4107,
    // Obj config ops (4200-4299)
    OcCategory = 4200, // official
    OcCert = 4201,     // official, see cs2
    OcCost = 4202,     // official, see cs2
    OcDebugname = 4203,
    OcDesc = 4204,      // official
    OcIop = 4205,       // official, see cs2
    OcMembers = 4206,   // official
    OcName = 4207,      // official
    OcOp = 4208,        // official, see cs2
    OcParam = 4209,     // official
    OcStackable = 4210, // official, see cs2
    OcTradeable = 4211,
    OcUncert = 4212, // official, see cs2
    OcWearPos2 = 4213,
    OcWearPos3 = 4214,
    OcWearPos = 4215,
    OcWeight = 4216,
    // Inventory ops (4300-4399)
    InvAllStock = 4300,
    InvSize = 4301, // official
    InvStockBase = 4302,
    InvAdd = 4303,        // official
    InvChangeSlot = 4304, // official
    InvClear = 4305,
    InvDel = 4306, // official
    InvDelSlot = 4307,
    InvDropItem = 4308,
    InvDropSlot = 4309,
    InvFreespace = 4310,
    InvGetNum = 4311,
    InvGetObj = 4312, // official
    InvItemSpace = 4313,
    InvItemSpace2 = 4314, // official
    InvMoveFromSlot = 4315,
    InvMoveToSlot = 4316,     // official
    BothMoveInv = 4317,       // official
    InvMoveItem = 4318,       // official
    InvMoveItemCert = 4319,   // official
    InvMoveItemUncert = 4320, // official
    InvSetSlot = 4321,        // official
    InvTotal = 4322,          // official
    InvTotalCat = 4323,
    InvTransmit = 4324,
    InvOtherTransmit = 4325,
    InvStopTransmit = 4326,
    BothDropSlot = 4327,
    InvDropAll = 4328,
    InvTotalParam = 4329,      // official, see cs2
    InvTotalParamStack = 4330, // official, see cs2
    // Enum ops (4400-4499)
    Enum = 4400,               // official
    EnumGetOutputCount = 4401, // official
    // String ops (4500-4599)
    AppendNum = 4500,           // official, see cs2
    Append = 4501,              // official, see cs2
    AppendSignNum = 4502,       // official, see cs2
    Lowercase = 4503,           // official, see cs2
    TextGender = 4504,          // official, see cs2
    ToString = 4505,            // official, see cs2
    Compare = 4506,             // official, see cs2
    TextSwitch = 4507,          // official, see cs2
    AppendChar = 4508,          // official, see cs2
    StringLength = 4509,        // official, see cs2
    SubString = 4510,           // official, see cs2
    StringIndexOfChar = 4511,   // official, see cs2
    StringIndexOfString = 4512, // official, see cs2
    // Number ops (4600-4699)
    Add = 4600,              // official, see cs2
    Sub = 4601,              // official, see cs2
    Multiply = 4602,         // official, see cs2
    Divide = 4603,           // official, see cs2
    Random = 4604,           // official, see cs2
    RandomInc = 4605,        // official, see cs2
    Interpolate = 4606,      // official, see cs2
    AddPercent = 4607,       // official, see cs2
    SetBit = 4608,           // official, see cs2
    ClearBit = 4609,         // official, see cs2
    TestBit = 4610,          // official, see cs2
    Modulo = 4611,           // official, see cs2
    Pow = 4612,              // official, see cs2
    InvPow = 4613,           // official, see cs2
    And = 4614,              // official, see cs2
    Or = 4615,               // official, see cs2
    Min = 4616,              // official, see cs2
    Max = 4617,              // official, see cs2
    Scale = 4618,            // official, see cs2
    BitCount = 4619,         // custom
    ToggleBit = 4620,        // custom
    SetBitRange = 4621,      // custom
    ClearBitRange = 4622,    // custom
    GetBitRange = 4623,      // custom
    SetBitRangeToInt = 4624, // custom
    SinDeg = 4625,           // custom
    CosDeg = 4626,           // custom
    Atan2Deg = 4627,         // custom
    Abs = 4628,              // custom
    // DB ops (7500-7599)
    DbFindWithCount = 7500,
    DbFindNext = 7501,
    DbGetField = 7502,
    DbGetFieldCount = 7503,
    DbListAllWithCount = 7504,
    DbGetRowTable = 7505,
    DbFindByIndex = 7506,
    DbFindRefineWithCount = 7507,
    DbFind = 7508,
    DbFindRefine = 7509,
    DbListAll = 7510,
    // Debug ops (10000-11000)
    Error = 10000,
    MapProduction = 10001,
    MapLastClock = 10002,        // custom
    MapLastWorld = 10003,        // custom
    MapLastClientIn = 10004,     // custom
    MapLastNpc = 10005,          // custom
    MapLastPlayer = 10006,       // custom
    MapLastLogout = 10007,       // custom
    MapLastLogin = 10008,        // custom
    MapLastZone = 10009,         // custom
    MapLastClientOut = 10010,    // custom
    MapLastCleanup = 10011,      // custom
    MapLastBandwidthIn = 10012,  // custom
    MapLastBandwidthOut = 10013, // custom
}

impl From<u16> for ScriptOpcode {
    #[inline(always)]
    fn from(code: u16) -> ScriptOpcode {
        match code {
            // Core language ops (0-99)
            0 => ScriptOpcode::PushConstantInt,
            1 => ScriptOpcode::PushVarp,
            2 => ScriptOpcode::PopVarp,
            3 => ScriptOpcode::PushConstantString,
            4 => ScriptOpcode::PushVarn,
            5 => ScriptOpcode::PopVarn,
            6 => ScriptOpcode::Branch,
            7 => ScriptOpcode::BranchNot,
            8 => ScriptOpcode::BranchEquals,
            9 => ScriptOpcode::BranchLessThan,
            10 => ScriptOpcode::BranchGreaterThan,
            11 => ScriptOpcode::PushVars,
            12 => ScriptOpcode::PopVars,
            21 => ScriptOpcode::Return,
            22 => ScriptOpcode::GoSub,
            23 => ScriptOpcode::Jump,
            24 => ScriptOpcode::Switch,
            25 => ScriptOpcode::PushVarbit,
            26 => ScriptOpcode::PopVarbit,
            31 => ScriptOpcode::BranchLessThanOrEquals,
            32 => ScriptOpcode::BranchGreaterThanOrEquals,
            33 => ScriptOpcode::PushIntLocal,
            34 => ScriptOpcode::PopIntLocal,
            35 => ScriptOpcode::PushStringLocal,
            36 => ScriptOpcode::PopStringLocal,
            37 => ScriptOpcode::JoinString,
            38 => ScriptOpcode::PopIntDiscard,
            39 => ScriptOpcode::PopStringDiscard,
            40 => ScriptOpcode::GoSubWithParams,
            41 => ScriptOpcode::JumpWithParams,
            42 => ScriptOpcode::PushVarcInt,
            43 => ScriptOpcode::PopVarcInt,
            44 => ScriptOpcode::DefineArray,
            45 => ScriptOpcode::PushArrayInt,
            46 => ScriptOpcode::PopArrayInt,
            100 => ScriptOpcode::EndCoreOps,
            // Server ops (1000-1999)
            1000 => ScriptOpcode::CoordX,
            1001 => ScriptOpcode::CoordY,
            1002 => ScriptOpcode::CoordZ,
            1003 => ScriptOpcode::Distance,
            1004 => ScriptOpcode::HuntAll,
            1005 => ScriptOpcode::HuntNext,
            1006 => ScriptOpcode::InZone,
            1007 => ScriptOpcode::LineOfSight,
            1008 => ScriptOpcode::LineOfWalk,
            1009 => ScriptOpcode::MapBlocked,
            1010 => ScriptOpcode::MapIndoors,
            1011 => ScriptOpcode::MapClock,
            1012 => ScriptOpcode::MapLocAddUnsafe,
            1013 => ScriptOpcode::MapMembers,
            1014 => ScriptOpcode::MapPlayerCount,
            1015 => ScriptOpcode::MapFindSquare,
            1016 => ScriptOpcode::MoveCoord,
            1017 => ScriptOpcode::PlayerCount,
            1018 => ScriptOpcode::ProjAnimMap,
            1019 => ScriptOpcode::ProjAnimNpc,
            1020 => ScriptOpcode::ProjAnimPl,
            1021 => ScriptOpcode::SeqLength,
            1022 => ScriptOpcode::SplitGet,
            1023 => ScriptOpcode::SplitGetAnim,
            1024 => ScriptOpcode::SplitInit,
            1025 => ScriptOpcode::SplitLineCount,
            1026 => ScriptOpcode::SplitPageCount,
            1027 => ScriptOpcode::SpotAnimMap,
            1028 => ScriptOpcode::StatRandom,
            1029 => ScriptOpcode::StructParam,
            1030 => ScriptOpcode::WorldDelay,
            1031 => ScriptOpcode::NpcsCount,
            1032 => ScriptOpcode::ZonesCount,
            1033 => ScriptOpcode::LocsCount,
            1034 => ScriptOpcode::ObjsCount,
            1035 => ScriptOpcode::MapMulti,
            // Player ops (2000-2499)
            2000 => ScriptOpcode::AllowDesign,
            2001 => ScriptOpcode::Anim,
            2002 => ScriptOpcode::BasReadyAnim,
            2003 => ScriptOpcode::BasRunning,
            2004 => ScriptOpcode::BasTurnOnSpot,
            2005 => ScriptOpcode::BasWalkB,
            2006 => ScriptOpcode::BasWalkF,
            2007 => ScriptOpcode::BasWalkL,
            2008 => ScriptOpcode::BasWalkR,
            2009 => ScriptOpcode::BufferFull,
            2010 => ScriptOpcode::BuildAppearance,
            2011 => ScriptOpcode::Busy,
            2012 => ScriptOpcode::CamLookAt,
            2013 => ScriptOpcode::CamMoveTo,
            2014 => ScriptOpcode::CamReset,
            2015 => ScriptOpcode::CamShake,
            2016 => ScriptOpcode::ClearQueue,
            2017 => ScriptOpcode::ClearSoftTimer,
            2018 => ScriptOpcode::ClearTimer,
            2019 => ScriptOpcode::GetTimer,
            2020 => ScriptOpcode::Coord,
            2021 => ScriptOpcode::Damage,
            2022 => ScriptOpcode::Displayname,
            2023 => ScriptOpcode::FaceSquare,
            2024 => ScriptOpcode::FindUid,
            2025 => ScriptOpcode::Gender,
            2026 => ScriptOpcode::GetQueue,
            2027 => ScriptOpcode::StatAdvance,
            2028 => ScriptOpcode::HeadiconsGet,
            2029 => ScriptOpcode::HeadiconsSet,
            2030 => ScriptOpcode::HealEnergy,
            2031 => ScriptOpcode::HintCoord,
            2032 => ScriptOpcode::HintNpc,
            2033 => ScriptOpcode::HintPlayer,
            2034 => ScriptOpcode::HintStop,
            2035 => ScriptOpcode::IfClose,
            2036 => ScriptOpcode::TutClose,
            2037 => ScriptOpcode::IfMultiZone,
            2038 => ScriptOpcode::IfOpenChat,
            2039 => ScriptOpcode::TutOpen,
            2040 => ScriptOpcode::IfOpenMain,
            2041 => ScriptOpcode::IfOpenMainSide,
            2042 => ScriptOpcode::IfOpenSide,
            2043 => ScriptOpcode::IfSetAnim,
            2044 => ScriptOpcode::IfSetColour,
            2045 => ScriptOpcode::IfSetHide,
            2046 => ScriptOpcode::IfSetModel,
            2047 => ScriptOpcode::IfSetRecol,
            2048 => ScriptOpcode::IfSetNpcHead,
            2049 => ScriptOpcode::IfSetObject,
            2050 => ScriptOpcode::IfSetPlayerHead,
            2051 => ScriptOpcode::IfSetPosition,
            2052 => ScriptOpcode::IfSetResumeButtons,
            2053 => ScriptOpcode::IfSetTab,
            2054 => ScriptOpcode::IfSetTabActive,
            2055 => ScriptOpcode::TutFlash,
            2056 => ScriptOpcode::IfSetText,
            2057 => ScriptOpcode::LastLoginInfo,
            2058 => ScriptOpcode::LastCom,
            2059 => ScriptOpcode::LastInt,
            2060 => ScriptOpcode::LastItem,
            2061 => ScriptOpcode::LastSlot,
            2062 => ScriptOpcode::LastTargetSlot,
            2063 => ScriptOpcode::LastUseItem,
            2064 => ScriptOpcode::LastUseSlot,
            2065 => ScriptOpcode::LongQueue,
            2066 => ScriptOpcode::Mes,
            2067 => ScriptOpcode::MidiJingle,
            2068 => ScriptOpcode::MidiSong,
            2069 => ScriptOpcode::Name,
            2070 => ScriptOpcode::PApRange,
            2071 => ScriptOpcode::PArriveDelay,
            2072 => ScriptOpcode::PCountDialog,
            2073 => ScriptOpcode::PDelay,
            2074 => ScriptOpcode::PExactMove,
            2075 => ScriptOpcode::PFindUid,
            2076 => ScriptOpcode::PLocMerge,
            2077 => ScriptOpcode::PLogout,
            2078 => ScriptOpcode::POpHeld,
            2079 => ScriptOpcode::POpLoc,
            2080 => ScriptOpcode::POpNpc,
            2081 => ScriptOpcode::POpNpcT,
            2082 => ScriptOpcode::POpObj,
            2083 => ScriptOpcode::POpPlayer,
            2084 => ScriptOpcode::POpPlayerT,
            2085 => ScriptOpcode::PPauseButton,
            2086 => ScriptOpcode::PStopAction,
            2087 => ScriptOpcode::PTeleJump,
            2088 => ScriptOpcode::PTeleport,
            2089 => ScriptOpcode::PWalk,
            2090 => ScriptOpcode::PlayerFindAllZone,
            2091 => ScriptOpcode::PlayerFindNext,
            2092 => ScriptOpcode::Queue,
            2093 => ScriptOpcode::Say,
            2094 => ScriptOpcode::WalkTrigger,
            2095 => ScriptOpcode::SetTimer,
            2096 => ScriptOpcode::SoftTimer,
            2097 => ScriptOpcode::SoundSynth,
            2098 => ScriptOpcode::SpotAnimPl,
            2099 => ScriptOpcode::StaffModLevel,
            2100 => ScriptOpcode::Stat,
            2101 => ScriptOpcode::StatAdd,
            2102 => ScriptOpcode::StatBase,
            2103 => ScriptOpcode::StatHeal,
            2104 => ScriptOpcode::StatSub,
            2105 => ScriptOpcode::StrongQueue,
            2106 => ScriptOpcode::Uid,
            2107 => ScriptOpcode::WeakQueue,
            2108 => ScriptOpcode::IfOpenMainOverlay,
            2109 => ScriptOpcode::AfkEvent,
            2110 => ScriptOpcode::LowMemory,
            2111 => ScriptOpcode::SetIdkit,
            2112 => ScriptOpcode::PClearPendingAction,
            2113 => ScriptOpcode::GetWalkTrigger,
            2114 => ScriptOpcode::Busy2,
            2115 => ScriptOpcode::FindHero,
            2116 => ScriptOpcode::BothHeroPoints,
            2117 => ScriptOpcode::SetGender,
            2118 => ScriptOpcode::SetSkinColour,
            2119 => ScriptOpcode::PAnimProtect,
            2120 => ScriptOpcode::RunEnergy,
            2121 => ScriptOpcode::Weight,
            2122 => ScriptOpcode::LastCoord,
            // Npc ops (2500-2999)
            2500 => ScriptOpcode::NpcAdd,
            2501 => ScriptOpcode::NpcAnim,
            2502 => ScriptOpcode::NpcBaseStat,
            2503 => ScriptOpcode::NpcCategory,
            2504 => ScriptOpcode::NpcChangeType,
            2505 => ScriptOpcode::NpcCoord,
            2506 => ScriptOpcode::NpcDamage,
            2507 => ScriptOpcode::NpcDel,
            2508 => ScriptOpcode::NpcDelay,
            2509 => ScriptOpcode::NpcFaceSquare,
            2510 => ScriptOpcode::NpcFind,
            2511 => ScriptOpcode::NpcFindAllAny,
            2512 => ScriptOpcode::NpcFindAll,
            2513 => ScriptOpcode::NpcFindExact,
            2514 => ScriptOpcode::NpcFindHero,
            2515 => ScriptOpcode::NpcFindAllZone,
            2516 => ScriptOpcode::NpcFindNext,
            2517 => ScriptOpcode::NpcFindUid,
            2518 => ScriptOpcode::NpcGetMode,
            2519 => ScriptOpcode::NpcHeroPoints,
            2520 => ScriptOpcode::NpcName,
            2521 => ScriptOpcode::NpcParam,
            2522 => ScriptOpcode::NpcQueue,
            2523 => ScriptOpcode::NpcRange,
            2524 => ScriptOpcode::NpcSay,
            2525 => ScriptOpcode::NpcHuntAll,
            2526 => ScriptOpcode::NpcHuntNext,
            2527 => ScriptOpcode::NpcSetHunt,
            2528 => ScriptOpcode::NpcSetHuntMode,
            2529 => ScriptOpcode::NpcSetMode,
            2530 => ScriptOpcode::NpcWalkTrigger,
            2531 => ScriptOpcode::NpcSetTimer,
            2532 => ScriptOpcode::NpcStat,
            2533 => ScriptOpcode::NpcStatAdd,
            2534 => ScriptOpcode::NpcStatHeal,
            2535 => ScriptOpcode::NpcStatSub,
            2536 => ScriptOpcode::NpcTele,
            2537 => ScriptOpcode::NpcType,
            2538 => ScriptOpcode::NpcUid,
            2539 => ScriptOpcode::SpotAnimNpc,
            2540 => ScriptOpcode::NpcWalk,
            2541 => ScriptOpcode::NpcAttackRange,
            2542 => ScriptOpcode::NpcHasOp,
            2543 => ScriptOpcode::NpcArriveDelay,
            // Loc ops (3000-3499)
            3000 => ScriptOpcode::LocAdd,
            3001 => ScriptOpcode::LocAngle,
            3002 => ScriptOpcode::LocAnim,
            3003 => ScriptOpcode::LocCategory,
            3004 => ScriptOpcode::LocChange,
            3005 => ScriptOpcode::LocCoord,
            3006 => ScriptOpcode::LocDel,
            3007 => ScriptOpcode::LocFind,
            3008 => ScriptOpcode::LocFindAllZone,
            3009 => ScriptOpcode::LocFindNext,
            3010 => ScriptOpcode::LocName,
            3011 => ScriptOpcode::LocParam,
            3012 => ScriptOpcode::LocShape,
            3013 => ScriptOpcode::LocType,
            // Obj ops (3500-4000)
            3500 => ScriptOpcode::ObjAdd,
            3501 => ScriptOpcode::ObjAddAll,
            3502 => ScriptOpcode::ObjCoord,
            3503 => ScriptOpcode::ObjCount,
            3504 => ScriptOpcode::ObjDel,
            3505 => ScriptOpcode::ObjName,
            3506 => ScriptOpcode::ObjParam,
            3507 => ScriptOpcode::ObjTakeItem,
            3508 => ScriptOpcode::ObjType,
            3509 => ScriptOpcode::ObjFind,
            // Npc config ops (4000-4099)
            4000 => ScriptOpcode::NcCategory,
            4001 => ScriptOpcode::NcDebugname,
            4002 => ScriptOpcode::NcDesc,
            4003 => ScriptOpcode::NcName,
            4004 => ScriptOpcode::NcOp,
            4005 => ScriptOpcode::NcParam,
            // Loc config ops (4100-4199)
            4100 => ScriptOpcode::LcCategory,
            4101 => ScriptOpcode::LcDebugname,
            4102 => ScriptOpcode::LcDesc,
            4103 => ScriptOpcode::LcName,
            4104 => ScriptOpcode::LcOp,
            4105 => ScriptOpcode::LcParam,
            4106 => ScriptOpcode::LcWidth,
            4107 => ScriptOpcode::LcLength,
            // Obj config ops (4200-4299)
            4200 => ScriptOpcode::OcCategory,
            4201 => ScriptOpcode::OcCert,
            4202 => ScriptOpcode::OcCost,
            4203 => ScriptOpcode::OcDebugname,
            4204 => ScriptOpcode::OcDesc,
            4205 => ScriptOpcode::OcIop,
            4206 => ScriptOpcode::OcMembers,
            4207 => ScriptOpcode::OcName,
            4208 => ScriptOpcode::OcOp,
            4209 => ScriptOpcode::OcParam,
            4210 => ScriptOpcode::OcStackable,
            4211 => ScriptOpcode::OcTradeable,
            4212 => ScriptOpcode::OcUncert,
            4213 => ScriptOpcode::OcWearPos2,
            4214 => ScriptOpcode::OcWearPos3,
            4215 => ScriptOpcode::OcWearPos,
            4216 => ScriptOpcode::OcWeight,
            // Inventory ops (4300-4399)
            4300 => ScriptOpcode::InvAllStock,
            4301 => ScriptOpcode::InvSize,
            4302 => ScriptOpcode::InvStockBase,
            4303 => ScriptOpcode::InvAdd,
            4304 => ScriptOpcode::InvChangeSlot,
            4305 => ScriptOpcode::InvClear,
            4306 => ScriptOpcode::InvDel,
            4307 => ScriptOpcode::InvDelSlot,
            4308 => ScriptOpcode::InvDropItem,
            4309 => ScriptOpcode::InvDropSlot,
            4310 => ScriptOpcode::InvFreespace,
            4311 => ScriptOpcode::InvGetNum,
            4312 => ScriptOpcode::InvGetObj,
            4313 => ScriptOpcode::InvItemSpace,
            4314 => ScriptOpcode::InvItemSpace2,
            4315 => ScriptOpcode::InvMoveFromSlot,
            4316 => ScriptOpcode::InvMoveToSlot,
            4317 => ScriptOpcode::BothMoveInv,
            4318 => ScriptOpcode::InvMoveItem,
            4319 => ScriptOpcode::InvMoveItemCert,
            4320 => ScriptOpcode::InvMoveItemUncert,
            4321 => ScriptOpcode::InvSetSlot,
            4322 => ScriptOpcode::InvTotal,
            4323 => ScriptOpcode::InvTotalCat,
            4324 => ScriptOpcode::InvTransmit,
            4325 => ScriptOpcode::InvOtherTransmit,
            4326 => ScriptOpcode::InvStopTransmit,
            4327 => ScriptOpcode::BothDropSlot,
            4328 => ScriptOpcode::InvDropAll,
            4329 => ScriptOpcode::InvTotalParam,
            4330 => ScriptOpcode::InvTotalParamStack,
            // Enum ops (4400-4499)
            4400 => ScriptOpcode::Enum,
            4401 => ScriptOpcode::EnumGetOutputCount,
            // String ops (4500-4599)
            4500 => ScriptOpcode::AppendNum,
            4501 => ScriptOpcode::Append,
            4502 => ScriptOpcode::AppendSignNum,
            4503 => ScriptOpcode::Lowercase,
            4504 => ScriptOpcode::TextGender,
            4505 => ScriptOpcode::ToString,
            4506 => ScriptOpcode::Compare,
            4507 => ScriptOpcode::TextSwitch,
            4508 => ScriptOpcode::AppendChar,
            4509 => ScriptOpcode::StringLength,
            4510 => ScriptOpcode::SubString,
            4511 => ScriptOpcode::StringIndexOfChar,
            4512 => ScriptOpcode::StringIndexOfString,
            // Number ops (4600-4699)
            4600 => ScriptOpcode::Add,
            4601 => ScriptOpcode::Sub,
            4602 => ScriptOpcode::Multiply,
            4603 => ScriptOpcode::Divide,
            4604 => ScriptOpcode::Random,
            4605 => ScriptOpcode::RandomInc,
            4606 => ScriptOpcode::Interpolate,
            4607 => ScriptOpcode::AddPercent,
            4608 => ScriptOpcode::SetBit,
            4609 => ScriptOpcode::ClearBit,
            4610 => ScriptOpcode::TestBit,
            4611 => ScriptOpcode::Modulo,
            4612 => ScriptOpcode::Pow,
            4613 => ScriptOpcode::InvPow,
            4614 => ScriptOpcode::And,
            4615 => ScriptOpcode::Or,
            4616 => ScriptOpcode::Min,
            4617 => ScriptOpcode::Max,
            4618 => ScriptOpcode::Scale,
            4619 => ScriptOpcode::BitCount,
            4620 => ScriptOpcode::ToggleBit,
            4621 => ScriptOpcode::SetBitRange,
            4622 => ScriptOpcode::ClearBitRange,
            4623 => ScriptOpcode::GetBitRange,
            4624 => ScriptOpcode::SetBitRangeToInt,
            4625 => ScriptOpcode::SinDeg,
            4626 => ScriptOpcode::CosDeg,
            4627 => ScriptOpcode::Atan2Deg,
            4628 => ScriptOpcode::Abs,
            // DB ops (7500-7599)
            7500 => ScriptOpcode::DbFindWithCount,
            7501 => ScriptOpcode::DbFindNext,
            7502 => ScriptOpcode::DbGetField,
            7503 => ScriptOpcode::DbGetFieldCount,
            7504 => ScriptOpcode::DbListAllWithCount,
            7505 => ScriptOpcode::DbGetRowTable,
            7506 => ScriptOpcode::DbFindByIndex,
            7507 => ScriptOpcode::DbFindRefineWithCount,
            7508 => ScriptOpcode::DbFind,
            7509 => ScriptOpcode::DbFindRefine,
            7510 => ScriptOpcode::DbListAll,
            // Debug ops (10000-11000)
            10000 => ScriptOpcode::Error,
            10001 => ScriptOpcode::MapProduction,
            10002 => ScriptOpcode::MapLastClock,
            10003 => ScriptOpcode::MapLastWorld,
            10004 => ScriptOpcode::MapLastClientIn,
            10005 => ScriptOpcode::MapLastNpc,
            10006 => ScriptOpcode::MapLastPlayer,
            10007 => ScriptOpcode::MapLastLogout,
            10008 => ScriptOpcode::MapLastLogin,
            10009 => ScriptOpcode::MapLastZone,
            10010 => ScriptOpcode::MapLastClientOut,
            10011 => ScriptOpcode::MapLastCleanup,
            10012 => ScriptOpcode::MapLastBandwidthIn,
            10013 => ScriptOpcode::MapLastBandwidthOut,
            _ => panic!("Invalid script opcode value: {}", code),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ScriptInfo {
    name: String,
    path: String,
    lookup: i32,
    params: Vec<u8>,
    pcs: Vec<i32>,
    lines: Vec<i32>,
}

#[wasm_bindgen]
impl ScriptInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        path: String,
        lookup: i32,
        params: Vec<u8>,
        pcs: Vec<i32>,
        lines: Vec<i32>,
    ) -> ScriptInfo {
        return ScriptInfo {
            name,
            path,
            lookup,
            params,
            pcs,
            lines,
        };
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ScriptFile {
    id: usize,
    int_local_count: u16,
    string_local_count: u16,
    int_arg_count: u16,
    string_arg_count: u16,
    switch_table: Map,
    info: ScriptInfo,
    codes: Vec<u16>,
    int_operands: Vec<usize>,
    string_operands: Vec<String>,
}

#[wasm_bindgen]
impl ScriptFile {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: usize,
        int_local_count: u16,
        string_local_count: u16,
        int_arg_count: u16,
        string_arg_count: u16,
        switch_table: Map,
        info: ScriptInfo,
        codes: Vec<u16>,
        int_operands: Vec<usize>,
        string_operands: Array,
    ) -> ScriptFile {
        return ScriptFile {
            id,
            int_local_count,
            string_local_count,
            int_arg_count,
            string_arg_count,
            switch_table,
            info,
            codes,
            int_operands,
            string_operands: string_operands
                .iter()
                .filter_map(|value| value.as_string())
                .collect::<Vec<String>>(),
        };
    }

    #[wasm_bindgen]
    pub fn _clone(&self) -> ScriptFile {
        self.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn id(&self) -> usize {
        // TODO: remove this export after refactoring scriptstate.
        return self.id;
    }

    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String {
        // TODO: remove this export after refactoring scriptstate.
        return self.info.name.clone();
    }

    #[wasm_bindgen(method, getter)]
    pub fn params(&self) -> Vec<u8> {
        // TODO: remove this export after refactoring scriptstate.
        return self.info.params.clone();
    }

    #[wasm_bindgen(method, getter)]
    pub fn lookup(&self) -> i32 {
        return self.info.lookup;
    }
}

struct GoSubFrame {
    script: ScriptFile,
    pc: isize, // program counter
    int_locals: Vec<i32>,
    string_locals: Vec<String>,
}

struct GoToFrame {
    script: ScriptFile,
    pc: isize,
}

#[repr(i8)]
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum ScriptExecutionState {
    Aborted,
    Running,
    Finished,
    Suspended, // suspended to move to player
    PauseButton,
    CountDialog,
    NpcSuspended,   // suspended to move to npc
    WorldSuspended, // suspended to move to world
}

#[repr(u8)]
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ScriptPointer {
    ActivePlayer,
    ActivePlayer2,
    ProtectedActivePlayer,
    ProtectedActivePlayer2,
    ActiveNpc,
    ActiveNpc2,
    ActiveLoc,
    ActiveLoc2,
    ActiveObj,
    ActiveObj2,
    Last,
}

impl From<i32> for ScriptPointer {
    #[inline(always)]
    fn from(value: i32) -> Self {
        match value {
            0 => ScriptPointer::ActivePlayer,
            1 => ScriptPointer::ActivePlayer2,
            2 => ScriptPointer::ProtectedActivePlayer,
            3 => ScriptPointer::ProtectedActivePlayer2,
            4 => ScriptPointer::ActiveNpc,
            5 => ScriptPointer::ActiveNpc2,
            6 => ScriptPointer::ActiveLoc,
            7 => ScriptPointer::ActiveLoc2,
            8 => ScriptPointer::ActiveObj,
            9 => ScriptPointer::ActiveObj2,
            10 => ScriptPointer::Last,
            _ => panic!("Pointer not found for from value of: {}", value),
        }
    }
}

#[wasm_bindgen]
pub struct ScriptState {
    script: ScriptFile,
    execution_state: ScriptExecutionState,
    pc: isize,    // program counter
    opcount: i32, // number of opcodes executed
    gosub_frame_stack: Vec<GoSubFrame>,
    gosub_fp: usize, // gosub frame pointer
    goto_frame_stack: Vec<GoToFrame>,
    goto_fp: usize, // goto frame pointer
    int_stack: Vec<i32>,
    isp: usize, // integer stack pointer
    string_stack: Vec<String>,
    ssp: usize, // string stack pointer
    int_locals: Vec<i32>,
    string_locals: Vec<String>,
    pointers: i32, // state pointers
    trigger: u8,
    _self: JsValue,
    active_player: Player,
    active_player2: Player,
    active_npc: Npc,
    active_npc2: Npc,
    active_loc: Loc,
    active_loc2: Loc,
    active_obj: Obj,
    active_obj2: Obj,
    last_int: i32,
    error: String,
}

impl ScriptState {
    pub const ACTIVE_NPC: [ScriptPointer; 2] =
        [ScriptPointer::ActiveNpc, ScriptPointer::ActiveNpc2];

    pub const ACTIVE_LOC: [ScriptPointer; 2] =
        [ScriptPointer::ActiveLoc, ScriptPointer::ActiveLoc2];

    pub const ACTIVE_OBJ: [ScriptPointer; 2] =
        [ScriptPointer::ActiveObj, ScriptPointer::ActiveObj2];

    pub const ACTIVE_PLAYER: [ScriptPointer; 2] =
        [ScriptPointer::ActivePlayer, ScriptPointer::ActivePlayer2];

    pub const PROTECTED_ACTIVE_PLAYER: [ScriptPointer; 2] = [
        ScriptPointer::ProtectedActivePlayer,
        ScriptPointer::ProtectedActivePlayer2,
    ];

    // ---- frames

    /// Pops the most recent subroutine frame from the frame stack and restores its state.
    ///
    /// This method is used to return from a subroutine by restoring the execution context
    /// from the most recent frame saved via `push_frame`. It retrieves the last `GoSubFrame`
    /// from the `frame_stack` and updates the current script execution context, including
    /// the program counter (`pc`) and local variables, to the state of the popped frame.
    ///
    /// After restoring the frame, the frame pointer (`fp`) is decremented, reflecting the
    /// return to the previous frame.
    ///
    /// This method assumes that there is at least one frame in the stack to pop, meaning
    /// it should only be called after a `push_frame` has occurred and control is expected
    /// to return to a prior frame (i.e., during subroutine completion).
    ///
    /// # Behavior
    ///
    /// - Pops the most recent `GoSubFrame` from the `frame_stack`.
    /// - Restores the script, program counter (`pc`), and local variables (`int_locals` and `string_locals`) from the popped frame.
    /// - Decrements the frame pointer (`fp`) to reflect returning to the previous frame.
    #[inline(always)]
    pub fn pop_frame(&mut self) -> Result<(), String> {
        let frame: GoSubFrame = self.gosub_frame_stack.pop().ok_or("Stack is empty")?;
        self.gosub_fp -= 1;
        self.pc = frame.pc;
        self.script = frame.script;
        self.int_locals = frame.int_locals;
        self.string_locals = frame.string_locals;
        return Ok(());
    }

    /// Pushes a new subroutine frame onto the frame stack for the given script.
    ///
    /// This method saves the current execution context, including the current script, program counter (PC),
    /// and local variables, into a new `GoSubFrame`. The frame is then pushed onto the `frame_stack`, allowing
    /// the current frame to be resumed later when the subroutine completes. This is akin to a `GOSUB` routine
    /// in traditional programming, where the control flow is expected to return.
    ///
    /// After saving the current frame, the program counter (`pc`) is set to -1 to prepare for execution
    /// in the new subroutine frame. Local variables are populated from the integer and string stacks based
    /// on the argument counts (`int_arg_count` and `string_arg_count`) specified in the provided `ScriptFile`.
    ///
    /// Unlike `goto_frame`, which discards the current frame and does not allow returning, `push_frame`
    /// preserves the current frame and enables returning to it later, making it suitable for subroutine
    /// execution where a return point is necessary.
    ///
    /// # Parameters
    ///
    /// - `script`: A reference to the `ScriptFile` to execute in the new subroutine frame.
    ///
    /// # Behavior
    ///
    /// - Saves the current script, program counter, and local variables in a `GoSubFrame`.
    /// - Pushes this `GoSubFrame` onto the `frame_stack`, allowing the current frame to be resumed later.
    /// - Increments the frame pointer (`fp`) to reflect the new frame.
    /// - Resets the program counter (`pc`) to -1 to prepare for execution in the new subroutine.
    /// - Initializes local integer and string variables by popping them from the respective stacks.
    #[inline(always)]
    pub fn gosub_frame(&mut self, script: ScriptFile) {
        self.gosub_frame_stack.push(GoSubFrame {
            script: self.script.clone(),
            pc: self.pc,
            int_locals: self.int_locals.clone(),
            string_locals: self.string_locals.clone(),
        });
        self.gosub_fp += 1;
        self.new_program(script);
    }

    /// Jumps to a new goto frame without saving the current frame's context.
    ///
    /// This method is used to jump to a new frame in the execution stack, discarding the current frame's context.
    /// It saves the current script and program counter (PC) into the `goto_frame_stack`, then clears the frame stack,
    /// sets the current frame pointer (`fp`) to 0, and resets the program counter to -1.
    ///
    /// The local integer and string variables for the new frame are populated by popping values from their respective
    /// stacks, based on the argument counts (`int_arg_count` and `string_arg_count`) specified in the provided `ScriptFile`.
    ///
    /// This function differs from a typical subroutine frame push (like in `push_frame`) because it clears the
    /// frame stack, meaning that control flow does not return to the previous state.
    ///
    /// # Parameters
    ///
    /// - `script`: A reference to the `ScriptFile` for the new frame to jump to.
    ///
    /// # Behavior
    ///
    /// - Pushes the current script and program counter onto the `goto_frame_stack`.
    /// - Increments the `goto_fp` (GoTo Frame Pointer) by 1.
    /// - Clears the `frame_stack`, discarding any previously saved frames.
    /// - Resets the frame pointer (`fp`) and program counter (`pc`).
    /// - Initializes local integer and string variables by popping them from the respective stacks.
    #[inline(always)]
    pub fn goto_frame(&mut self, script: ScriptFile) {
        self.goto_frame_stack.push(GoToFrame {
            script: self.script.clone(),
            pc: self.pc,
        });
        self.goto_fp += 1;
        self.gosub_frame_stack.truncate(0);
        self.gosub_fp = 0;
        self.new_program(script);
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub fn new_program(&mut self, script: ScriptFile) {
        let mut int_locals: Vec<i32> = vec![0; script.int_local_count as usize];
        let int_arg_count: usize = script.int_arg_count as usize;
        for index in 0..int_arg_count {
            int_locals[int_arg_count - index - 1] = self.pop_int();
        }

        let mut string_locals: Vec<String> = vec![String::new(); script.string_local_count as usize];
        let string_arg_count: usize = script.string_arg_count as usize;
        for index in 0..string_arg_count {
            string_locals[string_arg_count - index - 1] = self.pop_string();
        }

        self.pc = -1;
        self.script = script;
        self.int_locals = int_locals;
        self.string_locals = string_locals;
    }

    #[inline(always)]
    pub fn pop_args(&mut self) -> Vec<JsValue> {
        let types: String = self.pop_string();
        let mut args: Vec<JsValue> = Vec::with_capacity(types.len());
        for char in types.chars().rev() {
            args.push(match char {
                's' => JsValue::from(self.pop_string()),
                _ => JsValue::from(self.pop_int()),
            })
        }
        return args;
    }

    #[inline(always)]
    pub fn branch(&mut self, branch: isize) {
        self.pc += branch;
    }

    /// Protects the execution flow based on pointer validation and executes a callback on success.
    ///
    /// This method checks if the script state contains the required pointer based on the current operand.
    /// If the pointer is valid, the provided `on_success` callback is executed with the script state.
    /// Otherwise, an error is returned detailing the missing pointer.
    ///
    /// # Parameters
    ///
    /// - `pointers`: A slice of `ScriptPointer` values representing valid pointers for the current state.
    /// - `on_success`: A closure that is executed if the required pointer is valid. It receives a mutable reference
    ///   to the current `ScriptState` and returns a `Result<(), String>`.
    ///
    /// # Return
    ///
    /// Returns `Ok(())` if the pointer check succeeds and the `on_success` closure executes successfully.
    /// If the pointer is invalid or the closure returns an error, a `Result::Err` with a descriptive error message is returned.
    ///
    /// # Panics
    ///
    /// This function does not panic, but it assumes that `pointers` has valid indices, which depends on the
    /// operand value returned by `self.int_operand()`. Out-of-bounds indexing on `pointers` will result in a panic.
    ///
    /// # Side Effects
    ///
    /// - The state may be modified by the `on_success` closure. The closure has full access to the mutable state.
    /// - The method does not alter any state if the pointer check fails or if an error is returned.
    pub fn protect<F>(&mut self, pointers: &[ScriptPointer], on_success: F)
    where
        F: FnOnce(&mut ScriptState),
    {
        let pointer: ScriptPointer = pointers[self.int_operand()];
        if self.pointer_check(pointer) {
            return on_success(self);
        }
        self.abort(format!(
            "Required pointer: {}, current: {}",
            self.pointer_print(1 << pointer as i32),
            self.pointer_print(self.pointers)
        ));
    }

    /// Retrieves the active player from the script state based on the current operand.
    ///
    /// This method returns either `active_player` or `active_player2` based on the result of `self.int_operand()`.
    /// If the operand is `0`, `active_player` is returned; otherwise, `active_player2` is returned.
    ///
    /// # Return
    ///
    /// Returns an `i32` representing the ID of the active player, which can be either `active_player` or `active_player2`
    /// depending on the operand value.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    ///
    /// # Side Effects
    ///
    /// - This function does not modify the internal state; it only reads the current state.
    #[inline(always)]
    pub fn get_active_player(&self) -> Result<&Player, String> {
        let player: &Player = match self.int_operand() {
            0 => &self.active_player,
            _ => &self.active_player2,
        };
        if player.is_null() {
            return Err(String::from("Attempt to access null active_player"));
        }
        return Ok(player);
    }

    /// Sets the active player in the script state based on the current operand.
    ///
    /// This method updates either `active_player` or `active_player2` based on the result of `self.int_operand()`.
    /// If the operand is `0`, `active_player` is set; otherwise, `active_player2` is set.
    ///
    /// # Parameters
    ///
    /// - `player`: An `i32` representing the player ID to be set as the active player.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    ///
    /// # Side Effects
    ///
    /// - Modifies the internal state of the `ScriptState` by setting either `active_player` or `active_player2`
    ///   depending on the operand value.
    #[inline(always)]
    pub fn set_active_player(&mut self, player: Player) {
        match self.int_operand() {
            0 => self.active_player = player,
            _ => self.active_player2 = player,
        }
    }

    pub fn get_active_player1(&self) -> &Player {
        return &self.active_player;
    }

    pub fn get_active_player2(&self) -> &Player {
        return &self.active_player2;
    }

    #[inline(always)]
    pub fn get_active_npc(&self) -> Result<&Npc, String> {
        let npc: &Npc = match self.int_operand() {
            0 => &self.active_npc,
            _ => &self.active_npc2,
        };
        if npc.is_null() {
            return Err(String::from("Attempt to access null active_npc"));
        }
        return Ok(npc);
    }

    #[inline(always)]
    pub fn set_active_npc(&mut self, npc: Npc) {
        match self.int_operand() {
            0 => self.active_npc = npc,
            _ => self.active_npc2 = npc,
        }
    }

    #[inline(always)]
    pub fn get_active_obj(&self) -> Result<&Obj, String> {
        let npc: &Obj = match self.int_operand() {
            0 => &self.active_obj,
            _ => &self.active_obj2,
        };
        if npc.is_null() {
            return Err(String::from("Attempt to access null active_obj"));
        }
        return Ok(npc);
    }

    #[inline(always)]
    pub fn set_active_obj(&mut self, obj: Obj) {
        match self.int_operand() {
            0 => self.active_obj = obj,
            _ => self.active_obj2 = obj,
        }
    }

    #[inline(always)]
    pub fn get_script(&self) -> &ScriptFile {
        return &self.script;
    }

    #[inline(always)]
    pub fn get_switch_table(
        &self,
        key: i32,
        switch: usize,
    ) -> Result<Option<i32>, serde_wasm_bindgen::Error> {
        let switch_table: JsValue = self.script.switch_table.get(&JsValue::from(key));
        let table: HashMap<usize, i32> = serde_wasm_bindgen::from_value(switch_table)?;
        return Ok(table.get(&switch).copied());
    }

    #[inline(always)]
    pub fn get_fp(&self) -> usize {
        return self.gosub_fp;
    }

    #[inline(always)]
    pub fn get_isp(&self) -> usize {
        return self.isp;
    }

    #[inline(always)]
    pub fn set_isp(&mut self, isp: usize) {
        self.isp = isp;
    }

    #[inline(always)]
    pub fn get_ssp(&self) -> usize {
        return self.ssp;
    }

    #[inline(always)]
    pub fn set_ssp(&mut self, ssp: usize) {
        self.ssp = ssp;
    }

    #[inline(always)]
    pub fn get_int_locals(&mut self) -> &mut Vec<i32> {
        return &mut self.int_locals;
    }

    #[inline(always)]
    pub fn get_string_locals(&mut self) -> &mut Vec<String> {
        return &mut self.string_locals;
    }

    #[inline(always)]
    pub fn opcodes(&self) -> &Vec<u16> {
        return &self.script.codes;
    }

    pub fn script_line_number(&self, pc: isize) -> i32 {
        for i in 0..self.script.info.pcs.len() {
            if self.script.info.pcs[i] > pc as i32 {
                return self.script.info.lines[i.saturating_sub(1)];
            }
        }
        return *self.script.info.lines.last().unwrap_or(&0);
    }

    #[inline(always)]
    pub fn set_execution_state(&mut self, state: ScriptExecutionState) {
        self.execution_state = state;
    }

    #[inline(always)]
    pub fn get_pc(&self) -> isize {
        return self.pc;
    }

    #[inline(always)]
    pub fn set_pc(&mut self, pc: isize) {
        self.pc = pc;
    }

    #[inline(always)]
    pub fn set_opcount(&mut self, opcount: i32) {
        self.opcount = opcount;
    }

    pub fn abort(&mut self, message: String) {
        let file_name: String = self.get_script_file_name();
        let script_name: &String = &self.script.info.name;
        let line_number: i32 = self.script_line_number(self.pc);

        self.execution_state = ScriptExecutionState::Aborted;
        self.error = format!(
            r#"
            Script Error: {message}
            File: {file_name}

            1. {name} - {file_name}:{line}"#,
            message = message,
            file_name = file_name,
            name = script_name,
            line = line_number
        );
    }
}

#[wasm_bindgen]
impl ScriptState {
    /// Creates a new `ScriptState` instance with specified `i32` and `String` arguments.
    ///
    /// This method initializes a `ScriptState` for a given `ScriptFile` and sets up local
    /// variables based on the provided `i32` and `String` arguments. It allocates space for
    /// local variables and populates them with the arguments supplied.
    ///
    /// # Parameters
    ///
    /// - `script`: A reference to the `ScriptFile` to execute.
    /// - `int_args`: A vector of `i32` representing integer arguments for the script.
    /// - `string_args`: A vector of `String` representing string arguments for the script.
    ///
    /// # Return
    ///
    /// Returns a new instance of `ScriptState` initialized with the provided arguments.
    ///
    /// # Safety
    ///
    /// This function assumes that the `ScriptFile` provided has valid local counts for
    /// integer and string variables. If the `script` is improperly initialized, it may cause
    /// undefined behavior.
    #[rustfmt::skip]
    #[wasm_bindgen(constructor)]
    pub fn new(
        script: &ScriptFile,
        int_args: Vec<i32>,
        string_args: Vec<String>,
    ) -> ScriptState {
        let mut int_locals: Vec<i32> = vec![0; script.int_local_count as usize];
        let mut string_locals: Vec<String> = vec![String::new(); script.string_local_count as usize];

        int_locals[..int_args.len()].copy_from_slice(&int_args);
        string_locals[..string_args.len()].clone_from_slice(&string_args);

        let trigger: u8 = script.lookup() as u8 & 0xff;

        return ScriptState {
            script: script.clone(),
            execution_state: ScriptExecutionState::Running,
            pc: -1,
            opcount: 0,
            gosub_frame_stack: Vec::with_capacity(50),
            gosub_fp: 0,
            goto_frame_stack: Vec::with_capacity(50),
            goto_fp: 0,
            int_stack: vec![0; 1000],
            isp: 0,
            string_stack: vec![String::new(); 1000],
            ssp: 0,
            int_locals,
            string_locals,
            pointers: 0,
            trigger,
            _self: JsValue::NULL,
            active_player: Player::from(JsValue::NULL),
            active_player2: Player::from(JsValue::NULL),
            active_npc: Npc::from(JsValue::NULL),
            active_npc2: Npc::from(JsValue::NULL),
            active_loc: Loc::from(JsValue::NULL),
            active_loc2: Loc::from(JsValue::NULL),
            active_obj: Obj::from(JsValue::NULL),
            active_obj2: Obj::from(JsValue::NULL),
            last_int: 0,
            error: String::new(),
        }
    }

    #[inline(always)]
    #[wasm_bindgen(method, js_name = intOperand)]
    pub fn int_operand(&self) -> usize {
        return self.script.int_operands[self.pc as usize];
    }

    /// Pushes an `i32` value onto the integer stack.
    ///
    /// This method places the provided integer value at the current integer stack pointer
    /// and then increments the pointer. It assumes that there is sufficient space on the stack.
    /// When a `ScriptState` is created, it allocates the string stack with a capacity of 1000.
    ///
    /// # Parameters
    ///
    /// - `value`: The `i32` value to push onto the integer stack.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = pushInt)]
    pub fn push_int(&mut self, value: i32) {
        unsafe { *self.int_stack.as_mut_ptr().add(self.isp) = value };
        self.isp += 1;
    }

    /// Pops the top integer value from the integer stack.
    ///
    /// This method decreases the integer stack pointer and retrieves the integer at the new top
    /// of the stack. The caller should ensure there are values on the stack.
    ///
    /// # Return
    ///
    /// Returns the top integer value as an `i32`.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = popInt)]
    pub fn pop_int(&mut self) -> i32 {
        self.isp -= 1;
        return unsafe { *self.int_stack.as_ptr().add(self.isp) };
    }

    // ---- strings

    #[rustfmt::skip]
    #[inline(always)]
    #[wasm_bindgen(method, js_name = stringOperand)]
    pub fn string_operand(&self) -> String {
        return unsafe { (*self.script.string_operands.as_ptr().add(self.pc as usize)).clone() };
    }

    /// Pushes a `String` value onto the string stack.
    ///
    /// This method places the provided string value at the current string stack pointer
    /// and then increments the pointer. It assumes that there is sufficient space on the stack.
    /// When a `ScriptState` is created, it allocates the string stack with a capacity of 1000.
    ///
    /// # Parameters
    ///
    /// - `value`: The `String` value to push onto the string stack.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = pushString)]
    pub fn push_string(&mut self, value: String) {
        unsafe { *self.string_stack.as_mut_ptr().add(self.ssp) = value };
        self.ssp += 1;
    }

    /// Pops the top string value from the string stack.
    ///
    /// This method decreases the string stack pointer and retrieves the string at the new top
    /// of the stack. IThe caller should ensure there are values on the stack.
    ///
    /// # Return
    ///
    /// Returns the top string value as a `String`.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = popString)]
    pub fn pop_string(&mut self) -> String {
        self.ssp -= 1;
        return unsafe { (*self.string_stack.as_ptr().add(self.ssp)).clone() };
    }

    // ---- pointers

    /// Adds a specified pointer to the internal pointer state.
    ///
    /// This method modifies the internal state by setting the bit corresponding
    /// to the provided `pointer`. It effectively adds the pointer to the current
    /// set of pointers tracked by the instance.
    ///
    /// # Parameters
    ///
    /// - `pointer`: A `ScriptPointer` representing the pointer to be added.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the internal state in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided pointer is a valid variant of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - Updates the internal `pointers` state to include the new pointer.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = pointerAdd)]
    pub fn pointer_add(&mut self, pointer: ScriptPointer) {
        self.pointers |= 1 << pointer as i32;
    }

    /// Removes a specified pointer from the internal pointer state.
    ///
    /// This method modifies the internal state by clearing the bit corresponding
    /// to the provided `pointer`. It effectively removes the pointer from the current
    /// set of pointers tracked by the instance.
    ///
    /// # Parameters
    ///
    /// - `pointer`: A `ScriptPointer` representing the pointer to be removed.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies the internal state in place.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided pointer is a valid variant of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - Updates the internal `pointers` state to exclude the specified pointer.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = pointerRemove)]
    pub fn pointer_remove(&mut self, pointer: ScriptPointer) {
        self.pointers &= !(1 << pointer as i32);
    }

    /// Checks whether a specified pointer is currently set in the internal state.
    ///
    /// This method examines the internal pointer state and returns a boolean indicating
    /// whether the specified `pointer` is currently active (set).
    ///
    /// # Parameters
    ///
    /// - `pointer`: A `ScriptPointer` representing the pointer to check.
    ///
    /// # Return
    ///
    /// Returns `true` if the pointer is currently set, and `false` otherwise.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided pointer is a valid variant of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - This function does not modify the internal state.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = pointerGet)]
    pub fn pointer_get(&self, pointer: ScriptPointer) -> bool {
        return (self.pointers & (1 << pointer as i32)) != 0;
    }

    /// Validates the presence of required pointers in the internal state.
    ///
    /// This method checks if all specified pointers are currently set in the internal
    /// state. If any required pointer is missing, the function will panic with a detailed
    /// error message that includes both the required and current pointer states.
    ///
    /// # Parameters
    ///
    /// - `pointers`: A slice of `ScriptPointer` representing the pointers to check for presence.
    ///
    /// # Return
    ///
    /// This function does not return any value. It modifies control flow by panicking if
    /// any required pointer is missing.
    ///
    /// # Panics
    ///
    /// Panics if any of the specified pointers are not set in the internal state.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as the
    /// provided pointers are valid variants of `ScriptPointer`.
    ///
    /// # Side Effects
    ///
    /// - This function may panic and terminate the execution if required pointers are not found.
    #[inline(always)]
    #[wasm_bindgen(method, js_name = pointerCheck)]
    pub fn pointer_check(&self, pointer: ScriptPointer) -> bool {
        let flag: i32 = 1 << pointer as i32;
        return self.pointers & flag == flag;
    }

    /// Converts the specified flags into a readable string representation of active pointers.
    ///
    /// This method takes a set of pointer flags and generates a string listing the
    /// corresponding `ScriptPointer` variants that are active in the given flags.
    ///
    /// # Parameters
    ///
    /// - `flags`: An `i32` representing the bit flags of pointers to convert to a string.
    ///
    /// # Return
    ///
    /// Returns a `String` that lists the active `ScriptPointer` variants corresponding
    /// to the provided flags. The string is formatted as a comma-separated list.
    ///
    /// # Panics
    ///
    /// This function does not panic under normal operation.
    ///
    /// # Safety
    ///
    /// There are no specific safety concerns for this function. It is safe as long as
    /// the provided flags represent valid states for the `ScriptPointer` variants.
    ///
    /// # Side Effects
    ///
    /// - This function does not modify any state. It only reads the internal state and generates
    ///   a string representation of the provided flags.
    fn pointer_print(&self, flags: i32) -> String {
        let mut text = String::new();
        for i in 0..(ScriptPointer::Last as usize) {
            let flag: i32 = 1 << i;
            if flags & flag != 0 {
                text.push_str(&format!("{:?}, ", ScriptPointer::from(i as i32)));
            }
        }
        if !text.is_empty() {
            text.truncate(text.len() - 2); // Remove last ", "
        }
        return text;
    }

    #[inline(always)]
    #[wasm_bindgen(method, getter = execution)]
    pub fn get_execution_state(&self) -> ScriptExecutionState {
        return self.execution_state;
    }

    #[inline(always)]
    #[wasm_bindgen(method, getter = error)]
    pub fn get_error(&self) -> String {
        return self.error.clone();
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = error)]
    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = self)]
    pub fn set_self(&mut self, _self: JsValue) {
        self._self = _self;
    }

    #[inline(always)]
    #[wasm_bindgen(method, getter = self)]
    pub fn get_self(&mut self) -> JsValue {
        return self._self.clone();
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activePlayer)]
    pub fn set_active_player1(&mut self, player: Player) {
        self.active_player = player;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activePlayer2)]
    pub fn set_active_player2(&mut self, player: Player) {
        self.active_player2 = player;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = activePlayerScript)]
    pub fn set_active_player_script(&mut self, state: ScriptState) {
        if let Ok(player) = self.get_active_player() {
            player.active_script(state);
        }
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = activePlayerProtect)]
    pub fn set_active_player_protect(&mut self, protect: bool) {
        if let Ok(player) = self.get_active_player() {
            player.protect(protect);
        }
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activeNpc)]
    pub fn set_active_npc1(&mut self, npc: Npc) {
        self.active_npc = npc;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activeNpc2)]
    pub fn set_active_npc2(&mut self, npc: Npc) {
        self.active_npc2 = npc;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = activeNpcScript)]
    pub fn set_active_npc_script(&mut self, state: ScriptState) {
        if let Ok(npc) = self.get_active_npc() {
            npc.active_script(state);
        }
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activeLoc)]
    pub fn set_active_loc1(&mut self, loc: Loc) {
        self.active_loc = loc;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activeLoc2)]
    pub fn set_active_loc2(&mut self, loc: Loc) {
        self.active_loc2 = loc;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activeObj)]
    pub fn set_active_obj1(&mut self, obj: Obj) {
        self.active_obj = obj;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = _activeObj2)]
    pub fn set_active_obj2(&mut self, obj: Obj) {
        self.active_obj2 = obj;
    }

    #[inline(always)]
    #[wasm_bindgen(method, getter = opcount)]
    pub fn get_opcount(&self) -> i32 {
        return self.opcount;
    }

    #[inline(always)]
    #[wasm_bindgen(method, setter = lastInt)]
    pub fn set_last_int(&mut self, last_int: i32) {
        self.last_int = last_int;
    }

    #[inline(always)]
    #[wasm_bindgen(method, getter = scriptName)]
    pub fn get_script_name(&self) -> String {
        return self.script.name();
    }

    #[inline(always)]
    #[wasm_bindgen(method, getter = scriptFileName)]
    pub fn get_script_file_name(&self) -> String {
        return self
            .script
            .info
            .path
            .clone()
            .split('/')
            .last()
            .unwrap()
            .split('\\')
            .last()
            .map(String::from)
            .unwrap();
    }
}
