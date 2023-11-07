package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.BadSessionResponse
import com.lostcity.session.type.SuccessfulModeratorResponse
import com.lostcity.session.type.SuccessfulResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class SuccessfulModeratorEncoder : CodecEncoder<SuccessfulModeratorResponse> {
    override suspend fun encode(session: Session, response: SuccessfulModeratorResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(18)
        return buffer.flip()
    }
}