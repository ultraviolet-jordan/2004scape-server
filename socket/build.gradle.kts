plugins {
    alias(deps.plugins.jvm)
}

dependencies {
    implementation(deps.bundles.ktor)
    implementation(deps.guice)

    implementation(project(":game"))
    implementation(project(":jagex2"))
    implementation(project(":session"))
}
