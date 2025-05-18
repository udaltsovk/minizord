package ru.udaltsovk.minizord.email.config

import org.springframework.beans.factory.annotation.Value
import org.springframework.context.annotation.Configuration

/**
 * Конфигурация приложения.
 */
@Configuration
class ApplicationConfig(
    /**
     * URL платформы.
     */
    @Value("\${minizord.platform-url}")
    val platformUrl: String,

    /**
     * Адрес электронной почты.
     */
    @Value("\${minizord.mail-address}")
    val mailAddress: String,
)
