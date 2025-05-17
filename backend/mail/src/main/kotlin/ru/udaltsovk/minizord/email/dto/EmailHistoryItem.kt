package ru.udaltsovk.minizord.email.dto

import ru.udaltsovk.minizord.email.entity.Email
import ru.udaltsovk.minizord.email.exception.InvalidRequestException
import ru.udaltsovk.minizord.email.proto.EmailType
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemResponse
import ru.udaltsovk.minizord.email.proto.SendEmailRequest

fun Email.toDto(): GetEmailHistoryItemResponse =
    GetEmailHistoryItemResponse.newBuilder()
        .setId(id.toString())
        .setReceiver(receiver)
        .setEmailType(emailType)
        .setSentAt(sentAt.toTimestamp())
        .build()

fun SendEmailRequest.toEntity() = Email(
    receiver = receiver,
    emailType = when {
        hasRegistrationEmail() -> EmailType.REGISTRATION_EMAIL
        hasPasswordResetEmail() -> EmailType.PASSWORD_RESET_EMAIL
        else -> throw InvalidRequestException()
    }
)