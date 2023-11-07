package com.lostcity.ws

import com.google.inject.Guice
import com.google.inject.Inject
import com.google.inject.Singleton
import com.lostcity.game.GameServer
import com.lostcity.socket.packet.Packet
import com.lostcity.socket.packet.PacketBuilder
import com.lostcity.socket.packet.PacketReader
import com.lostcity.session.Session
import com.lostcity.session.SessionCodecs
import com.lostcity.session.SessionModule
import com.lostcity.socket.SocketModule
import com.lostcity.socket.WSClientSocket
import com.lostcity.ws.plugin.installWebSocketsPlugin
import dev.misfitlabs.kotlinguice4.getInstance
import io.ktor.server.engine.*
import io.ktor.server.routing.*
import io.ktor.server.websocket.*
import java.security.interfaces.RSAPrivateCrtKey
import kotlin.reflect.KClass
import kotlin.system.exitProcess

fun main(args: Array<String>) {
    try {
        val injector = Guice.createInjector(
            WSModule(args),
            SessionModule,
            SocketModule
        )

        val application = injector.getInstance<Application>()
        val engine = application.engine

        application.gameServer.start {
            engine.environment.log.info("WS: Online!")
            with(engine.application) {
                installWebSocketsPlugin()

                routing {
                    webSocketRaw("binary", false) {
                        val session = Session(
                            seed = ((Math.random() * 99999999.0).toLong() shl 32) + (Math.random() * 99999999.0).toLong(),
                            pem = application.pem,
                            codecs = application.codecs
                        )
                        val client = WSClientSocket(
                            socket = this,
                            builders = application.builders,
                            readers = Array(255) { application.readers.firstOrNull { reader -> reader.id == it } },
                        )
                        engine.environment.log.info("WS: New Connection!")
                        client.accept(session)
                    }
                }
            }
            engine.start(wait = true)
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
    val readers: Set<PacketReader<Packet>>
) {
    init {
        Runtime.getRuntime().addShutdownHook(ShutdownHook(engine))
    }
}
