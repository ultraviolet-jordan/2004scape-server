import Packet from '#/io/Packet.js';
import ZoneProt from '#/network/rs225/server/prot/ZoneProt.js';
import ZoneMessageEncoder from '#/network/server/codec/ZoneMessageEncoder.js';
import LocAnim from '#/network/server/model/LocAnim.js';

export default class LocAnimEncoder extends ZoneMessageEncoder<LocAnim> {
    prot = ZoneProt.LOC_ANIM;

    encode(buf: Packet, message: LocAnim): void {
        buf.p1(message.coord);
        buf.p1((message.shape << 2) | (message.angle & 0x3));
        buf.p2(message.seq);
    }
}
