import Packet from '#jagex2/io/Packet.js';
import ServerProt from '#lostcity/server/ServerProt.js';

export abstract class ZoneEvent {
    readonly out: Packet;

    protected constructor(prot: ServerProt, size: number) {
        this.out = new Packet(new Uint8Array(size));
        this.out.p1(prot.id);
    }
}

export class MapAnimEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, id: number, height: number, delay: number) {
        super(ServerProt.MAP_ANIM, 1 + 1 + 2 + 1 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p2(id);
        this.out.p1(height);
        this.out.p2(delay);
    }
}

export class MapProjAnimEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, dstX: number, dstZ: number, target: number, spotanim: number, srcHeight: number, dstHeight: number, startDelay: number, endDelay: number, peak: number, arc: number) {
        super(ServerProt.MAP_PROJANIM, 1 + 1 + 1 + 1 + 2 + 2 + 1 + 1 + 2 + 2 + 1 + 1);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p1(dstX - srcX);
        this.out.p1(dstZ - srcZ);
        this.out.p2(target); // 0: coord, > 0: npc, < 0: player
        this.out.p2(spotanim);
        this.out.p1(srcHeight);
        this.out.p1(dstHeight);
        this.out.p2(startDelay);
        this.out.p2(endDelay);
        this.out.p1(peak);
        this.out.p1(arc);
    }
}

export class LocAddChangeEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, id: number, shape: number, angle: number) {
        super(ServerProt.LOC_ADD_CHANGE, 1 + 1 + 1 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p1((shape << 2) | (angle & 3));
        this.out.p2(id);
    }
}

export class LocDelEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, shape: number, angle: number) {
        super(ServerProt.LOC_DEL, 1 + 1 + 1 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p1((shape << 2) | (angle & 3));
    }
}

export class LocMergeEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, shape: number, angle: number, id: number, startCycle: number, endCycle: number, pid: number, east: number, south: number, west: number, north: number) {
        super(ServerProt.LOC_MERGE, 1 + 1 + 1 + 2 + 2 + 2 + 2 + 1 + 1 + 1 + 1);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p1((shape << 2) | (angle & 3));
        this.out.p2(id);
        this.out.p2(startCycle);
        this.out.p2(endCycle);
        this.out.p2(pid);
        this.out.p1(east - srcX);
        this.out.p1(south - srcZ);
        this.out.p1(west - srcX);
        this.out.p1(north - srcZ);
    }
}

export class LocAnimEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, shape: number, angle: number, id: number) {
        super(ServerProt.LOC_ANIM, 1 + 1 + 1 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p1((shape << 2) | (angle & 3));
        this.out.p2(id);
    }
}

export class ObjAddEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, id: number, count: number) {
        super(ServerProt.OBJ_ADD, 1 + 1 + 2 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p2(id);

        if (count > 65535) {
            count = 65535;
        }
        this.out.p2(count);
    }
}

export class ObjCountEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, id: number, count: number) {
        super(ServerProt.OBJ_COUNT, 1 + 1 + 2 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p2(id);

        if (count > 65535) {
            count = 65535;
        }
        this.out.p2(count);
    }
}

export class ObjDelEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, id: number) {
        super(ServerProt.OBJ_DEL, 1 + 1 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p2(id);
    }
}

export class ObjRevealEvent extends ZoneEvent {
    constructor(srcX: number, srcZ: number, id: number, count: number, receiverId: number) {
        super(ServerProt.OBJ_REVEAL, 1 + 1 + 2 + 2 + 2);
        this.out.p1(((srcX & 0x7) << 4) | (srcZ & 0x7));
        this.out.p2(id);
        this.out.p2(count);
        this.out.p2(receiverId);
    }
}

