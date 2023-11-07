package com.lostcity.tcp

import com.google.inject.Provider
import com.google.inject.Singleton
import io.ktor.network.selector.SelectorManager
import kotlinx.coroutines.Dispatchers

@Singleton
class ServerSocketSelectorProvider : Provider<SelectorManager> {
    override fun get(): SelectorManager = SelectorManager(Dispatchers.IO)
}
