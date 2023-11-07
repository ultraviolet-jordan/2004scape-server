package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.BadSessionResponse
import com.lostcity.session.type.SuccessfulResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class SuccessfulEncoder : CodecEncoder<SuccessfulResponse> {
    override suspend fun encode(session: Session, response: SuccessfulResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(2)
        return buffer.flip()
    }
}