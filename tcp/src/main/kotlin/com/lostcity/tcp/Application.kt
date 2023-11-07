package com.lostcity.tcp

import com.google.inject.Guice
import com.google.inject.Inject
import com.google.inject.Singleton
import com.lostcity.game.GameServer
import com.lostcity.socket.packet.Packet
import com.lostcity.socket.packet.PacketBuilder
import com.lostcity.socket.packet.PacketReader
import com.lostcity.session.SessionCodecs
import com.lostcity.session.Session
import com.lostcity.session.SessionModule
import com.lostcity.socket.SocketModule
import com.lostcity.socket.TCPClientSocket
import dev.misfitlabs.kotlinguice4.getInstance
import io.ktor.network.sockets.*
import io.ktor.server.engine.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import java.security.interfaces.RSAPrivateCrtKey
import kotlin.reflect.KClass
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    try {
        val injector = Guice.createInjector(
            TCPModule(args),
            SessionModule,
            SocketModule
        )

        val application = injector.getInstance<Application>()
        val serverSocket = injector.getInstance<ServerSocket>()

        application.gameServer.start {
            application.engine.environment.log.info("TCP: Online!")
            runBlocking {
                while (true) {
                    val socket = serverSocket.accept()
                    val session = Session(
                        seed = ((Math.random() * 99999999.0).toLong() shl 32) + (Math.random() * 99999999.0).toLong(),
                        pem = application.pem,
                        codecs = application.codecs
                    )
                    val client = TCPClientSocket(
                        socket = socket,
                        builders = application.builders,
                        readers = Array(255) { application.readers.firstOrNull { reader -> reader.id == it } },
                    )
                    launch(Dispatchers.IO) {
                        client.accept(session)
                    }
                }
            }
        }
    } catch (exception: Exception) {
        exception.printStackTrace()
        exitProcess(0);
    }
}

@Singleton
class Application @Inject constructor(
    val engine: ApplicationEngine,
    val gameServer: GameServer,
    val codecs: SessionCodecs,
    val pem: RSAPrivateCrtKey,
    val builders: Map<KClass<*>, PacketBuilder<Packet>>,
    val readers: Set<PacketReader<Packet>>,
) {
    init {
        Runtime.getRuntime().addShutdownHook(ShutdownHook(engine))
    }
}
