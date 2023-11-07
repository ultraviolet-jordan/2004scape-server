package com.lostcity.game

object ClientProtLengths: MutableList<Int> by ArrayList(255) {
    val Default = -3

    init {
        this.fill(Default)

        this[ClientProt.MAP_REQUEST_AREAS] = -1;
        this[ClientProt.IDLE_TIMER] = 0;
        this[ClientProt.NO_TIMEOUT] = 0;

        this[ClientProt.EVENT_TRACKING] = -2;
        this[ClientProt.EVENT_CAMERA_POSITION] = 6;

        this[ClientProt.ANTICHEAT_OPLOC4] = 4;
        this[ClientProt.ANTICHEAT_OPNPC3] = 4;
        this[ClientProt.ANTICHEAT_OPHELD1] = 3;
        this[ClientProt.ANTICHEAT_OPNPC5] = 2;
        this[ClientProt.ANTICHEAT_OPHELD4] = 0;
        this[ClientProt.ANTICHEAT_OPLOC5] = 4;
        this[ClientProt.ANTICHEAT_INV_BUTTON5] = 4;
        this[ClientProt.ANTICHEAT_OPPLAYER2] = 2;
        this[ClientProt.ANTICHEAT_INV_BUTTON4] = 1;

        this[ClientProt.ANTICHEAT_SIDEBAR_INPUT] = 1;
        this[ClientProt.ANTICHEAT_DRAW] = -1;
        this[ClientProt.ANTICHEAT_UPDATE] = 3;
        this[ClientProt.ANTICHEAT_UPDATE2] = 4;
        this[ClientProt.ANTICHEAT_UPDATE_LOCS] = 0;
        this[ClientProt.ANTICHEAT_UPDATE_PLAYERS] = -1;

        this[ClientProt.OPOBJ1] = 6;
        this[ClientProt.OPOBJ2] = 6;
        this[ClientProt.OPOBJ3] = 6;
        this[ClientProt.OPOBJ4] = 6;
        this[ClientProt.OPOBJ5] = 6;
        this[ClientProt.OPOBJT] = 8;
        this[ClientProt.OPOBJU] = 12;

        this[ClientProt.OPNPC1] = 2;
        this[ClientProt.OPNPC2] = 2;
        this[ClientProt.OPNPC3] = 2;
        this[ClientProt.OPNPC4] = 2;
        this[ClientProt.OPNPC5] = 2;
        this[ClientProt.OPNPCT] = 4;
        this[ClientProt.OPNPCU] = 8;

        this[ClientProt.OPLOC1] = 6;
        this[ClientProt.OPLOC2] = 6;
        this[ClientProt.OPLOC3] = 6;
        this[ClientProt.OPLOC4] = 6;
        this[ClientProt.OPLOC5] = 6;
        this[ClientProt.OPLOCT] = 8;
        this[ClientProt.OPLOCU] = 12;

        this[ClientProt.OPPLAYER1] = 2;
        this[ClientProt.OPPLAYER2] = 2;
        this[ClientProt.OPPLAYER3] = 2;
        this[ClientProt.OPPLAYER4] = 2;
        this[ClientProt.OPPLAYERT] = 4;
        this[ClientProt.OPPLAYERU] = 8;

        this[ClientProt.OPHELD1] = 6;
        this[ClientProt.OPHELD2] = 6;
        this[ClientProt.OPHELD3] = 6;
        this[ClientProt.OPHELD4] = 6;
        this[ClientProt.OPHELD5] = 6;
        this[ClientProt.OPHELDT] = 8;
        this[ClientProt.OPHELDU] = 12;

        this[ClientProt.INV_BUTTON1] = 6;
        this[ClientProt.INV_BUTTON2] = 6;
        this[ClientProt.INV_BUTTON3] = 6;
        this[ClientProt.INV_BUTTON4] = 6;
        this[ClientProt.INV_BUTTON5] = 6;
        this[ClientProt.IF_BUTTON] = 2;

        this[ClientProt.RESUME_PAUSEBUTTON] = 2;
        this[ClientProt.CLOSE_MODAL] = 0;
        this[ClientProt.RESUME_P_COUNTDIALOG] = 4;
        this[ClientProt.IF_FLASHING_TAB] = 1;

        this[ClientProt.MOVE_OPCLICK] = -1;
        this[ClientProt.BUG_REPORT] = 10;
        this[ClientProt.MOVE_MINIMAPCLICK] = -1;
        this[ClientProt.INV_BUTTOND] = 6;
        this[ClientProt.IGNORELIST_DEL] = 8;
        this[ClientProt.IGNORELIST_ADD] = 8;
        this[ClientProt.IF_DESIGN] = 13;
        this[ClientProt.CHAT_SETMODE] = 3;
        this[ClientProt.MESSAGE_PRIVATE] = -1;
        this[ClientProt.FRIENDLIST_DEL] = 8;
        this[ClientProt.FRIENDLIST_ADD] = 8;
        this[ClientProt.CLIENT_CHEAT] = -1;
        this[ClientProt.MESSAGE_PUBLIC] = -1;
        this[ClientProt.MOVE_GAMECLICK] = -1;
    }

    private operator fun set(prot: ClientProt, value: Int) {
        set(prot.id, value)
    }
}