import LocType from '#lostcity/cache/LocType.js';
import { ParamHelper } from '#lostcity/cache/ParamHelper.js';
import ParamType from '#lostcity/cache/ParamType.js';

import ScriptOpcode from '#lostcity/engine/script/ScriptOpcode.js';
import { CommandHandlers } from '#lostcity/engine/script/ScriptRunner.js';

import {check, LocTypeValid, ParamTypeValid} from '#lostcity/engine/script/ScriptValidators.js';

const LocConfigOps: CommandHandlers = {
    [ScriptOpcode.LC_NAME]: state => {
        const locType: LocType = check(state.popInt(), LocTypeValid);

        state.pushString(locType.name ?? locType.debugname ?? 'null');
    },

    [ScriptOpcode.LC_PARAM]: state => {
        const [locId, paramId] = state.popInts(2);

        const locType: LocType = check(locId, LocTypeValid);
        const paramType: ParamType = check(paramId, ParamTypeValid);
        if (paramType.isString()) {
            state.pushString(ParamHelper.getStringParam(paramId, locType, paramType.defaultString));
        } else {
            state.pushInt(ParamHelper.getIntParam(paramId, locType, paramType.defaultInt));
        }
    },

    [ScriptOpcode.LC_CATEGORY]: state => {
        const locType: LocType = check(state.popInt(), LocTypeValid);

        state.pushInt(locType.category);
    },

    [ScriptOpcode.LC_DESC]: state => {
        const locType: LocType = check(state.popInt(), LocTypeValid);

        state.pushString(locType.desc ?? 'null');
    },

    [ScriptOpcode.LC_DEBUGNAME]: state => {
        const locType: LocType = check(state.popInt(), LocTypeValid);

        state.pushString(locType.debugname ?? 'null');
    },

    [ScriptOpcode.LC_WIDTH]: state => {
        const locType: LocType = check(state.popInt(), LocTypeValid);

        state.pushInt(locType.width);
    },

    [ScriptOpcode.LC_LENGTH]: state => {
        const locType: LocType = check(state.popInt(), LocTypeValid);

        state.pushInt(locType.length);
    }
};

export default LocConfigOps;
