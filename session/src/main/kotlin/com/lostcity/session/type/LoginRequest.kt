package com.lostcity.session.type

import com.lostcity.session.CodecDecoderType

data class LoginRequest(
    val info: Int? = null,
    val uid: Int? = null,
    val username: String? = null,
    val password: String? = null,
    val seed: IntArray? = null,
): CodecDecoderType {
    companion object {
        val InvalidUsernameOrPassword = LoginRequest()
        val YourAccountHasBeenDisabled = LoginRequest()
        val YourAccountIsAlreadyLoggedIn = LoginRequest()
        val RuneScapeHasBeenUpdated = LoginRequest()
        val ThisWorldIsFull = LoginRequest()
        val LoginServerOffline = LoginRequest()
        val LoginLimitExceeded = LoginRequest()
        val BadSession = LoginRequest()
        val LoginServerRejected = LoginRequest()
        val YouNeedAMembersAccountToLoginThisWorld = LoginRequest()
        val CouldNotComplete = LoginRequest()
        val ThisServerIsBeingUpdated = LoginRequest()
        val LoginAttemptsExceeded = LoginRequest()
        val YouAreStandingInAMembersOnlyArea = LoginRequest()
    }
}
