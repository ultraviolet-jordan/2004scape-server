package com.lostcity.session

import java.nio.ByteBuffer

interface CodecEncoder<out T : CodecEncoderType> {
    suspend fun encode(session: Session, response: @UnsafeVariance T): ByteBuffer?
}
