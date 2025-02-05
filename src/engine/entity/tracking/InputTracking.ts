import Player from '#/engine/entity/Player.js';
import EnableTracking from '#/network/server/model/EnableTracking.js';
import FinishTracking from '#/network/server/model/FinishTracking.js';
import World from '#/engine/World.js';
import ReportEvent from '#/engine/entity/tracking/ReportEvent.js';

enum Area {
    NONE,
    VIEWPORT,
    CHAT,
    TAB,
    MINIMAP
}

export default class InputTracking {
    private static readonly TRACKING_RATE: number = 5; // 2m track interval +offset. lower this to be more aggressive.
    private static readonly TRACKING_TIME: number = 5; // 90s to track from the client
    private static readonly REPORT_TIME: number = 8; // 5s for client to report

    private static readonly MIN_X: number = 0;
    private static readonly MAX_X: number = 789;
    private static readonly MIN_Z: number = 0;
    private static readonly MAX_Z: number = 532;

    private readonly player: Player;

    // server properties
    enabled: boolean = false;
    lastTrack: number = -1;
    waitingReport: boolean = false;
    lastReport: number = -1;

    opheldClicks: number = 0;
    moveClicks: number = 0;

    // client properties
    private mouseX: number = 0;
    private mouseY: number = 0;

    constructor(player: Player) {
        this.player = player;
        this.lastTrack = World.currentTick + (InputTracking.TRACKING_RATE/* + this.offset(15)*/);
    }

    process(): void {
        // if tracking finished and waiting for client to send back the report up to 15s.
        // if (this.waitingReport && this.lastReport + InputTracking.REPORT_TIME <= World.currentTick) {
        //     this.report(ReportEvent.NO_REPORT);
        //     return;
        // }
        // if current tracking dont do anything.
        if (this.lastTrack > World.currentTick) {
            return;
        }
        // if need to start tracking.
        if (this.lastTrack <= World.currentTick + (InputTracking.TRACKING_RATE/* + this.offset(15)*/) && !this.enabled) {
            this.enable();
            return;
        }
        this.disable(false);
    }

    enable(): void {
        if (this.enabled) {
            return;
        }
        this.enabled = true;
        // start tracking for 90s. it ends early if the client gives some input.
        this.lastTrack = World.currentTick + InputTracking.TRACKING_TIME;
        this.player.write(new EnableTracking());
        console.log('Tracking enabled!');
    }

    disable(early: boolean): void {
        if (!this.enabled) {
            return;
        }
        this.enabled = false;
        this.lastTrack = World.currentTick + (InputTracking.TRACKING_RATE/* + this.offset(15)*/);
        this.player.write(new FinishTracking());
        if (!early && !this.waitingReport) {
            // wait up to 15s for the client to send us the data.
            this.waitingReport = true;
            this.lastReport = World.currentTick;
        }
        console.log('Tracking disabled!');
    }

    tracking(): boolean {
        return this.enabled || this.waitingReport;
    }

    report(event: ReportEvent): void {
        if (!this.waitingReport) {
            return;
        }
        if (event === ReportEvent.NO_REPORT) {
            // this means that:
            // 1: the player is trying to avoid afk timer.
            // 2: the player is on a very slow connection and the report packet never came in.
            // TODO: log this?
            // this.player.tryLogout = true;
            return;
        }
        // everything below means the player was active during this tracking.
        this.waitingReport = false;
        this.lastReport = -1;
    }

    onmousemove(event: number, time: number, mouseX: number, mouseY: number): void {
        if (mouseX < InputTracking.MIN_X || mouseX > InputTracking.MAX_X || mouseY < InputTracking.MIN_Z || mouseY > InputTracking.MAX_Z) {
            return;
        }

        let x = this.mouseX;
        let y = this.mouseY;

        if (event === 1) {
            x += mouseX;
            y += mouseY;
        } else if (event === 2) {
            x = mouseX - 128 + this.mouseX;
            y = mouseY - 128 + this.mouseY;
        } else if (event === 3) {
            x = mouseX;
            y = mouseY;
        }

        this.mouseX = x;
        this.mouseY = y;

        console.log(`Mouse Move: (${this.mouseX}, ${this.mouseY}), Time: ${time}`);
    }

    onmousedown(time: number, mouseX: number, mouseY: number): void {
        if (mouseX < InputTracking.MIN_X || mouseX > InputTracking.MAX_X || mouseY < InputTracking.MIN_Z || mouseY > InputTracking.MAX_Z) {
            return;
        }

        this.mouseX = mouseX;
        this.mouseY = mouseY;


        console.log(`Mouse Down: (${mouseX}, ${mouseY}), Time: ${time}`);
    }

    trackOpHeld(): void {
        this.opheldClicks++;
    }

    trackMoveClick(): void {
        this.moveClicks++;
    }

    reset(): void {
        this.opheldClicks = 0;
    }

    private onViewport(): ReportEvent {

    }

    private findMouseArea(): Area {
        if (this.insideViewportArea()) {
            return Area.VIEWPORT;
        } else if (this.insideChatPopupArea()) {
            return Area.CHAT;
        } else if (this.insideTabArea()) {
            return Area.TAB;
        } else if (this.insideMinimapArea()) {
            return Area.MINIMAP;
        }
        return Area.NONE;
    }

    private insideMinimapArea(): boolean {
        // 170 x 160
        const viewportAreaX1: number = 570;
        const viewportAreaY1: number = 11;
        const viewportAreaX2: number = viewportAreaX1 + 170;
        const viewportAreaY2: number = viewportAreaY1 + 160;
        return this.mouseX >= viewportAreaX1 && this.mouseX <= viewportAreaX2 && this.mouseY >= viewportAreaY1 && this.mouseY <= viewportAreaY2;
    };

    private insideViewportArea(): boolean {
        // 512 x 334
        const viewportAreaX1: number = 8;
        const viewportAreaY1: number = 11;
        const viewportAreaX2: number = viewportAreaX1 + 512;
        const viewportAreaY2: number = viewportAreaY1 + 334;
        return this.mouseX >= viewportAreaX1 && this.mouseX <= viewportAreaX2 && this.mouseY >= viewportAreaY1 && this.mouseY <= viewportAreaY2;
    };

    private insideTabArea(): boolean {
        // 190 x 261
        const tabAreaX1: number = 562;
        const tabAreaY1: number = 231;
        const tabAreaX2: number = tabAreaX1 + 190;
        const tabAreaY2: number = tabAreaY1 + 261;
        return this.mouseX >= tabAreaX1 && this.mouseX <= tabAreaX2 && this.mouseY >= tabAreaY1 && this.mouseY <= tabAreaY2;
    };

    private insideChatPopupArea(): boolean {
        // 495 x 99
        const chatInputAreaX1: number = 11;
        const chatInputAreaY1: number = 383;
        const chatInputAreaX2: number = chatInputAreaX1 + 495;
        const chatInputAreaY2: number = chatInputAreaY1 + 99;
        return this.mouseX >= chatInputAreaX1 && this.mouseX <= chatInputAreaX2 && this.mouseY >= chatInputAreaY1 && this.mouseY <= chatInputAreaY2;
    };

    private offset(n: number): number {
        return Math.floor(Math.random() * (n - (-n) + 1)) + (-n);
    }
}