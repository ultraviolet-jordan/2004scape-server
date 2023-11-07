package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.ThisServerIsBeingUpdatedResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class ThisServerIsBeingUpdatedEncoder : CodecEncoder<ThisServerIsBeingUpdatedResponse> {
    override suspend fun encode(session: Session, response: ThisServerIsBeingUpdatedResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(14)
        return buffer.flip()
    }
}