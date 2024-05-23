import Zone from '#lostcity/engine/zone/Zone.js';

export default class ZoneManager {
    static zoneIndex(x: number, z: number, level: number): number {
        return ((x >> 3) & 0x7ff) | (((z >> 3) & 0x7ff) << 11) | ((level & 0x3) << 22);
    }

    static unpackIndex(index: number): { x: number; z: number; level: number } {
        const x: number = (index & 0x7ff) << 3;
        const z: number = ((index >> 11) & 0x7ff) << 3;
        const level: number = index >> 22;
        return { x, z, level };
    }

    zones: Map<number, Zone> = new Map();

    // ----

    getZone(absoluteX: number, absoluteZ: number, level: number): Zone {
        const zoneIndex: number = ZoneManager.zoneIndex(absoluteX, absoluteZ, level);
        let zone: Zone | undefined = this.zones.get(zoneIndex);
        if (typeof zone === 'undefined') {
            zone = new Zone(zoneIndex);
            this.zones.set(zoneIndex, zone);
        }
        return zone;
    }
}