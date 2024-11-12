import { ScriptExecutionState, ScriptFile, ScriptPointer, ScriptState } from '../../../../runescript-runtime/dist/runescript-runtime.js';

import Entity from '#lostcity/entity/Entity.js';
import { ScriptArgument } from '#lostcity/entity/EntityQueueRequest.js';
import Loc from '#lostcity/entity/Loc.js';
import Obj from '#lostcity/entity/Obj.js';
import Npc from '#lostcity/entity/Npc.js';
import Player from '#lostcity/entity/Player.js';
import { printWarning } from '#lostcity/util/Logger.js';
import World from '#lostcity/engine/World.js';

// script executor
export default class ScriptRunner {
    static init(script: ScriptFile, self: Entity | null = null, target: Entity | null = null, args: ScriptArgument[] | null = []) {
        const intLocals: number[] = [];
        const stringLocals: string[] = [];

        if (args !== null && args.length > 0) {
            for (let i = 0; i < args.length; i++) {
                const arg = args[i];

                if (typeof arg === 'number') {
                    intLocals.push(arg);
                } else {
                    stringLocals.push(arg);
                }
            }
        }

        const state = new ScriptState(script, Int32Array.from(intLocals), stringLocals);
        state.self = self;

        if (self instanceof Player) {
            state._activePlayer = self;
            state.pointerAdd(ScriptPointer.ActivePlayer);
        } else if (self instanceof Npc) {
            state._activeNpc = self;
            state.pointerAdd(ScriptPointer.ActiveNpc);
        } else if (self instanceof Loc) {
            state._activeLoc = self;
            state.pointerAdd(ScriptPointer.ActiveLoc);
        } else if (self instanceof Obj) {
            state._activeObj = self;
            state.pointerAdd(ScriptPointer.ActiveObj);
        }

        if (target instanceof Player) {
            if (self instanceof Player) {
                state._activePlayer2 = target;
                state.pointerAdd(ScriptPointer.ActivePlayer2);
            } else {
                state._activePlayer = target;
                state.pointerAdd(ScriptPointer.ActivePlayer);
            }
        } else if (target instanceof Npc) {
            if (self instanceof Npc) {
                state._activeNpc2 = target;
                state.pointerAdd(ScriptPointer.ActiveNpc2);
            } else {
                state._activeNpc = target;
                state.pointerAdd(ScriptPointer.ActiveNpc);
            }
        } else if (target instanceof Loc) {
            if (self instanceof Loc) {
                state._activeLoc2 = target;
                state.pointerAdd(ScriptPointer.ActiveLoc2);
            } else {
                state._activeLoc = target;
                state.pointerAdd(ScriptPointer.ActiveLoc);
            }
        } else if (target instanceof Obj) {
            if (self instanceof Obj) {
                state._activeObj2 = target;
                state.pointerAdd(ScriptPointer.ActiveObj2);
            } else {
                state._activeObj = target;
                state.pointerAdd(ScriptPointer.ActiveObj);
            }
        }

        return state;
    }

    static execute(state: ScriptState, reset = false, benchmark = false): ScriptExecutionState {
        if (state.execution !== ScriptExecutionState.Running) {
            // state.executionHistory.push(state.execution);
        }

        const start: number = performance.now() * 1000;
        state.execute(World);
        const time: number = (performance.now() * 1000 - start) | 0;
        const message: string = `Warning [cpu time]: Script: ${state.scriptName}, time: ${time}us, opcount: ${state.opcount}`;
        if (state.self instanceof Player) {
            state.self.wrappedMessageGame(message);
        } else {
            printWarning(message);
        }

        if (state.execution === ScriptExecutionState.Aborted) {
            // TODO: move all the commented code into the rust lib.

            // print the last opcode executed
            // const opcodes = state.opcodes;
            // if (state.pc >= 0 && state.pc < opcodes.length) {
            //     const opcode = opcodes[state.pc];
            //
            //     let secondary = state.intOperand;
            //     if (opcode === ScriptOpcode.POP_VARP || opcode === ScriptOpcode.POP_VARN || opcode === ScriptOpcode.PUSH_VARP || opcode === ScriptOpcode.PUSH_VARN) {
            //         secondary = (state.intOperand >> 16) & 0x1;
            //     } else if (opcode <= ScriptOpcode.POP_ARRAY_INT) {
            //         secondary = 0;
            //     }
            //
            //     err.message = ScriptOpcode[opcode].toLowerCase() + ' ' + err.message;
            //     if (secondary) {
            //         err.message = '.' + err.message;
            //     }
            // }

            if (state.self instanceof Player) {
                state.self.wrappedMessageGame(state.error);

                // let trace = 1;
                // for (let i = state.fp; i > 0; i--) {
                //     const frame = state.frames[i];
                //     if (frame) {
                //         trace++;
                //         state.self.wrappedMessageGame(`    ${trace}: ${frame.script.name} - ${frame.script.fileName}:${frame.script.lineNumber(frame.pc)}`);
                //     }
                // }
                // for (let i = state.debugFp; i >= 0; i--) {
                //     const frame = state.debugFrames[i];
                //     if (frame) {
                //         trace++;
                //         state.self.wrappedMessageGame(`    ${trace}: ${frame.script.name} - ${frame.script.fileName}:${frame.script.lineNumber(frame.pc)}`);
                //     }
                // }
            }

            // console.error(state.error);

            // let trace = 1;
            // for (let i = state.fp; i > 0; i--) {
            //     const frame = state.frames[i];
            //     if (frame) {
            //         trace++;
            //         console.error(`    ${trace}: ${frame.script.name} - ${frame.script.fileName}:${frame.script.lineNumber(frame.pc)}`);
            //     }
            // }
            // for (let i = state.debugFp; i >= 0; i--) {
            //     const frame = state.debugFrames[i];
            //     if (frame) {
            //         trace++;
            //         console.error(`    ${trace}: ${frame.script.name} - ${frame.script.fileName}:${frame.script.lineNumber(frame.pc)}`);
            //     }
            // }

            state.free();
            return ScriptExecutionState.Aborted;
        }

        if (state.execution === ScriptExecutionState.Finished) {
            state.free();
            return ScriptExecutionState.Finished;
        }

        return state.execution;
    }
}
