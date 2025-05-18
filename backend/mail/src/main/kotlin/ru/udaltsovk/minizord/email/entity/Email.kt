package ru.udaltsovk.minizord.email.entity

import jakarta.persistence.Column
import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.GenerationType
import jakarta.persistence.Id
import org.hibernate.proxy.HibernateProxy
import ru.udaltsovk.minizord.email.proto.EmailType
import java.time.LocalDateTime
import java.util.UUID

/**
 * Сущность, представляющая отправленное электронное письмо.
 * @property id Уникальный идентификатор письма.
 * @property receiver Получатель письма.
 * @property emailType Тип письма.
 * @property sentAt Время отправки письма.
 */
@Entity(name = "sent_emails")
data class Email(
    /**
     * Уникальный идентификатор письма.
     */
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    val id: UUID? = null,

    /**
     * Получатель письма.
     */
    @Column(nullable = false)
    val receiver: String,

    /**
     * Тип письма.
     */
    @Column(nullable = false)
    val emailType: EmailType,

    /**
     * Время отправки письма.
     */
    @Column(nullable = false)
    val sentAt: LocalDateTime = LocalDateTime.now(),
) {
    final override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other == null) return false
        val oEffectiveClass =
            if (other is HibernateProxy) other.hibernateLazyInitializer.persistentClass else other.javaClass
        val thisEffectiveClass =
            if (this is HibernateProxy) this.hibernateLazyInitializer.persistentClass else this.javaClass
        if (thisEffectiveClass != oEffectiveClass) return false
        other as Email

        return id == other.id
    }

    final override fun hashCode(): Int =
        if (this is HibernateProxy) this.hibernateLazyInitializer.persistentClass.hashCode() else javaClass.hashCode()

    @Override
    override fun toString(): String {
        return this::class.simpleName + "(  id = $id )"
    }
}
