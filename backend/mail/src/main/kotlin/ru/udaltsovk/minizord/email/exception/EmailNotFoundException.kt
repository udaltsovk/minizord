package ru.udaltsovk.minizord.email.exception

import jakarta.persistence.EntityNotFoundException

/**
 * Исключение, выбрасываемое, когда электронное письмо не найдено.
 */
class EmailNotFoundException : EntityNotFoundException()
