package com.lostcity.session

import com.google.inject.Inject
import com.google.inject.Singleton

@Singleton
data class SessionCodecs @Inject constructor(
    val encoders: Set<CodecEncoder<CodecEncoderType>>,
    val decoders: Set<CodecDecoder<CodecDecoderType>>
)
