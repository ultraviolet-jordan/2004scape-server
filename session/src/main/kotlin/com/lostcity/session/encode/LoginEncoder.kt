package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.*
import java.nio.ByteBuffer

@Singleton
class LoginEncoder : CodecEncoder<LoginResponse> {
    override suspend fun encode(session: Session, response: LoginResponse): ByteBuffer? {
        return when (response.request) {
            LoginRequest.InvalidUsernameOrPassword -> session.codec(InvalidUsernameOrPasswordEncoder::class, InvalidUsernameOrPasswordResponse)
            LoginRequest.YourAccountHasBeenDisabled -> session.codec(YourAccountHasBeenDisabledEncoder::class, YourAccountHasBeenDisabledResponse)
            LoginRequest.YourAccountIsAlreadyLoggedIn -> session.codec(YourAccountIsAlreadyLoggedInEncoder::class, YourAccountIsAlreadyLoggedInResponse)
            LoginRequest.RuneScapeHasBeenUpdated -> session.codec(RuneScapeHasBeenUpdatedEncoder::class, RuneScapeHasBeenUpdatedResponse)
            LoginRequest.ThisWorldIsFull -> session.codec(ThisWorldIsFullEncoder::class, ThisWorldIsFullResponse)
            LoginRequest.LoginServerOffline -> session.codec(LoginServerOfflineEncoder::class, LoginServerOfflineResponse)
            LoginRequest.LoginLimitExceeded -> session.codec(LoginLimitExceededEncoder::class, LoginLimitExceededResponse)
            LoginRequest.BadSession -> session.codec(BadSessionEncoder::class, BadSessionResponse)
            LoginRequest.LoginServerRejected -> session.codec(LoginServerRejectedSessionEncoder::class, LoginServerRejectedSessionResponse)
            LoginRequest.YouNeedAMembersAccountToLoginThisWorld -> session.codec(YouNeedAMembersAccountToLoginThisWorldEncoder::class, YouNeedAMembersAccountToLoginThisWorldResponse)
            LoginRequest.CouldNotComplete -> session.codec(CouldNotCompleteEncoder::class, CouldNotCompleteResponse)
            LoginRequest.ThisServerIsBeingUpdated -> session.codec(ThisServerIsBeingUpdatedEncoder::class, ThisServerIsBeingUpdatedResponse)
            LoginRequest.LoginAttemptsExceeded -> session.codec(LoginAttemptsExceededEncoder::class, LoginAttemptsExceededResponse)
            LoginRequest.YouAreStandingInAMembersOnlyArea -> session.codec(YouAreStandingInAMembersOnlyAreaEncoder::class, YouAreStandingInAMembersOnlyAreaResponse)
            else -> {
                // TODO additional checks like if already logged in
                if (response.reconnecting) {
                    session.codec(SuccessfulReconnectEncoder::class, SuccessfulReconnectResponse)
                } else {
                    session.codec(SuccessfulEncoder::class, SuccessfulResponse)
                }
            }
        }
    }
}