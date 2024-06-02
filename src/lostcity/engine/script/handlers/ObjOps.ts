import InvType from '#lostcity/cache/InvType.js';
import ObjType from '#lostcity/cache/ObjType.js';
import { ParamHelper } from '#lostcity/cache/ParamHelper.js';
import ParamType from '#lostcity/cache/ParamType.js';

import World from '#lostcity/engine/World.js';

import ScriptOpcode from '#lostcity/engine/script/ScriptOpcode.js';
import ScriptPointer from '#lostcity/engine/script/ScriptPointer.js';
import { CommandHandlers } from '#lostcity/engine/script/ScriptRunner.js';

import Obj from '#lostcity/entity/Obj.js';
import { Position } from '#lostcity/entity/Position.js';

import Environment from '#lostcity/util/Environment.js';

import {
    check,
    CoordValid,
    DurationValid,
    InvTypeValid,
    ObjStackValid,
    ObjTypeValid,
    ParamTypeValid
} from '#lostcity/engine/script/ScriptValidators.js';

const ActiveObj = [ScriptPointer.ActiveObj, ScriptPointer.ActiveObj2];
const ActivePlayer = [ScriptPointer.ActivePlayer, ScriptPointer.ActivePlayer2];

const ObjOps: CommandHandlers = {
    [ScriptOpcode.OBJ_ADD]: state => {
        const [coord, objId, count, duration] = state.popInts(4);

        if (objId === -1 || count === -1) {
            return;
        }

        const objType: ObjType = check(objId, ObjTypeValid/*, ObjNotDummyValid*/);
        check(duration, DurationValid);
        const position: Position = check(coord, CoordValid);
        check(count, ObjStackValid);

        const obj: Obj = new Obj(position.level, position.x, position.z, objType.id, count);
        World.addObj(obj, state.activePlayer, duration);
        state.activeObj = obj;
        state.pointerAdd(ActiveObj[state.intOperand]);

        if (Environment.CLIRUNNER) {
            state.activePlayer.invAdd(InvType.getByName('bank')!, objType.id, count);
        }
    },

    [ScriptOpcode.OBJ_ADDALL]: state => {
        throw new Error('unimplemented');
    },

    [ScriptOpcode.OBJ_PARAM]: state => {
        const paramType: ParamType = check(state.popInt(), ParamTypeValid);

        const objType: ObjType = check(state.activeObj.type, ObjTypeValid);
        if (paramType.isString()) {
            state.pushString(ParamHelper.getStringParam(paramType.id, objType, paramType.defaultString));
        } else {
            state.pushInt(ParamHelper.getIntParam(paramType.id, objType, paramType.defaultInt));
        }
    },

    [ScriptOpcode.OBJ_NAME]: state => {
        const objType: ObjType = check(state.activeObj.type, ObjTypeValid);

        state.pushString(objType.name ?? objType.debugname ?? 'null');
    },

    [ScriptOpcode.OBJ_DEL]: state => {
        if (state.pointerGet(ActivePlayer[state.intOperand])) {
            World.removeObj(state.activeObj, state.activePlayer);
        } else {
            World.removeObj(state.activeObj, null);
        }
    },

    [ScriptOpcode.OBJ_COUNT]: state => {
        state.pushInt(check(state.activeObj.count, ObjStackValid));
    },

    [ScriptOpcode.OBJ_TYPE]: state => {
        state.pushInt(check(state.activeObj.type, ObjTypeValid).id);
    },

    [ScriptOpcode.OBJ_TAKEITEM]: state => {
        const invType: InvType = check(state.popInt(), InvTypeValid);

        const obj = state.activeObj;
        if (World.getObj(obj.x, obj.z, obj.level, obj.id)) {
            const objType: ObjType = check(obj.type, ObjTypeValid);
            state.activePlayer.playerLog('Picked up item', objType.debugname as string);

            state.activePlayer.invAdd(invType, obj.id, obj.count);
            World.removeObj(obj, state.activePlayer);
        }
    },

    [ScriptOpcode.OBJ_COORD]: state => {
        const obj = state.activeObj;
        state.pushInt(Position.packCoord(obj.level, obj.x, obj.z));
    }
};

export default ObjOps;
