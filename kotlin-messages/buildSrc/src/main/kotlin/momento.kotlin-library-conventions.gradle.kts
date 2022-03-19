/*
 * This file was generated by the Gradle 'init' task.
 */

plugins {
    // Apply the common convention plugin for shared build configuration between library and application projects.
    id("momento.kotlin-common-conventions")

    // Apply the java-library plugin for API and implementation separation.
    `java-library`
    `maven-publish`
}

val getCodeArtifactToken: () -> String by extra

configure<PublishingExtension> {
    repositories {
        maven {
            url = uri("https://momento-prod-401011790710.d.codeartifact.us-west-2.amazonaws.com/maven/maven-momento/")
            credentials {
                username = "aws"
                password = getCodeArtifactToken()
            }
        }
    }
}