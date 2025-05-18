package ru.udaltsovk.minizord.email.config

import io.opentelemetry.api.common.AttributeKey
import io.opentelemetry.api.trace.SpanKind
import io.opentelemetry.contrib.sampler.RuleBasedRoutingSampler
import io.opentelemetry.sdk.autoconfigure.spi.AutoConfigurationCustomizer
import io.opentelemetry.sdk.autoconfigure.spi.AutoConfigurationCustomizerProvider
import io.opentelemetry.sdk.autoconfigure.spi.ConfigProperties
import io.opentelemetry.sdk.trace.samplers.Sampler
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import java.util.function.BiFunction

/**
 * Конфигурация для фильтрации путей трассировки OpenTelemetry.
 */
@Configuration
class FilterPaths {
    /**
     * Настраивает семплер OpenTelemetry для исключения путей actuator.
     * @return Поставщик кастомизатора автоконфигурации.
     */
    @Bean
    fun otelCustomizer(): AutoConfigurationCustomizerProvider {
        return AutoConfigurationCustomizerProvider { p: AutoConfigurationCustomizer? ->
            p!!.addSamplerCustomizer(
                BiFunction { fallback: Sampler?, config: ConfigProperties? ->
                    RuleBasedRoutingSampler.builder(SpanKind.SERVER, fallback)
                        .drop(AttributeKey.stringKey("http.url"), "^/actuator").build()
                }
            )
        }
    }
}
