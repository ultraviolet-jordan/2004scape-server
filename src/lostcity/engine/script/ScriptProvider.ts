import Packet from '#jagex2/io/Packet.js';

import ServerTriggerType from '#lostcity/engine/script/ServerTriggerType.js';

import {TargetOp} from '#lostcity/entity/PathingEntity.js';
import { printFatalError, printWarning } from '#lostcity/util/Logger.js';
import { ScriptFile, ScriptInfo } from '../../../../runescript-runtime/dist/runescript-runtime.js';
import ScriptOpcode from '#lostcity/engine/script/ScriptOpcode.js';

function isLargeOperand(opcode: number): boolean {
    if (opcode > 100) {
        return false;
    }

    switch (opcode) {
        case ScriptOpcode.RETURN:
        case ScriptOpcode.POP_INT_DISCARD:
        case ScriptOpcode.POP_STRING_DISCARD:
        case ScriptOpcode.GOSUB:
        case ScriptOpcode.JUMP:
            return false;
    }

    return true;
}

// maintains a list of scripts (id <-> name)
export default class ScriptProvider {
    /**
     * The expected version of the script compiler that the runtime should be loading scripts from.
     */
    public static readonly COMPILER_VERSION = 20;

    /**
     * Array of loaded scripts.
     */
    private static scripts: ScriptFile[] = [];

    /**
     * Mapping of unique trigger + type/category/global key to script.
     */
    private static scriptLookup = new Map<number, ScriptFile>();

    /**
     * Mapping of script names to its id.
     */
    private static scriptNames = new Map<string, number>();

    /**
     * Loads all scripts from `dir`.
     *
     * @param dir The directory that holds the script.{dat,idx} files.
     * @returns The number of scripts loaded.
     */
    static load(dir: string): number {
        const dat = Packet.load(`${dir}/server/script.dat`);
        const idx = Packet.load(`${dir}/server/script.idx`);
        return this.parse(dat, idx);
    }

    static async loadAsync(dir: string): Promise<number> {
        const [dat, idx] = await Promise.all([Packet.loadAsync(`${dir}/server/script.dat`), Packet.loadAsync(`${dir}/server/script.idx`)]);
        return this.parse(dat, idx);
    }

    static parse(dat: Packet, idx: Packet): number {
        if (!dat.data.length || !idx.data.length) {
            printFatalError('No server cache found. Please build the cache first.');
        }

        const entries = dat.g2();
        idx.pos += 2;

        const version = dat.g4();
        if (version !== ScriptProvider.COMPILER_VERSION) {
            printFatalError('\nFatal: Scripts were compiled with an older RuneScript compiler. Please update it, try `npm run build` and then restart the server.');
        }

        const scripts = new Array<ScriptFile>(entries);
        const scriptNames = new Map<string, number>();
        const scriptLookup = new Map<number, ScriptFile>();

        let loaded = 0;
        for (let id = 0; id < entries; id++) {
            const size = idx.g2();
            if (size === 0) {
                continue;
            }

            try {
                const data: Uint8Array = new Uint8Array(size);
                dat.gdata(data, 0, data.length);
                const script: ScriptFile = this.decode(id, new Packet(data));
                scripts[id] = script;
                scriptNames.set(script.name, id);

                // add the script to lookup table if the value isn't -1
                if (script.lookup !== 0xffffffff) {
                    scriptLookup.set(script.lookup, script);
                }

                loaded++;
            } catch (err) {
                console.error(err);
                printWarning(`Warning: Failed to load script ${id}, something may have been partially written`);
                return -1;
            }
        }

        ScriptProvider.scripts = scripts;
        ScriptProvider.scriptNames = scriptNames;
        ScriptProvider.scriptLookup = scriptLookup;
        return loaded;
    }

    static decode(id: number, stream: Packet): ScriptFile {
        const length: number = stream.data.length;
        if (length < 16) {
            throw new Error('Invalid script file (minimum length)');
        }

        stream.pos = length - 2;

        const trailerLen: number = stream.g2();
        const trailerPos: number = length - trailerLen - 12 - 2;

        if (trailerPos < 0 || trailerPos >= length) {
            throw new Error('Invalid script file (bad trailer pos)');
        }

        stream.pos = trailerPos;

        // const script = new ScriptFile(id);
        const instructions: number = stream.g4(); // we don't need to preallocate anything in JS, but still need to read it
        const intLocalCount: number = stream.g2();
        const stringLocalCount: number = stream.g2();
        const intArgCount: number = stream.g2();
        const stringArgCount: number = stream.g2();

        const switchTable: Map<number, Map<number, number>> = new Map();
        const switches: number = stream.g1();
        for (let i = 0; i < switches; i++) {
            const count: number = stream.g2();
            const table: Map<number, number> = new Map();
            for (let j = 0; j < count; j++) {
                table.set(stream.g4(), stream.g4());
            }
            switchTable.set(i, table);
        }

        stream.pos = 0;

        const name: string = stream.gjstr(0);
        const path: string = stream.gjstr(0);
        const lookup = stream.g4();

        const parameterTypeCount: number = stream.g1();
        const params: Uint8Array = new Uint8Array(parameterTypeCount);
        for (let i = 0; i < parameterTypeCount; i++) {
            params[i] = stream.g1();
        }

        const lineNumberTableLength = stream.g2();
        const pcs = new Int32Array(lineNumberTableLength);
        const lines = new Int32Array(lineNumberTableLength);
        for (let i = 0; i < lineNumberTableLength; i++) {
            pcs[i] = stream.g4();
            lines[i] = stream.g4();
        }

        const info: ScriptInfo = new ScriptInfo(name, path, lookup, params, pcs, lines);

        const codes: Uint16Array = new Uint16Array(instructions);
        const intOperands: Uint32Array = new Uint32Array(instructions);
        const stringOperands: Array<string> = new Array(instructions);

        let pc: number = 0;
        while (trailerPos > stream.pos) {
            const opcode: number = stream.g2();

            if (opcode === ScriptOpcode.PUSH_CONSTANT_STRING) {
                stringOperands[pc] = stream.gjstr(0);
            } else if (isLargeOperand(opcode)) {
                intOperands[pc] = stream.g4();
            } else {
                intOperands[pc] = stream.g1();
            }
            codes[pc++] = opcode;
        }

        return new ScriptFile(id, intLocalCount, stringLocalCount, intArgCount, stringArgCount, switchTable, info, codes, intOperands, stringOperands);
    }

    /**
     * Finds a script by `id`.
     * @param id The script id to find.
     * @returns The script.
     */
    static get(id: number): ScriptFile | undefined {
        return this.scripts[id];
    }

    /**
     * Finds a script by `name`.
     * @param name The script name to find.
     * @returns The script.
     */
    static getByName(name: string): ScriptFile | undefined {
        const id = ScriptProvider.scriptNames.get(name);
        if (id === undefined) {
            return undefined;
        }
        return ScriptProvider.scripts[id];
    }

    /**
     * Used to look up a script by the `type` and `category`.
     *
     * This function will attempt to search for a script given the specific `type`,
     * if one is not found it attempts one for `category`, and if still not found
     * it will attempt for the global script.
     *
     * @param trigger The script trigger to find.
     * @param type The script subject type id.
     * @param category The script subject category id.
     */
    static getByTrigger(trigger: TargetOp, type: number = -1, category: number = -1): ScriptFile | undefined {
        let script = ScriptProvider.scriptLookup.get(trigger | (0x2 << 8) | (type << 10));
        if (script) {
            return script;
        }
        script = ScriptProvider.scriptLookup.get(trigger | (0x1 << 8) | (category << 10));
        if (script) {
            return script;
        }
        return ScriptProvider.scriptLookup.get(trigger);
    }

    /**
     * Used to look up a script by a specific combo. Does not attempt any other combinations.
     *
     * If `type` is not `-1`, only the `type` specific script will be looked up. Likewise
     * for `category`. If both `type` and `category` are `-1`, then only the global script
     * will be looked up.
     *
     * @param trigger The script trigger to find.
     * @param type The script subject type id.
     * @param category The script subject category id.
     */
    static getByTriggerSpecific(trigger: ServerTriggerType, type: number = -1, category: number = -1): ScriptFile | undefined {
        if (type !== -1) {
            return ScriptProvider.scriptLookup.get(trigger | (0x2 << 8) | (type << 10));
        } else if (category !== -1) {
            return ScriptProvider.scriptLookup.get(trigger | (0x1 << 8) | (category << 10));
        }
        return ScriptProvider.scriptLookup.get(trigger);
    }
}
