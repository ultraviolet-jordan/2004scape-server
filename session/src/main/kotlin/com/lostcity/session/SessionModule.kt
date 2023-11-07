package com.lostcity.session

import com.lostcity.session.decode.LoginDecoder
import com.lostcity.session.encode.*
import dev.misfitlabs.kotlinguice4.KotlinModule
import dev.misfitlabs.kotlinguice4.multibindings.KotlinMultibinder

object SessionModule : KotlinModule() {
    override fun configure() {
        KotlinMultibinder.newSetBinder<CodecEncoder<CodecEncoderType>>(kotlinBinder).apply {
            addBinding().to<BadSessionEncoder>()
            addBinding().to<CouldNotCompleteEncoder>()
            addBinding().to<InvalidUsernameOrPasswordEncoder>()
            addBinding().to<LoginAttemptsExceededEncoder>()
            addBinding().to<LoginEncoder>()
            addBinding().to<LoginLimitExceededEncoder>()
            addBinding().to<LoginServerOfflineEncoder>()
            addBinding().to<LoginServerRejectedSessionEncoder>()
            addBinding().to<RuneScapeHasBeenUpdatedEncoder>()
            addBinding().to<ServerSeedEncoder>()
            addBinding().to<SuccessfulEncoder>()
            addBinding().to<SuccessfulModeratorEncoder>()
            addBinding().to<SuccessfulReconnectEncoder>()
            addBinding().to<ThisServerIsBeingUpdatedEncoder>()
            addBinding().to<ThisWorldIsFullEncoder>()
            addBinding().to<YouAreStandingInAMembersOnlyAreaEncoder>()
            addBinding().to<YouNeedAMembersAccountToLoginThisWorldEncoder>()
            addBinding().to<YourAccountHasBeenDisabledEncoder>()
            addBinding().to<YourAccountIsAlreadyLoggedInEncoder>()
        }

        KotlinMultibinder.newSetBinder<CodecDecoder<CodecDecoderType>>(kotlinBinder).apply {
            addBinding().to<LoginDecoder>()
        }
    }
}