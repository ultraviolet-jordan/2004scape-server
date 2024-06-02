import ObjType from '#lostcity/cache/ObjType.js';
import { ParamHelper } from '#lostcity/cache/ParamHelper.js';
import ParamType from '#lostcity/cache/ParamType.js';

import ScriptOpcode from '#lostcity/engine/script/ScriptOpcode.js';
import { CommandHandlers } from '#lostcity/engine/script/ScriptRunner.js';

import {check, ObjTypeValid, ParamTypeValid} from '#lostcity/engine/script/ScriptValidators.js';

const ObjConfigOps: CommandHandlers = {
    [ScriptOpcode.OC_NAME]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushString(objType.name ?? objType.debugname ?? 'null');
    },

    [ScriptOpcode.OC_PARAM]: state => {
        const [objId, paramId] = state.popInts(2);

        const objType: ObjType = check(objId, ObjTypeValid);
        const paramType: ParamType = check(paramId, ParamTypeValid);
        if (paramType.isString()) {
            state.pushString(ParamHelper.getStringParam(paramId, objType, paramType.defaultString));
        } else {
            state.pushInt(ParamHelper.getIntParam(paramId, objType, paramType.defaultInt));
        }
    },

    [ScriptOpcode.OC_CATEGORY]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.category);
    },

    [ScriptOpcode.OC_DESC]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushString(objType.desc ?? 'null');
    },

    [ScriptOpcode.OC_MEMBERS]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.members ? 1 : 0);
    },

    [ScriptOpcode.OC_WEIGHT]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.weight);
    },

    [ScriptOpcode.OC_WEARPOS]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.wearpos);
    },

    [ScriptOpcode.OC_WEARPOS2]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.wearpos2);
    },

    [ScriptOpcode.OC_WEARPOS3]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.wearpos3);
    },

    [ScriptOpcode.OC_COST]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.cost);
    },

    [ScriptOpcode.OC_TRADEABLE]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.tradeable ? 1 : 0);
    },

    [ScriptOpcode.OC_DEBUGNAME]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushString(objType.debugname ?? 'null');
    },

    [ScriptOpcode.OC_CERT]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        if (objType.certtemplate == -1 && objType.certlink >= 0) {
            state.pushInt(objType.certlink);
        } else {
            state.pushInt(objType.id);
        }
    },

    [ScriptOpcode.OC_UNCERT]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        if (objType.certtemplate >= 0 && objType.certlink >= 0) {
            state.pushInt(objType.certlink);
        } else {
            state.pushInt(objType.id);
        }
    },

    [ScriptOpcode.OC_STACKABLE]: state => {
        const objType: ObjType = check(state.popInt(), ObjTypeValid);

        state.pushInt(objType.stackable ? 1 : 0);
    }
};

export default ObjConfigOps;
