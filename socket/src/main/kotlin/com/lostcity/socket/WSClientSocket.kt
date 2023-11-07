package com.lostcity.socket

import com.lostcity.game.ClientProtLengths
import com.lostcity.socket.packet.Packet
import com.lostcity.socket.packet.PacketBuilder
import com.lostcity.socket.packet.PacketReader
import com.lostcity.session.Session
import com.lostcity.session.decode.LoginDecoder
import com.lostcity.session.encode.LoginEncoder
import com.lostcity.session.encode.ServerSeedEncoder
import com.lostcity.session.type.LoginResponse
import com.lostcity.session.type.ServerSeedResponse
import io.ktor.websocket.*
import jagex2.io.Isaac
import jagex2.io.g1
import jagex2.io.g1b
import jagex2.io.g2
import java.nio.ByteBuffer
import kotlin.reflect.KClass

class WSClientSocket(
    builders: Map<KClass<*>, PacketBuilder<Packet>>,
    readers: Array<PacketReader<Packet>?>,
    private val socket: WebSocketSession
) : ClientSocket(builders, readers) {
    override suspend fun accept(session: Session) {
        val buffer = session.codec(ServerSeedEncoder::class, ServerSeedResponse(session.seed))
        if (buffer == null) {
            terminate("Could not encode server seed.")
            return
        }
        socket.send(buffer.array())
        socket.flush()
        login(session)
    }

    override suspend fun login(session: Session) {
        val incoming = socket.incoming.receive()
        if (incoming !is Frame.Binary) {
            terminate("Frame was not binary.")
            return
        }
        val opcode = incoming.buffer.g1
        val length = incoming.buffer.g1
        if (opcode != 16 && opcode != 18) {
            terminate("Invalid login buffer from tcp.")
            return
        }
        val buffer = ByteBuffer.allocate(length).put(incoming.buffer).flip()
        val loginRequest = session.codec(LoginDecoder::class, buffer)
        if (loginRequest == null) {
            terminate("Invalid login request.")
            return
        }

        val loginResponse = session.codec(LoginEncoder::class, LoginResponse(opcode == 18, loginRequest))
        if (loginResponse == null) {
            terminate("Invalid login response.")
            return
        }

        val seed = loginRequest.seed!!
        this.encryptor = Isaac.create(seed)
        this.decryptor = Isaac.create(IntArray(seed.size) { seed[it] + 50 })

        socket.send(loginResponse.array())
        socket.flush()

        game()
    }

    override suspend fun game() {
        while (true) {
            val isaac = this.decryptor
            if (isaac == null) {
                terminate("Decryptor was null.")
                break
            }
            val incoming = socket.incoming.receive()
            if (incoming !is Frame.Binary) {
                terminate("Frame was not binary.")
                break
            }
            val buffer = incoming.buffer
            val id = (buffer.g1b - isaac.nextInt) and 0xff
            if (id > ClientProtLengths.size) {
                continue
            }

            println(id)

            val serverLength = ClientProtLengths[id]
            if (serverLength == ClientProtLengths.Default) {
                continue
            }

            val clientLength = when {
                serverLength != -1 && serverLength != -2 -> serverLength
                serverLength == -1 -> buffer.g1
                else -> buffer.g2
            }

            val reader = readers[id] ?: continue

            if (reader.length != -1 && reader.length != clientLength) {
                continue
            }

            val payload = ByteBuffer.allocate(clientLength).put(incoming.buffer).flip()
            readPacket(reader.readPacket(payload, clientLength))
        }
    }

    override fun flush(buffer: ByteBuffer) {
        TODO("Not yet implemented")
    }

    override suspend fun terminate(message: String) {
        socket.close(CloseReason(CloseReason.Codes.PROTOCOL_ERROR, message))
    }
}