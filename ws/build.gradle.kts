plugins {
    application
    alias(deps.plugins.jvm)
    alias(deps.plugins.shadow)
}

dependencies {
    implementation(deps.bundles.ktor)
    implementation(deps.guice)
    implementation(deps.bouncycastle)

    implementation(project(":game"))
    implementation(project(":session"))
    implementation(project(":socket"))
}

application {
    mainClass.set("com.lostcity.ws.ApplicationKt")
    applicationDefaultJvmArgs += listOf(
        "-Xmx512m",
        "-XX:+UseZGC"
    )
}
