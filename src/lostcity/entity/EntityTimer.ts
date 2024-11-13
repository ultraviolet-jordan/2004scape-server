import { ScriptArgument } from '#lostcity/entity/EntityQueueRequest.js';
import { PlayerTimerType, ScriptFile } from '../../../runescript-runtime/dist/runescript-runtime.js';

export enum NpcTimerType {
    NPC
}

export type TimerType = NpcTimerType | PlayerTimerType;

export interface EntityTimer {
    /**
     * The type of the timer.
     */
    type: TimerType;

    /**
     * The script to execute.
     */
    script: ScriptFile;

    /**
     * The arguments to execute the script with.
     */
    args: ScriptArgument[] | null;

    /**
     * The time interval between executions.
     */
    interval: number;

    /**
     * Tracks the time until execution.
     */
    clock: number;
}
