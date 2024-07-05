import MessageHandler from '#lostcity/network/incoming/handler/MessageHandler.js';
import Player from '#lostcity/entity/Player.js';
import ChatSetMode from '#lostcity/network/incoming/model/ChatSetMode.js';
import PublicChat from '#lostcity/entity/chat/PublicChat.js';
import PrivateChat from '#lostcity/entity/chat/PrivateChat.js';
import TradeDuelChat from '#lostcity/entity/chat/TradeDuelChat.js';
import ChatFilterSettings from '#lostcity/network/outgoing/model/ChatFilterSettings.js';

export default class ChatSetModeHandler extends MessageHandler<ChatSetMode> {
    handle(message: ChatSetMode, player: Player): boolean {
        const {publicChat, privateChat, tradeDuelChat} = message;

        if (publicChat < PublicChat.ON || publicChat > PublicChat.HIDE) {
            return false;
        }

        if (privateChat < PrivateChat.ON || privateChat > PrivateChat.OFF) {
            return false;
        }

        if (tradeDuelChat < TradeDuelChat.ON || tradeDuelChat > TradeDuelChat.OFF) {
            return false;
        }

        player.publicChat = publicChat;
        player.privateChat = privateChat;
        player.tradeDuelChat = tradeDuelChat;
        player.write(new ChatFilterSettings(publicChat, privateChat, tradeDuelChat));
        return true;
    }
}
