package com.lostcity.socket.packet

import java.nio.ByteBuffer

abstract class PacketReader<out T : Packet>(
    val id: Int,
    val length: Int
) {
    abstract suspend fun readPacket(buffer: ByteBuffer, length: Int): T
}
