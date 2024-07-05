import IncomingMessage from '#lostcity/network/incoming/IncomingMessage.js';
import ClientProtCategory from '#lostcity/network/incoming/prot/ClientProtCategory.js';

export default class ChatSetMode extends IncomingMessage {
    category = ClientProtCategory.USER_EVENT;

    constructor(
        readonly publicChat: number,
        readonly privateChat: number,
        readonly tradeDuelChat: number
    ) {
        super();
    }
}
