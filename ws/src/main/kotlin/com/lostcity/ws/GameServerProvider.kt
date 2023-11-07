package com.lostcity.ws

import com.google.inject.Inject
import com.google.inject.Provider
import com.google.inject.Singleton
import com.lostcity.game.GameServer
import java.security.interfaces.RSAPrivateCrtKey

@Singleton
class GameServerProvider @Inject constructor(
    private val rsaPrivateCrtKey: RSAPrivateCrtKey
) : Provider<GameServer> {
    override fun get(): GameServer {
        return GameServer(rsaPrivateCrtKey)
    }
}