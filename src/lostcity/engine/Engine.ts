import Player from '#lostcity/entity/Player.js';
import { ScriptFile } from '../../../runescript-runtime/dist/runescript-runtime.js';
import Obj from '#lostcity/entity/Obj.js';
import ObjType from '#lostcity/cache/config/ObjType.js';
import World from '#lostcity/engine/World.js';
import ScriptProvider from '#lostcity/engine/script/ScriptProvider.js';
import Environment from '#lostcity/util/Environment.js';
import EntityLifeCycle from '#lostcity/entity/EntityLifeCycle.js';

export interface Engine {
    getPlayerByUid(uid: number): Player | null;

    getScript(script: number): ScriptFile | undefined;
    getObjType(id: number): ObjType | undefined;

    isProduction(): boolean;
    isMembers(): boolean;

    objAddAll(x: number, y: number, z: number, id: number, count: number, duration: number, stackable: boolean): Obj;
}

export default class ScriptEngine implements Engine {
    getPlayerByUid(uid: number): Player | null {
        return World.getPlayerByUid(uid);
    }

    getObjType(id: number): ObjType | undefined {
        return ObjType.get(id);
    }

    getScript(script: number): ScriptFile | undefined {
        return ScriptProvider.get(script)?._clone();
    }

    isProduction(): boolean {
        return Environment.NODE_PRODUCTION;
    }

    isMembers(): boolean {
        return Environment.NODE_MEMBERS;
    }

    objAddAll(x: number, y: number, z: number, id: number, count: number, duration: number, stackable: boolean): Obj {
        if (!stackable || count === 1) {
            let obj: Obj = new Obj(y, x, z, EntityLifeCycle.DESPAWN, id, 1);
            World.addObj(obj, -1, duration);
            for (let i = 1; i < count; i++) {
                obj = new Obj(y, x, z, EntityLifeCycle.DESPAWN, id, 1);
                World.addObj(obj, -1, duration);
            }
            return obj;
        }
        const obj: Obj = new Obj(y, x, z, EntityLifeCycle.DESPAWN, id, count);
        World.addObj(obj, -1, duration);
        return obj;
    }
}
