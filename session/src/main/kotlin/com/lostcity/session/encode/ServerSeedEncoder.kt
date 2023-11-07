package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.ServerSeedResponse
import jagex2.io.p8
import java.nio.ByteBuffer

@Singleton
class ServerSeedEncoder : CodecEncoder<ServerSeedResponse> {
    override suspend fun encode(session: Session, response: ServerSeedResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(8)
        buffer.p8(response.seed)
        return buffer.flip()
    }
}