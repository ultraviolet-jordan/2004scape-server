package com.lostcity.ws

import com.lostcity.game.GameServer
import dev.misfitlabs.kotlinguice4.KotlinModule
import io.ktor.server.application.*
import io.ktor.server.engine.*
import java.security.interfaces.RSAPrivateCrtKey

class WSModule(
    private val args: Array<String>
) : KotlinModule() {
    override fun configure() {
        bind<ApplicationArguments>().toInstance(ApplicationArguments(args))
        bind<ApplicationEnvironment>().toProvider<ApplicationEnvironmentProvider>().asEagerSingleton()
        bind<ApplicationEngine>().toProvider<ApplicationEngineProvider>().asEagerSingleton()
        bind<RSAPrivateCrtKey>().toProvider<RSAPrivateCrtKeyProvider>().asEagerSingleton()
        bind<GameServer>().toProvider<GameServerProvider>().asEagerSingleton()
    }
}