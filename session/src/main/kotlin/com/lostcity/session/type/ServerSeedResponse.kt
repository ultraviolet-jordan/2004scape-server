package com.lostcity.session.type

import com.lostcity.session.CodecEncoderType

data class ServerSeedResponse(
    val seed: Long
) : CodecEncoderType
