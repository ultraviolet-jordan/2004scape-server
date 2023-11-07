plugins {
    alias(deps.plugins.jvm)
}

dependencies {
    implementation(deps.bundles.ktor)

    implementation(project(":jagex2"))
}
