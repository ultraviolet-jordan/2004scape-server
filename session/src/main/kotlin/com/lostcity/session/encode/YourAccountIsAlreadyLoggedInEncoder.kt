package com.lostcity.session.encode

import com.google.inject.Singleton
import com.lostcity.session.CodecEncoder
import com.lostcity.session.Session
import com.lostcity.session.type.BadSessionResponse
import com.lostcity.session.type.YouAreStandingInAMembersOnlyAreaResponse
import com.lostcity.session.type.YouNeedAMembersAccountToLoginThisWorldResponse
import com.lostcity.session.type.YourAccountHasBeenDisabledResponse
import jagex2.io.p1
import java.nio.ByteBuffer

@Singleton
class YourAccountIsAlreadyLoggedInEncoder : CodecEncoder<YourAccountHasBeenDisabledResponse> {
    override suspend fun encode(session: Session, response: YourAccountHasBeenDisabledResponse): ByteBuffer {
        val buffer = ByteBuffer.allocate(1)
        buffer.p1(5)
        return buffer.flip()
    }
}