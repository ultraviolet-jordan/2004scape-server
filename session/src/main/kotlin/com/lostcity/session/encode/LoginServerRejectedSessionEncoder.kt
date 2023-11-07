package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.LoginServerRejectedSessionResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class LoginServerRejectedSessionEncoder : CodecEncoder<LoginServerRejectedSessionResponse> {
    override suspend fun encode(session: Session, response: LoginServerRejectedSessionResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(11)
        return buffer.flip()
    }
}