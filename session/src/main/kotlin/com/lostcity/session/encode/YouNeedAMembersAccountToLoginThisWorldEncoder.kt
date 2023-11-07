package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.BadSessionResponse
import com.lostcity.session.type.YouAreStandingInAMembersOnlyAreaResponse
import com.lostcity.session.type.YouNeedAMembersAccountToLoginThisWorldResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class YouNeedAMembersAccountToLoginThisWorldEncoder : CodecEncoder<YouNeedAMembersAccountToLoginThisWorldResponse> {
    override suspend fun encode(session: Session, response: YouNeedAMembersAccountToLoginThisWorldResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(12)
        return buffer.flip()
    }
}