rootProject.name = "rust-demo"

pluginManagement {
    val smithyGradleVersion: String by settings
    plugins {
        id("software.amazon.smithy.gradle.smithy-jar").version(smithyGradleVersion)
        id("software.amazon.smithy.gradle.smithy-base").version(smithyGradleVersion)
    }

    repositories {
        mavenLocal()
        mavenCentral()
        gradlePluginPortal()
    }
}

// === Modules ===
include("catalog-svc-smithy")
include("catalog-svc-smithy:client")
include("catalog-svc-smithy:smithy")
include("catalog-svc-smithy:server")

include("demo-notes2")
include("demo-notes2:client")
include("demo-notes2:smithy")
include("demo-notes2:server")

