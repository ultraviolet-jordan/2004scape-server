package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.BadSessionResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class BadSessionEncoder : CodecEncoder<BadSessionResponse> {
    override suspend fun encode(session: Session, response: BadSessionResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(10)
        return buffer.flip()
    }
}