package ru.udaltsovk.minizord.email.service

import com.samskivert.mustache.Mustache
import com.samskivert.mustache.Template
import io.micrometer.core.instrument.Counter
import io.micrometer.core.instrument.MeterRegistry
import org.slf4j.LoggerFactory
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.data.domain.Pageable
import org.springframework.mail.MailException
import org.springframework.mail.javamail.JavaMailSender
import org.springframework.mail.javamail.MimeMessageHelper
import org.springframework.stereotype.Service
import ru.udaltsovk.minizord.email.config.ApplicationConfig
import ru.udaltsovk.minizord.email.dto.toDto
import ru.udaltsovk.minizord.email.dto.toEntity
import ru.udaltsovk.minizord.email.entity.Email
import ru.udaltsovk.minizord.email.exception.EmailNotFoundException
import ru.udaltsovk.minizord.email.exception.InvalidRequestException
import ru.udaltsovk.minizord.email.proto.EmailType
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemRequest
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemResponse
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryRequest
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryResponse
import ru.udaltsovk.minizord.email.proto.SendEmailRequest
import ru.udaltsovk.minizord.email.proto.SendEmailResponse
import ru.udaltsovk.minizord.email.repository.EmailRepository
import java.util.UUID

/**
 * Сервис для отправки электронных писем и получения истории отправленных писем.
 *
 * @property emailRepository Репозиторий для работы с сущностями Email.
 * @property mailSender Отправитель электронных писем JavaMailSender.
 * @property mustacheCompiler Компилятор шаблонов Mustache.
 * @property applicationConfig Конфигурация приложения.
 */
@Service
class EmailService(
    private val emailRepository: EmailRepository,
    private val mailSender: JavaMailSender,
    private val mustacheCompiler: Mustache.Compiler,
    private val applicationConfig: ApplicationConfig,
) {
    @Autowired
    private lateinit var meterRegistry: MeterRegistry

    private val logger = LoggerFactory.getLogger(this::class.java)

    private val mailCounter = Counter.builder("minizord.mail.sent")

    /**
     * Отправляет электронное письмо на указанный адрес.
     *
     * @param req Запрос на отправку письма, содержащий тип письма, получателя и необходимые данные.
     * @return Ответ, указывающий на успешность отправки письма.
     * @throws InvalidRequestException если тип письма не распознан.
     */
    fun sendEmail(req: SendEmailRequest): SendEmailResponse {
        val email = req.toEntity()

        val message = mailSender.createMimeMessage()
        val messageHelper = MimeMessageHelper(message)
        messageHelper.setFrom(applicationConfig.mailAddress)
        messageHelper.setTo(email.receiver)

        val subject: String
        val template: Template
        val templateParams: HashMap<String, String>
        val emailTypeString: String
        when (email.emailType) {
            EmailType.REGISTRATION_EMAIL -> {
                emailTypeString = "registration"
                subject = "Minizord: завершение регистрации"
                template = mustacheCompiler.loadTemplate("email/registration")
                templateParams = hashMapOf(
                    "platform_url" to applicationConfig.platformUrl,
                    "email" to email.receiver,
                    "password" to req.registrationEmail.password
                )
            }

            EmailType.PASSWORD_RESET_EMAIL -> {
                emailTypeString = "password_reset"
                subject = "Minizord: сброс пароля"
                template = mustacheCompiler.loadTemplate("email/password_reset")
                templateParams = hashMapOf(
                    "platform_url" to applicationConfig.platformUrl,
                    "password" to req.passwordResetEmail.newPassword,
                )
            }

            EmailType.UNRECOGNIZED -> throw InvalidRequestException()
        }
        messageHelper.setSubject(subject)
        messageHelper.setText(template.execute(templateParams))
        message.addHeader("Content-Type", "text/html;charset=utf-8")

        try {
            logger.debug("Sending $emailTypeString email to the `${email.receiver}` email address")
            mailSender.send(message)
        } catch (e: MailException) {
            logger.error(
                "Unable to send $emailTypeString email to the `${email.receiver}` email address. Reason: ${e.message}"
            )
            return SendEmailResponse.newBuilder()
                .setSuccessful(false)
                .build()
        }

        val savedEmail = emailRepository.save(email)

        val counter = mailCounter.tag("mail_type", emailTypeString).register(meterRegistry)
        counter.increment()
        logger.info("Sent $emailTypeString email (id = ${savedEmail.id}) to the `${email.receiver}` email address")

        return SendEmailResponse.newBuilder()
            .setSuccessful(true)
            .build()
    }

    /**
     * Возвращает историю отправленных электронных писем с возможностью пагинации и фильтрации.
     *
     * @param req Запрос на получение истории, содержащий параметры пагинации и фильтрации.
     * @return Ответ, содержащий список элементов истории писем.
     */
    fun getEmailHistory(req: GetEmailHistoryRequest): GetEmailHistoryResponse {
        val size = if (req.hasSize()) {
            req.size
        } else {
            7
        }
        val page = if (req.hasPage()) {
            req.page
        } else {
            0
        }
        logger.debug("Sending email history with $size items on the page $page")
        val pagination = Pageable.ofSize(size).withPage(page)
        val history = when {
            req.hasEmailType() && req.hasReceiver() -> emailRepository.findAllByEmailTypeAndReceiver(
                req.emailType,
                req.receiver,
                pagination,
            )

            req.hasEmailType() -> emailRepository.findAllByEmailType(
                req.emailType,
                pagination
            )

            req.hasReceiver() -> emailRepository.findAllByReceiver(
                req.receiver,
                pagination
            )

            else -> emailRepository.findAll(pagination)
        }.map(Email::toDto)

        logger.info("Sent email history with ${history.size} items with pagination (size: $size, page: $page)")

        return GetEmailHistoryResponse.newBuilder()
            .addAllHistory(history)
            .build()
    }

    /**
     * Возвращает информацию об отправленном электронном письме по его идентификатору.
     *
     * @param req Запрос на получение элемента истории, содержащий идентификатор письма.
     * @return Ответ, содержащий информацию об указанном письме.
     * @throws InvalidRequestException если идентификатор письма имеет неверный формат.
     * @throws EmailNotFoundException если письмо с указанным идентификатором не найдено.
     */
    fun getById(req: GetEmailHistoryItemRequest): GetEmailHistoryItemResponse {
        val id = try {
            UUID.fromString(req.id)
        } catch (_: IllegalArgumentException) {
            throw InvalidRequestException()
        }
        logger.debug("Sending email(id = {}) from the history", id)
        val email = emailRepository.findById(id).orElseThrow { EmailNotFoundException() }

        logger.info("Sent email(id = $id) from the history")
        return email.toDto()
    }
}
