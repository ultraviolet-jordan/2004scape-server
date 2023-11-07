package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.BadSessionResponse
import com.lostcity.session.type.CouldNotCompleteResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class CouldNotCompleteEncoder : CodecEncoder<CouldNotCompleteResponse> {
    override suspend fun encode(session: Session, response: CouldNotCompleteResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(13)
        return buffer.flip()
    }
}