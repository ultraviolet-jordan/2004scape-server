import Player from '#lostcity/entity/Player.js';
import { ScriptFile } from '../../../runescript-runtime/dist/runescript-runtime.js';
import Obj from '#lostcity/entity/Obj.js';

interface Engine {
    getPlayerByUid(uid: number): Player | null;

    getScript(script: number): ScriptFile | undefined;

    isProduction(): boolean;

    objAddAll(coord: number, id: number, count: number, duration: number): Obj | null;
}

export default Engine;
