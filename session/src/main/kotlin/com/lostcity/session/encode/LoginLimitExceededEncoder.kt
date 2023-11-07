package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.InvalidUsernameOrPasswordResponse
import com.lostcity.session.type.LoginLimitExceededResponse
import com.lostcity.session.type.LoginServerOfflineResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class LoginLimitExceededEncoder : CodecEncoder<LoginLimitExceededResponse> {
    override suspend fun encode(session: Session, response: LoginLimitExceededResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(9)
        return buffer.flip()
    }
}