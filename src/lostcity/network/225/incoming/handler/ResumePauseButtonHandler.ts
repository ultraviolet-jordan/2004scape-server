import MessageHandler from '#lostcity/network/incoming/handler/MessageHandler.js';
import Player from '#lostcity/entity/Player.js';
import ResumePauseButton from '#lostcity/network/incoming/model/ResumePauseButton.js';
import { ScriptExecutionState } from '../../../../../../runescript-runtime/dist/runescript-runtime.js';

export default class ResumePauseButtonHandler extends MessageHandler<ResumePauseButton> {
    handle(_message: ResumePauseButton, player: Player): boolean {
        if (!player.activeScript || player.activeScript.execution !== ScriptExecutionState.PauseButton) {
            return false;
        }

        player.executeScript(player.activeScript, true, true);
        return true;
    }
}
