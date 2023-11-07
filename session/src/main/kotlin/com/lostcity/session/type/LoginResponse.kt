package com.lostcity.session.type

import com.lostcity.session.CodecEncoderType

data class LoginResponse(
    val reconnecting: Boolean,
    val request: LoginRequest
) : CodecEncoderType