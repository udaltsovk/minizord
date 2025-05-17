package ru.udaltsovk.minizord.email.repository

import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.jpa.repository.JpaRepository
import org.springframework.data.repository.ListPagingAndSortingRepository
import ru.udaltsovk.minizord.email.entity.Email
import ru.udaltsovk.minizord.email.proto.EmailType
import java.util.UUID

interface EmailRepository : JpaRepository<Email, UUID>, ListPagingAndSortingRepository<Email, UUID> {
    fun findAllByEmailTypeAndReceiver(emailType: EmailType?, receiver: String?, pageable: Pageable): Page<Email>

    fun findAllByEmailType(emailType: EmailType?, pageable: Pageable): Page<Email>

    fun findAllByReceiver(receiver: String?, pageable: Pageable): Page<Email>
}