package com.lostcity.socket

import com.lostcity.session.Session
import com.lostcity.socket.packet.*
import jagex2.io.*
import java.nio.ByteBuffer
import java.util.concurrent.ArrayBlockingQueue
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.ConcurrentLinkedDeque
import java.util.concurrent.CopyOnWriteArrayList
import kotlin.math.absoluteValue
import kotlin.reflect.KClass

abstract class ClientSocket(
    private val builders: Map<KClass<*>, PacketBuilder<Packet>>,
    protected val readers: Array<PacketReader<Packet>?>,
    private val readQueue: ConcurrentLinkedDeque<Packet> = ConcurrentLinkedDeque(),
    private val writeQueue: ByteBuffer = ByteBuffer.allocateDirect(5000)
) {
    protected var decryptor: Isaac? = null
    protected var encryptor: Isaac? = null

    abstract suspend fun accept(session: Session)
    abstract suspend fun login(session: Session)
    abstract suspend fun game()
    abstract fun flush(buffer: ByteBuffer)
    abstract suspend fun terminate(message: String)

    fun readPacket(packet: Packet) {
        readQueue.add(packet)
    }

    fun flushWriteQueue() {
        flush(writeQueue)
        writeQueue.clear()
    }

    fun writePacket(packet: Packet) {
        write(writeQueue, packet)
    }

    fun writePacketDirect(packet: Packet, length: Int) {
        val buffer = ByteBuffer.allocate(length + 3)
        write(buffer, packet)
        flush(buffer)
    }

    fun write(buffer: ByteBuffer, packet: Packet) {
        val isaac = encryptor ?: return
        val builder = builders[packet::class] ?: return
        val length = builder.length
        buffer.p1(builder.id + isaac.nextInt)
        if (length != -1 && length != -2) {
            builder.buildPacket(packet, buffer)
            return
        }
        val abs = length.absoluteValue
        val pos = buffer.position
        buffer.pad(abs)
        builder.buildPacket(packet, buffer)
        if (length == -1) {
            buffer.psize1(buffer.position - pos - abs)
        } else {
            buffer.psize2(buffer.position - pos - abs)
        }
    }
}