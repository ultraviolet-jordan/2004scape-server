package com.lostcity.session

import java.nio.ByteBuffer

interface CodecDecoder<out T : CodecDecoderType> {
    suspend fun decode(session: Session, buffer: ByteBuffer): @UnsafeVariance T
}
