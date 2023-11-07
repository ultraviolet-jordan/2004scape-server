package com.lostcity.socket.packet

import java.nio.ByteBuffer

abstract class PacketBuilder<out T : Packet>(
    val id: Int,
    val length: Int
) {
    abstract fun buildPacket(packet: @UnsafeVariance T, buffer: ByteBuffer)
}
