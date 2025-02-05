import MessageHandler from '#/network/client/handler/MessageHandler.js';
import EventTracking from '#/network/client/model/EventTracking.js';
import Player from '#/engine/entity/Player.js';
import Packet from '#/io/Packet.js';
import ReportEvent from '#/engine/entity/tracking/ReportEvent.js';
import InputTracking from '#/engine/entity/tracking/InputTracking.js';

enum InputTrackingEvent {
    MOUSEDOWNL = 1,
    MOUSEDOWNR = 2,
    MOUSEUPL = 3,
    MOUSEUPR = 4,
    MOUSEMOVE1 = 5,
    MOUSEMOVE2 = 6,
    MOUSEMOVE3 = 7,
    KEYDOWN = 8,
    KEYUP = 9,
    FOCUS = 10,
    BLUR = 11,
    MOUSEENTER = 12,
    MOUSELEAVE = 13
}

export default class EventTrackingHandler extends MessageHandler<EventTracking> {
    handle(message: EventTracking, player: Player): boolean {
        const bytes: Uint8Array = message.bytes;
        if (bytes.length === 0 || bytes.length > 5000) {
            return false;
        }
        const input: InputTracking = player.input;
        if (!input.tracking()) {
            return false;
        }
        const buf: Packet = new Packet(bytes);
        while (buf.available > 0 && input.tracking()) {
            const event: number = buf.g1();
            if (event === InputTrackingEvent.MOUSEDOWNL || event === InputTrackingEvent.MOUSEDOWNR) {
                this.onmousedown(input, buf);
            } else if (event === InputTrackingEvent.MOUSEUPL || event === InputTrackingEvent.MOUSEUPR) {
                this.onmouseup(input, buf);
            } else if (event === InputTrackingEvent.MOUSEMOVE1) {
                this.onmousemove(input, buf, 1);
            } else if (event === InputTrackingEvent.MOUSEMOVE2) {
                this.onmousemove(input, buf, 2);
            } else if (event === InputTrackingEvent.MOUSEMOVE3) {
                this.onmousemove(input, buf, 3);
            } else if (event === InputTrackingEvent.KEYDOWN) {
                this.onkeydown(input, buf);
            } else if (event === InputTrackingEvent.KEYUP) {
                this.onkeyup(input, buf);
            } else if (event === InputTrackingEvent.FOCUS) {
                this.onfocus(buf);
            } else if (event === InputTrackingEvent.BLUR) {
                this.onblur(buf);
            } else if (event === InputTrackingEvent.MOUSEENTER) {
                this.onmouseenter(buf);
            } else if (event === InputTrackingEvent.MOUSELEAVE) {
                this.onmouseleave(input, buf);
            } else {
                break;
            }
        }
        return true;
    }

    onmousedown(input: InputTracking, buf: Packet): void {
        const time: number = buf.g1();
        const pos: number = buf.g3();
        const x: number = pos & 0x3ff;
        const y: number = (pos >> 10) & 0x3ff;
        input.onmousedown(time, x, y);

        // if (input.enabled) {
        //     input.disable(true);
        // } else {
        //     input.report(ReportEvent.ACTIVE);
        // }
    }

    onmouseup(input: InputTracking, buf: Packet): void {
        buf.pos += 1; // time

        // if (input.enabled) {
        //     input.disable(true);
        // } else {
        //     input.report(ReportEvent.ACTIVE);
        // }
    }

    onmousemove(input: InputTracking, buf: Packet, op: number): void {
        const time: number = buf.g1();
        if (op === 1) {
            const pos: number = buf.g1();
            const x: number = (pos & 0xf) - 8;
            const y: number = ((pos >> 4) & 0xf) - 8;
            input.onmousemove(1, time, x, y);
        } else if (op === 2) {
            const x = buf.g1();
            const y = buf.g1();
            input.onmousemove(2, time, x, y);
        } else if (op === 3) {
            const pos: number = buf.g3();
            const x: number = pos & 0x3ff;
            const y: number = (pos >> 10) & 0x3ff;
            input.onmousemove(3, time, x, y);
        }

        // if (input.enabled) {
        //     input.disable(true);
        // } else {
        //     input.report(ReportEvent.ACTIVE);
        // }
    }

    onkeydown(input: InputTracking, buf: Packet): void {
        buf.pos += 1; // time
        buf.pos += 1; // key

        if (input.enabled) {
            input.disable(true);
        } else {
            input.report(ReportEvent.ACTIVE);
        }
    }

    onkeyup(input: InputTracking, buf: Packet): void {
        buf.pos += 1; // time
        buf.pos += 1; // key

        if (input.enabled) {
            input.disable(true);
        } else {
            input.report(ReportEvent.ACTIVE);
        }
    }

    onfocus(buf: Packet): void {
        buf.pos += 1; // time
    }

    onblur(buf: Packet): void {
        buf.pos += 1; // time
    }

    onmouseenter(buf: Packet): void {
        buf.pos += 1; // time
    }

    onmouseleave(input: InputTracking, buf: Packet): void {
        buf.pos += 1; // time

        if (input.enabled) {
            input.disable(true);
        } else {
            input.report(ReportEvent.ACTIVE);
        }
    }
}