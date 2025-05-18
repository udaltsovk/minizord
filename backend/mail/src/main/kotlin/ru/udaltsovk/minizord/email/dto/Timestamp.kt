package ru.udaltsovk.minizord.email.dto

import com.google.protobuf.Timestamp
import java.time.LocalDateTime
import java.time.ZoneOffset

/**
 * Преобразует [LocalDateTime] в [Timestamp] Google Protobuf.
 * @return Объект Timestamp.
 */
fun LocalDateTime.toTimestamp(): Timestamp {
    val instant = this.toInstant(ZoneOffset.UTC)
    return Timestamp.newBuilder()
        .setSeconds(instant.epochSecond)
        .setNanos(instant.nano)
        .build()
}
