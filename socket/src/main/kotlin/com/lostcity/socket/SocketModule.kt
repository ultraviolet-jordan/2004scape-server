package com.lostcity.socket

import com.lostcity.socket.packet.Packet
import com.lostcity.socket.packet.PacketBuilder
import com.lostcity.socket.packet.PacketReader
import dev.misfitlabs.kotlinguice4.KotlinModule
import dev.misfitlabs.kotlinguice4.multibindings.KotlinMapBinder
import dev.misfitlabs.kotlinguice4.multibindings.KotlinMultibinder
import kotlin.reflect.KClass

object SocketModule : KotlinModule() {
    override fun configure() {
        KotlinMapBinder.newMapBinder<KClass<*>, PacketBuilder<Packet>>(kotlinBinder).apply {

        }

        KotlinMultibinder.newSetBinder<PacketReader<Packet>>(kotlinBinder).apply {

        }
    }
}