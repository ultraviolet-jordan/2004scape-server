package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.InvalidUsernameOrPasswordResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class InvalidUsernameOrPasswordEncoder : CodecEncoder<InvalidUsernameOrPasswordResponse> {
    override suspend fun encode(session: Session, response: InvalidUsernameOrPasswordResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(3)
        return buffer.flip()
    }
}