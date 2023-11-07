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
import io.ktor.network.sockets.*
import jagex2.io.Isaac
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.nio.ByteBuffer
import kotlin.reflect.KClass

class TCPClientSocket(
    builders: Map<KClass<*>, PacketBuilder<Packet>>,
    readers: Array<PacketReader<Packet>?>,
    private val socket: Socket
) : ClientSocket(builders, readers) {
    private val readChannel = socket.openReadChannel()
    private val writeChannel = socket.openWriteChannel()

    override suspend fun accept(session: Session) {
        val buffer = session.codec(ServerSeedEncoder::class, ServerSeedResponse(session.seed))
        if (buffer == null) {
            terminate("Could not encode server seed.")
            return
        }
        writeChannel.writeFully(buffer)
        writeChannel.flush()
        login(session)
    }

    override suspend fun login(session: Session) {
        val opcode = readChannel.readByte().toInt() and 0xff
        if (opcode != 16 && opcode != 18) {
            terminate("Invalid login buffer.")
            return
        }
        val length = readChannel.readByte().toInt() and 0xff
        val buffer = ByteBuffer.allocate(length)
        val bytes = readChannel.readFully(buffer).also {
            buffer.flip()
        }
        if (bytes != length) {
            terminate("Invalid login buffer.")
            return
        }

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

        writeChannel.writeFully(loginResponse)
        writeChannel.flush()

        game()
    }

    override suspend fun game() {
        while (true) {
            val isaac = this.decryptor
            if (isaac == null) {
                terminate("Decryptor was null.")
                break
            }
            val id = (readChannel.readByte() - isaac.nextInt) and 0xff
            if (id > ClientProtLengths.size) {
                readChannel.discard(readChannel.availableForRead.toLong())
                continue
            }
            val serverLength = ClientProtLengths[id]
            if (serverLength == ClientProtLengths.Default) {
                readChannel.discard(readChannel.availableForRead.toLong())
                continue
            }

            val clientLength = when {
                serverLength != -1 && serverLength != -2 -> serverLength
                serverLength == -1 -> (readChannel.readByte().toInt() and 0xff)
                else -> (readChannel.readShort().toInt() and 0xffff)
            }

            val reader = readers[id]
            if (reader == null) {
                readChannel.discard(clientLength.toLong())
                continue
            }

            if (reader.length != -1 && reader.length != clientLength) {
                readChannel.discard(clientLength.toLong())
                continue
            }

            val payload = ByteBuffer.allocate(clientLength)
            val readBytes = readChannel.readFully(payload)
            if (readBytes != clientLength) {
                continue
            }

            readPacket(reader.readPacket(payload.flip(), clientLength))
        }
    }

    override fun flush(buffer: ByteBuffer) {
        TODO("Not yet implemented")
    }

    override suspend fun terminate(message: String) {
        withContext(Dispatchers.IO) {
            writeChannel.close(Throwable(message))
            socket.close()
        }
    }
}