package ru.udaltsovk.minizord.email.dto

import ru.udaltsovk.minizord.email.entity.Email
import ru.udaltsovk.minizord.email.exception.InvalidRequestException
import ru.udaltsovk.minizord.email.proto.EmailType
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemResponse
import ru.udaltsovk.minizord.email.proto.SendEmailRequest

/**
 * Преобразует сущность [Email] в DTO [GetEmailHistoryItemResponse].
 * @return DTO элемент истории писем.
 */
fun Email.toDto(): GetEmailHistoryItemResponse =
    GetEmailHistoryItemResponse.newBuilder()
        .setId(id.toString())
        .setReceiver(receiver)
        .setEmailType(emailType)
        .setSentAt(sentAt.toTimestamp())
        .build()

/**
 * Преобразует DTO [SendEmailRequest] в сущность [Email].
 * @return Сущность письма.
 * @throws InvalidRequestException если тип письма не распознан.
 */
fun SendEmailRequest.toEntity() = Email(
    receiver = receiver,
    emailType = when {
        hasRegistrationEmail() -> EmailType.REGISTRATION_EMAIL
        hasPasswordResetEmail() -> EmailType.PASSWORD_RESET_EMAIL
        else -> throw InvalidRequestException()
    }
)
