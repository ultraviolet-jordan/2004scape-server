package com.lostcity.game

import java.security.interfaces.RSAPrivateCrtKey

class GameServer(
    private val rsaPrivateCrtKey: RSAPrivateCrtKey
) {
    fun start(function: (RSAPrivateCrtKey) -> Unit) {
        function.invoke(rsaPrivateCrtKey)
    }
}