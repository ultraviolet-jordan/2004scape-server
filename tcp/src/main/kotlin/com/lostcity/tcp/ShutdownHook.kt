package com.lostcity.tcp

import io.ktor.server.application.*
import io.ktor.server.engine.ApplicationEngine
import io.ktor.server.engine.stop
import java.util.concurrent.TimeUnit

class ShutdownHook(
    private val engine: ApplicationEngine
) : Thread() {
    override fun run() {
        engine.environment.log.info("Running shutdown hook...")
        engine.environment.log.info("Shutting down the application engine...")
        engine.stop(3, 5, TimeUnit.SECONDS)
    }
}
