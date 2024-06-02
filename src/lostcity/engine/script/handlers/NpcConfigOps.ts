import NpcType from '#lostcity/cache/NpcType.js';
import { ParamHelper } from '#lostcity/cache/ParamHelper.js';
import ParamType from '#lostcity/cache/ParamType.js';

import ScriptOpcode from '#lostcity/engine/script/ScriptOpcode.js';
import { CommandHandlers } from '#lostcity/engine/script/ScriptRunner.js';

import {
    check,
    NpcTypeValid,
    NumberNotNull,
    ParamTypeValid
} from '#lostcity/engine/script/ScriptValidators.js';

const NpcConfigOps: CommandHandlers = {
    [ScriptOpcode.NC_NAME]: state => {
        const npcType: NpcType = check(state.popInt(), NpcTypeValid);

        state.pushString(npcType.name ?? npcType.debugname ?? 'null');
    },

    [ScriptOpcode.NC_PARAM]: state => {
        const [npcId, paramId] = state.popInts(2);

        const npcType: NpcType = check(npcId, NpcTypeValid);
        const paramType: ParamType = check(paramId, ParamTypeValid);
        if (paramType.isString()) {
            state.pushString(ParamHelper.getStringParam(paramType.id, npcType, paramType.defaultString));
        } else {
            state.pushInt(ParamHelper.getIntParam(paramType.id, npcType, paramType.defaultInt));
        }
    },

    [ScriptOpcode.NC_CATEGORY]: state => {
        const npcType: NpcType = check(state.popInt(), NpcTypeValid);

        state.pushInt(npcType.category);
    },

    [ScriptOpcode.NC_DESC]: state => {
        const npcType: NpcType = check(state.popInt(), NpcTypeValid);

        state.pushString(npcType.desc ?? 'null');
    },

    [ScriptOpcode.NC_DEBUGNAME]: state => {
        const npcType: NpcType = check(state.popInt(), NpcTypeValid);

        state.pushString(npcType.debugname ?? 'null');
    },

    [ScriptOpcode.NC_OP]: state => {
        const [npcId, op] = state.popInts(2);

        const npcType: NpcType = check(npcId, NpcTypeValid);
        check(op, NumberNotNull);

        if (!npcType.op) {
            state.pushString('');
            return;
        }
        state.pushString(npcType.op[op - 1] ?? '');
    }
};

export default NpcConfigOps;
