#![allow(non_snake_case)]

use std::rc::Rc;
use js_sys::{Array, JsString, Map, SharedArrayBuffer, Uint16Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::player_ops::Player;

#[repr(u16)]
#[wasm_bindgen]
#[derive(Debug)]
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
            lines
        }
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
    int_operands: Vec<i32>,
    string_operands: Array,
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
        int_operands: Vec<i32>,
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
            string_operands,
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn id(&self) -> usize { // TODO: remove this export after refactoring scriptstate.
        return self.id;
    }

    #[wasm_bindgen(method, getter)]
    pub fn name(&self) -> String { // TODO: remove this export after refactoring scriptstate.
        return self.info.name.clone();
    }

    #[wasm_bindgen(method, getter = fileName)]
    pub fn file_name(&self) -> String { // TODO: remove this export after refactoring scriptstate.
        return self.info.path.clone().split('/')
            .last()
            .unwrap()
            .split('\\')
            .last()
            .map(String::from)
            .unwrap();
    }

    #[wasm_bindgen(method, js_name = lineNumber)]
    pub fn line_number(&self, pc: i32) -> i32 { // TODO: remove this export after refactoring scriptstate.
        for i in 0..self.info.pcs.len() {
            if self.info.pcs[i] > pc {
                return self.info.lines[i.saturating_sub(1)];
            }
        }
        *self.info.lines.last().unwrap_or(&0)
    }

    #[wasm_bindgen(method, getter)]
    pub fn params(&self) -> Vec<u8> { // TODO: remove this export after refactoring scriptstate.
        return self.info.params.clone();
    }

    #[wasm_bindgen(method, getter)]
    pub fn lookup(&self) -> i32 { // TODO: remove this export after refactoring scriptstate.
        return self.info.lookup;
    }

    #[wasm_bindgen(method, getter = intArgCount)]
    pub fn int_arg_count(&self) -> u16 { // TODO: remove this export after refactoring scriptstate.
        return self.int_arg_count;
    }

    #[wasm_bindgen(method, getter = stringArgCount)]
    pub fn string_arg_count(&self) -> u16 { // TODO: remove this export after refactoring scriptstate.
        return self.string_arg_count;
    }

    #[wasm_bindgen(method, getter = intOperands)]
    pub fn int_operands(&self) -> Vec<i32> { // TODO: remove this export after refactoring scriptstate.
        return self.int_operands.clone();
    }

    #[wasm_bindgen(method, getter = stringOperands)]
    pub fn string_operands(&self) -> Array { // TODO: remove this export after refactoring scriptstate.
        return self.string_operands.clone();
    }

    #[wasm_bindgen(method, js_name = switchTables)] // returns Map<number, number> | undefined
    pub fn switch_tables(&self, key: i32) -> JsValue { // TODO: remove this export after refactoring scriptstate.
        return self.switch_table.get(&JsValue::from(key));
    }

    #[wasm_bindgen(method, getter)]
    pub fn opcodes(&self) -> Vec<u16> { // TODO: remove this export after refactoring scriptstate.
        return self.codes.clone();
    }
}

struct GoSubFrame {
    script: ScriptFile,
    pc: i32, // program counter
    int_locals: Vec<i32>,
    string_locals: Vec<String>,
}

struct GoToFrame {
    script: ScriptFile,
    pc: i32,
}

#[repr(i8)]
#[wasm_bindgen]
#[derive(Clone, Copy)]
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
    pc: i32,      // program counter
    opcount: i32, // number of opcodes executed
    frame_stack: Vec<GoSubFrame>,
    fp: usize, // frame pointer
    goto_frame_stack: Vec<GoToFrame>,
    goto_fp: usize,
    int_stack: Vec<i32>,
    isp: usize, // integer stack pointer
    string_stack: Vec<String>,
    ssp: usize, // string stack pointer
    int_locals: Vec<i32>,
    string_locals: Vec<String>,
    pointers: i32, // state pointers
    active_player: Rc<Player>,
    active_player2: Rc<Player>,
}

#[wasm_bindgen]
impl ScriptState {
    // const ACTIVE_NPC: [ScriptPointer; 2] =
    //     [ScriptPointer::ActiveNpc, ScriptPointer::ActiveNpc2];
    //
    // const ACTIVE_LOC: [ScriptPointer; 2] =
    //     [ScriptPointer::ActiveLoc, ScriptPointer::ActiveLoc2];
    //
    // const ACTIVE_OBJ: [ScriptPointer; 2] =
    //     [ScriptPointer::ActiveObj, ScriptPointer::ActiveObj2];
    //
    // const ACTIVE_PLAYER: [ScriptPointer; 2] =
    //     [ScriptPointer::ActivePlayer, ScriptPointer::ActivePlayer2];
    //
    // const PROTECTED_ACTIVE_PLAYER: [ScriptPointer; 2] = [
    //     ScriptPointer::ProtectedActivePlayer,
    //     ScriptPointer::ProtectedActivePlayer2,
    // ];

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
        script: ScriptFile,
        int_args: Vec<i32>,
        string_args: Vec<String>,
    ) -> ScriptState {
        let mut int_locals: Vec<i32> = vec![0; script.int_local_count as usize];
        let mut string_locals: Vec<String> = vec![String::new(); script.string_local_count as usize];

        int_locals[..int_args.len()].copy_from_slice(&int_args);
        string_locals[..string_args.len()].clone_from_slice(&string_args);

        return ScriptState {
            script,
            execution_state: ScriptExecutionState::Running,
            pc: -1,
            opcount: 0,
            frame_stack: Vec::with_capacity(50),
            fp: 0,
            goto_frame_stack: Vec::with_capacity(50),
            goto_fp: 0,
            int_stack: vec![0; 1000],
            isp: 0,
            string_stack: vec![String::new(); 1000],
            ssp: 0,
            int_locals,
            string_locals,
            pointers: 0,
            active_player: Rc::new(Player::null()),
            active_player2: Rc::new(Player::null()),
        }
    }

    #[inline(always)]
    pub fn int_operand(&self) -> i32 {
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
    pub fn push_int(&mut self, value: i32) {
        self.int_stack[self.isp] = value;
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
    pub fn pop_int(&mut self) -> i32 {
        self.isp -= 1;
        return self.int_stack[self.isp];
    }

    // ---- strings

    #[inline(always)]
    pub fn string_operand(&self) -> String {
        // return self.script.string_operands
        //     .iter()
        //     .filter_map(|value| value.as_string()) // Convert JsValue to Option<String>
        //     .collect::<Vec<String>>()
        //     [self.pc as usize]
        //     .clone();
        return self.script.string_operands.get(self.pc as u32).as_string().unwrap();
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
    pub fn push_string(&mut self, value: String) {
        self.string_stack[self.ssp] = value;
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
    pub fn pop_string(&mut self) -> String {
        self.ssp -= 1;
        return self.string_stack[self.ssp].clone();
    }

    pub fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
        self.isp = 0;
    }
}

impl ScriptState {

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
    pub fn pop_frame(&mut self) {
        let frame: GoSubFrame = self.frame_stack.pop().unwrap();
        self.fp -= 1;
        self.script = frame.script;
        self.pc = frame.pc;
        self.int_locals = frame.int_locals;
        self.string_locals = frame.string_locals;
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
    pub fn gosub_frame(&mut self, script: ScriptFile) {
        self.frame_stack.push(GoSubFrame {
            script: self.script.clone(),
            pc: self.pc,
            int_locals: self.int_locals.clone(),
            string_locals: self.string_locals.clone(),
        });

        self.fp += 1;
        self.pc = -1;

        if script.int_arg_count > 0 {
            for i in (0..script.int_arg_count as usize).rev() {
                self.int_locals[i] = self.pop_int();
            }
        }

        if script.string_arg_count > 0 {
            for i in (0..script.string_arg_count as usize).rev() {
                self.string_locals[i] = self.pop_string();
            }
        }

        self.script = script;
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
    pub fn goto_frame(&mut self, script: ScriptFile) {
        self.goto_frame_stack.push(GoToFrame {
            script: self.script.clone(),
            pc: self.pc,
        });

        self.goto_fp += 1;
        self.frame_stack.truncate(0);

        self.fp = 0;
        self.pc = -1;

        if script.int_arg_count > 0 {
            for i in (0..script.int_arg_count as usize).rev() {
                self.int_locals[i] = self.pop_int();
            }
        }

        if script.string_arg_count > 0 {
            for i in (0..script.string_arg_count as usize).rev() {
                self.string_locals[i] = self.pop_string();
            }
        }

        self.script = script;
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

    // testing purposes.
    pub fn pointer_debug(&self) -> String {
        return self.pointer_print(self.pointers);
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
    // pub fn protect<F>(&mut self, pointers: &[ScriptPointer], on_success: F) -> Result<(), String>
    // where
    //     F: FnOnce(&mut ScriptState) -> Result<(), String>,
    // {
    //     let pointer: ScriptPointer = pointers[self.int_operand() as usize];
    //     if self.pointer_check(pointer) {
    //         return on_success(self);
    //     }
    //     return Err(format!(
    //         "Required pointer: {}, current: {}",
    //         self.pointer_print(1 << pointer as i32),
    //         self.pointer_print(self.pointers)
    //     ));
    // }

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
    pub fn get_active_player(&self) -> Rc<Player> {
        return self.active_player.clone();
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
    pub fn set_active_player(&mut self, player: Player) {
        self.active_player = Rc::new(player);
    }

    pub fn active_player(&self) -> ScriptPointer {
        return [ScriptPointer::ActivePlayer, ScriptPointer::ActivePlayer2][0];
    }

    pub fn get_execution_state(&self) -> ScriptExecutionState {
        return self.execution_state;
    }

    pub fn set_execution_state(&mut self, state: ScriptExecutionState) {
        self.execution_state = state;
    }
}