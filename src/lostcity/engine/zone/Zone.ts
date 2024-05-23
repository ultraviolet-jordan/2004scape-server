import Player from '#lostcity/entity/Player.js';
import Npc from '#lostcity/entity/Npc.js';
import Obj from '#lostcity/entity/Obj.js';
import Loc from '#lostcity/entity/Loc.js';
import Entity from '#lostcity/entity/Entity.js';
import World from '#lostcity/engine/World.js';

import {
    LocAddChangeEvent,
    LocAnimEvent,
    LocDelEvent,
    LocMergeEvent,
    MapAnimEvent,
    MapProjAnimEvent,
    ObjAddEvent,
    ObjCountEvent,
    ObjDelEvent,
    ObjRevealEvent,
    ZoneEvent
} from '#lostcity/engine/zone/ZoneEvent.js';

export default class Zone {
    // constructor
    private readonly players: Player[];
    private readonly npcs: Npc[];
    private readonly objs: Obj[];
    private readonly locs: Loc[];

    private readonly spawnedObjs: Obj[];
    private readonly spawnedLocs: Loc[];

    private readonly index: number;

    // runtime
    private events: ZoneEvent[] = [];

    constructor(index: number) {
        this.players = [];
        this.npcs = [];
        this.objs = [];
        this.locs = [];
        this.spawnedObjs = [];
        this.spawnedLocs = [];
        this.index = index;
    }

    playerAdd(player: Player): void {
        this.players.push(player);
    }

    playerDel(player: Player): void {
        const index: number = this.players.indexOf(player);
        if (index !== -1) {
            this.players.splice(index, 1);
        }
    }

    npcAdd(npc: Npc, despawn: number): void {
        this.npcs.push(npc);
        this.add(npc, despawn);
    }

    npcDel(npc: Npc, respawn: number): void {
        const index: number = this.npcs.indexOf(npc);
        if (index !== -1) {
            this.npcs.splice(index, 1);
        }
        this.delete(npc, respawn);
    }

    reset(): void {
        this.events = [];
    }

    mapAnim(srcX: number, srcZ: number, id: number, height: number, delay: number): void {
        this.events.push(new MapAnimEvent(srcX, srcZ, id, height, delay));
    }

    mapProjAnim(srcX: number, srcZ: number, dstX: number, dstZ: number, target: number, spotanim: number, srcHeight: number, dstHeight: number, startDelay: number, endDelay: number, peak: number, arc: number): void {
        this.events.push(new MapProjAnimEvent(srcX, srcZ, dstX, dstZ, target, spotanim, srcHeight, dstHeight, startDelay, endDelay, peak, arc));
    }

    locAddChange(loc: Loc, add: boolean): void {
        this.events.push(new LocAddChangeEvent(loc.x, loc.z, loc.type, loc.shape, loc.angle));
    }

    locDel(srcX: number, srcZ: number, shape: number, angle: number): void {
        this.events.push(new LocDelEvent(srcX, srcZ, shape, angle));
    }

    locMerge(srcX: number, srcZ: number, shape: number, angle: number, id: number, startCycle: number, endCycle: number, pid: number, east: number, south: number, west: number, north: number): void {
        this.events.push(new LocMergeEvent(srcX, srcZ, shape, angle, id, startCycle, endCycle, pid, east, south, west, north));
    }

    locAnim(srcX: number, srcZ: number, shape: number, angle: number, id: number): void {
        this.events.push(new LocAnimEvent(srcX, srcZ, shape, angle, id));
    }

    objAdd(srcX: number, srcZ: number, id: number, count: number): void {
        this.events.push(new ObjAddEvent(srcX, srcZ, id, count));
    }

    objCount(srcX: number, srcZ: number, id: number, count: number): void {
        this.events.push(new ObjCountEvent(srcX, srcZ, id, count));
    }

    objDel(srcX: number, srcZ: number, id: number): void {
        this.events.push(new ObjDelEvent(srcX, srcZ, id));
    }

    objReveal(srcX: number, srcZ: number, id: number, count: number, receiverId: number): void {
        this.events.push(new ObjRevealEvent(srcX, srcZ, id, count, receiverId));
    }

    // addEntity(entity: Entity, despawn: number): void {
    //     if (entity instanceof Player) {
    //         this.players.add(entity);
    //     } else if (entity instanceof Npc) {
    //         this.npcs.add(entity);
    //     } else if (entity instanceof Loc) {
    //         this.locs.add(entity);
    //     } else if (entity instanceof Obj) {
    //         this.objs.add(entity);
    //     } else {
    //         throw new Error('something rlly bad happened here dAWG');
    //     }
    //     this.add(entity, despawn);
    // }
    //
    // deleteEntity(entity: Entity, respawn: number): void {
    //     if (entity instanceof Player) {
    //         this.players.delete(entity);
    //     } else if (entity instanceof Npc) {
    //         this.npcs.delete(entity);
    //     } else if (entity instanceof Loc) {
    //         this.locs.delete(entity);
    //     } else if (entity instanceof Obj) {
    //         this.objs.delete(entity);
    //     } else {
    //         throw new Error('something rlly bad happened here cat?');
    //     }
    //     this.delete(entity, respawn);
    // }

    private add(entity: Entity, despawn: number): void {
        entity.despawn = World.currentTick + despawn;
        entity.respawn = -1;
    }

    private delete(entity: Entity, respawn: number): void {
        entity.despawn = -1;
        entity.respawn = World.currentTick + respawn;
    }

    *getAllPlayers(): IterableIterator<Player> {
        for (const player of this.players) {
            yield player;
        }
    }

    *getAllNpcs(): IterableIterator<Npc> {
        for (const npc of this.npcs) {
            yield npc;
        }
    }

    *getActiveNpcs(): IterableIterator<Npc> {
        for (const npc of this.npcs) {
            if (npc.despawn !== -1) {
                if (npc.respawn !== -1) {
                    throw new Error('this is really bad npc');
                }
                continue;
            }
            yield npc;
        }
    }

    *getAllLocs(): IterableIterator<Loc> {
        for (const loc of this.locs) {
            yield loc;
        }
        for (const loc of this.spawnedLocs) {
            yield loc;
        }
    }

    *getActiveLocs(): IterableIterator<Loc> {
        for (const loc of this.locs) {
            if (loc.despawn !== -1) {
                if (loc.respawn !== -1) {
                    throw new Error('this is really bad loc');
                }
                continue;
            }
            yield loc;
        }
        for (const loc of this.spawnedLocs) {
            if (loc.despawn !== -1) {
                if (loc.respawn !== -1) {
                    throw new Error('this is really bad loc');
                }
                continue;
            }
            yield loc;
        }
    }

    *getAllObjs(): IterableIterator<Obj> {
        for (const obj of this.objs) {
            yield obj;
        }
        for (const obj of this.spawnedObjs) {
            yield obj;
        }
    }

    *getActiveObjs(): IterableIterator<Obj> {
        for (const obj of this.objs) {
            if (obj.despawn !== -1) {
                if (obj.respawn !== -1) {
                    throw new Error('this is really bad obj');
                }
                continue;
            }
            yield obj;
        }
        for (const obj of this.spawnedObjs) {
            if (obj.despawn !== -1) {
                if (obj.respawn !== -1) {
                    throw new Error('this is really bad obj');
                }
                continue;
            }
            yield obj;
        }
    }
}