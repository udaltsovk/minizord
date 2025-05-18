package ru.udaltsovk.minizord.email.repository

import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.repository.ListPagingAndSortingRepository
import ru.udaltsovk.minizord.email.entity.Email
import ru.udaltsovk.minizord.email.proto.EmailType
import java.util.UUID

/**
 * Репозиторий для работы с сущностями Email.
 */
interface EmailRepository : JpaRepository<Email, UUID>, ListPagingAndSortingRepository<Email, UUID> {
    /**
     * Находит все электронные письма по типу и получателю с пагинацией.
     *
     * @param emailType Тип электронного письма.
     * @param receiver Адрес получателя.
     * @param pageable Информация о пагинации.
     * @return Страница с электронными письмами.
     */
    fun findAllByEmailTypeAndReceiver(emailType: EmailType?, receiver: String?, pageable: Pageable): Page<Email>

    /**
     * Находит все электронные письма по типу с пагинацией.
     *
     * @param emailType Тип электронного письма.
     * @param pageable Информация о пагинации.
     * @return Страница с электронными письмами.
     */
    fun findAllByEmailType(emailType: EmailType?, pageable: Pageable): Page<Email>

    /**
     * Находит все электронные письма по получателю с пагинацией.
     *
     * @param receiver Адрес получателя.
     * @param pageable Информация о пагинации.
     * @return Страница с электронными письмами.
     */
    fun findAllByReceiver(receiver: String?, pageable: Pageable): Page<Email>
}
