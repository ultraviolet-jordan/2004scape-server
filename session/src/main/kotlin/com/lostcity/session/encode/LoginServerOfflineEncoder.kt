package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.InvalidUsernameOrPasswordResponse
import com.lostcity.session.type.LoginServerOfflineResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class LoginServerOfflineEncoder : CodecEncoder<LoginServerOfflineResponse> {
    override suspend fun encode(session: Session, response: LoginServerOfflineResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(8)
        return buffer.flip()
    }
}