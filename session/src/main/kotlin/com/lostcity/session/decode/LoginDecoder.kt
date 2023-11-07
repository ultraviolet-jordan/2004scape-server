package com.lostcity.session.decode

import com.google.inject.Singleton
import com.lostcity.session.CodecDecoder
import com.lostcity.session.Session
import com.lostcity.session.type.LoginRequest
import jagex2.io.g1
import jagex2.io.g4
import jagex2.io.gstr
import jagex2.io.rsadec
import java.nio.ByteBuffer

@Singleton
class LoginDecoder : CodecDecoder<LoginRequest> {
    override suspend fun decode(session: Session, buffer: ByteBuffer): LoginRequest {
        val version = buffer.g1
        if (version != 225) {
            return LoginRequest.RuneScapeHasBeenUpdated
        }
        val info = buffer.g1
        val crcs = IntArray(9) { buffer.g4 } // TODO
        buffer.rsadec(session.pem)
        val magic = buffer.g1
        if (magic != 10) {
            return LoginRequest.LoginServerRejected
        }
        val seed = IntArray(4) { buffer.g4 }
        val uid = buffer.g4
        val username = buffer.gstr
        val password = buffer.gstr
        println("Username: $username, Password: $password")
        return LoginRequest(info, uid, username, password, seed)
    }
}