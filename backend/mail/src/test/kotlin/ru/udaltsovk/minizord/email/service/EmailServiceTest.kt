package ru.udaltsovk.minizord.email.service

import com.samskivert.mustache.Mustache
import com.samskivert.mustache.Template
import io.micrometer.core.instrument.Counter
import io.micrometer.core.instrument.MeterRegistry
import jakarta.mail.internet.MimeMessage
import org.junit.Assert.assertThrows
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith
import org.mockito.ArgumentCaptor
import org.mockito.ArgumentMatchers.any
import org.mockito.ArgumentMatchers.anyMap
import org.mockito.ArgumentMatchers.anyString
import org.mockito.ArgumentMatchers.eq
import org.mockito.Captor
import org.mockito.InjectMocks
import org.mockito.Mock
import org.mockito.Mockito.doThrow
import org.mockito.Mockito.never
import org.mockito.Mockito.verify
import org.mockito.Mockito.`when`
import org.mockito.junit.jupiter.MockitoExtension
import org.springframework.data.domain.PageImpl
import org.springframework.data.domain.Pageable
import org.springframework.mail.MailSendException
import org.springframework.mail.javamail.JavaMailSender
import ru.udaltsovk.minizord.email.entity.Email
import ru.udaltsovk.minizord.email.exception.EmailNotFoundException
import ru.udaltsovk.minizord.email.exception.InvalidRequestException
import ru.udaltsovk.minizord.email.proto.EmailType
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemRequest
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryRequest
import ru.udaltsovk.minizord.email.proto.SendEmailRequest
import ru.udaltsovk.minizord.email.proto.SendEmailRequest.PasswordResetEmail
import ru.udaltsovk.minizord.email.proto.SendEmailRequest.RegistrationEmail
import ru.udaltsovk.minizord.email.repository.EmailRepository
import java.util.Optional
import java.util.UUID

/**
 * Тесты для [EmailService].
 */
@ExtendWith(MockitoExtension::class)
class EmailServiceTest {

    @Mock
    private lateinit var emailRepository: EmailRepository

    @Mock
    private lateinit var mailSender: JavaMailSender

    @Mock
    private lateinit var mustacheCompiler: Mustache.Compiler

    @Mock
    private lateinit var meterRegistry: MeterRegistry

    @Mock
    private lateinit var mockMimeMessage: MimeMessage

    @Mock
    private lateinit var mockTemplate: Template

    @Mock
    private lateinit var mockCounter: Counter

    @InjectMocks
    private lateinit var emailService: EmailService

    @Captor
    private lateinit var mimeMessageCaptor: ArgumentCaptor<MimeMessage>

    @Captor
    private lateinit var pageableCaptor: ArgumentCaptor<Pageable>

    private val platformUrl = "http://localhost:3000"
    private val mailAddress = "noreply@minizord.test"

    /**
     * Настраивает тестовое окружение перед каждым тестом.
     */
    @BeforeEach
    fun setUp() {
        // Reflectively set values for @Value annotated fields
        val serviceClass = EmailService::class.java
        val platformUrlField = serviceClass.getDeclaredField("platformUrl")
        platformUrlField.isAccessible = true
        platformUrlField.set(emailService, platformUrl)

        val mailAddressField = serviceClass.getDeclaredField("mailAddress")
        mailAddressField.isAccessible = true
        mailAddressField.set(emailService, mailAddress)

        `when`(meterRegistry.counter(anyString(), anyString(), anyString())).thenReturn(mockCounter)
        val meterRegistryField = serviceClass.getDeclaredField("meterRegistry")
        meterRegistryField.isAccessible = true
        meterRegistryField.set(emailService, meterRegistry)
        emailService.javaClass.getDeclaredMethod(
            "sendEmail",
            SendEmailRequest::class.java
        )
    }

    /**
     * Тест проверяет успешную отправку письма о регистрации.
     */
    @Test
    fun `sendEmail should send registration email successfully`() {
        val request = SendEmailRequest.newBuilder().setReceiver("test@example.com")
            .setRegistrationEmail(RegistrationEmail.newBuilder().setPassword("password123")).build()

        val emailEntity = Email(
            receiver = "test@example.com",
            emailType = EmailType.REGISTRATION_EMAIL,
        )
        val savedEmailEntity = emailEntity.copy(id = UUID.randomUUID())

        `when`(mailSender.createMimeMessage()).thenReturn(mockMimeMessage)
        `when`(mustacheCompiler.loadTemplate("email/registration")).thenReturn(mockTemplate)
        `when`(mockTemplate.execute(anyMap<String, String>())).thenReturn("Email Content")
        `when`(emailRepository.save(any(Email::class.java))).thenReturn(savedEmailEntity)

        val response = emailService.sendEmail(request)

        Assertions.assertTrue(response.successful)
        verify(mailSender).send(mimeMessageCaptor.capture())
        Assertions.assertEquals(mailAddress, mimeMessageCaptor.value.from[0].toString())
        Assertions.assertEquals(
            "test@example.com",
            mimeMessageCaptor.value.getRecipients(MimeMessage.RecipientType.TO)[0].toString()
        )
        Assertions.assertEquals("Minizord: завершение регистрации", mimeMessageCaptor.value.subject)
        Assertions.assertTrue(mimeMessageCaptor.value.getHeader("Content-Type")[0].contains("text/html;charset=utf-8"))
        verify(emailRepository).save(any(Email::class.java))
        verify(mockCounter).increment()
    }

    /**
     * Тест проверяет успешную отправку письма о сбросе пароля.
     */
    @Test
    fun `sendEmail should send password reset email successfully`() {
        val request = SendEmailRequest.newBuilder().setReceiver("test@example.com")
            .setPasswordResetEmail(PasswordResetEmail.newBuilder().setNewPassword("newPassword")).build()

        val emailEntity = Email(
            receiver = "test@example.com",
            emailType = EmailType.PASSWORD_RESET_EMAIL,
        )
        val savedEmailEntity = emailEntity.copy(id = UUID.randomUUID())

        `when`(mailSender.createMimeMessage()).thenReturn(mockMimeMessage)
        `when`(mustacheCompiler.loadTemplate("email/password_reset")).thenReturn(mockTemplate)
        `when`(mockTemplate.execute(anyMap<String, String>())).thenReturn("Email Content")
        `when`(emailRepository.save(any(Email::class.java))).thenReturn(savedEmailEntity)

        val response = emailService.sendEmail(request)

        Assertions.assertTrue(response.successful)
        verify(mailSender).send(mimeMessageCaptor.capture())
        Assertions.assertEquals("Minizord: сброс пароля", mimeMessageCaptor.value.subject)
        verify(emailRepository).save(any(Email::class.java))
        verify(mockCounter).increment()
    }

    /**
     * Тест проверяет, что метод возвращает неуспешный результат при [org.springframework.mail.MailException].
     */
    @Test
    fun `sendEmail should return unsuccessful on MailException`() {
        val request = SendEmailRequest.newBuilder().setReceiver("test@example.com")
            .setRegistrationEmail(RegistrationEmail.newBuilder().setPassword("password123")).build()

        `when`(mailSender.createMimeMessage()).thenReturn(mockMimeMessage)
        `when`(mustacheCompiler.loadTemplate(anyString())).thenReturn(mockTemplate)
        `when`(mockTemplate.execute(anyMap<String, String>())).thenReturn("Email Content")
        doThrow(MailSendException("Failed to send")).`when`(mailSender).send(any(MimeMessage::class.java))

        val response = emailService.sendEmail(request)

        Assertions.assertFalse(response.successful)
        verify(emailRepository, never()).save(any(Email::class.java))
        verify(mockCounter, never()).increment()
    }

    /**
     * Тест проверяет, что выбрасывается [InvalidRequestException] для нераспознанного типа письма.
     */
    @Test
    fun `sendEmail should throw InvalidRequestException for UNRECOGNIZED email type`() {
        val request = SendEmailRequest.newBuilder().setReceiver("test@example.com").build()

        assertThrows(InvalidRequestException::class.java) {
            emailService.sendEmail(request)
        }
    }

    /**
     * Тест проверяет получение истории писем с пагинацией по умолчанию.
     */
    @Test
    fun `getEmailHistory should return history with default pagination`() {
        val request = GetEmailHistoryRequest.newBuilder().build()
        val emailEntity = Email(
            id = UUID.randomUUID(),
            receiver = "test@example.com",
            emailType = EmailType.REGISTRATION_EMAIL,
        )
        val page = PageImpl(listOf(emailEntity))
        `when`(emailRepository.findAll(any(Pageable::class.java))).thenReturn(page)

        val response = emailService.getEmailHistory(request)

        Assertions.assertEquals(1, response.historyList.size)
        verify(emailRepository).findAll(pageableCaptor.capture())
        Assertions.assertEquals(0, pageableCaptor.value.pageNumber)
        Assertions.assertEquals(7, pageableCaptor.value.pageSize)
    }

    /**
     * Тест проверяет получение истории писем с пользовательской пагинацией.
     */
    @Test
    fun `getEmailHistory should return history with custom pagination`() {
        val request = GetEmailHistoryRequest.newBuilder().setPage(1).setSize(10).build()
        val emailEntity = Email(
            id = UUID.randomUUID(),
            receiver = "test@example.com",
            emailType = EmailType.REGISTRATION_EMAIL,
        )
        val page = PageImpl(listOf(emailEntity))
        `when`(emailRepository.findAll(any(Pageable::class.java))).thenReturn(page)

        emailService.getEmailHistory(request)

        verify(emailRepository).findAll(pageableCaptor.capture())
        Assertions.assertEquals(1, pageableCaptor.value.pageNumber)
        Assertions.assertEquals(10, pageableCaptor.value.pageSize)
    }

    /**
     * Тест проверяет фильтрацию истории писем по типу и получателю.
     */
    @Test
    fun `getEmailHistory should filter by emailType and receiver`() {
        val request = GetEmailHistoryRequest.newBuilder().setEmailType(EmailType.REGISTRATION_EMAIL)
            .setReceiver("test@example.com").build()
        val emailEntity = Email(
            id = UUID.randomUUID(),
            receiver = "test@example.com",
            emailType = EmailType.REGISTRATION_EMAIL,
        )
        val page = PageImpl(listOf(emailEntity))
        `when`(
            emailRepository.findAllByEmailTypeAndReceiver(
                eq(EmailType.REGISTRATION_EMAIL),
                eq("test@example.com"),
                any(Pageable::class.java)
            )
        ).thenReturn(page)

        val response = emailService.getEmailHistory(request)

        Assertions.assertEquals(1, response.historyList.size)
        verify(emailRepository).findAllByEmailTypeAndReceiver(
            eq(EmailType.REGISTRATION_EMAIL),
            eq("test@example.com"),
            any(Pageable::class.java)
        )
    }

    /**
     * Тест проверяет фильтрацию истории писем по типу.
     */
    @Test
    fun `getEmailHistory should filter by emailType`() {
        val request = GetEmailHistoryRequest.newBuilder().setEmailType(EmailType.PASSWORD_RESET_EMAIL).build()
        val emailEntity = Email(
            id = UUID.randomUUID(),
            receiver = "another@example.com",
            emailType = EmailType.PASSWORD_RESET_EMAIL,
        )
        val page = PageImpl(listOf(emailEntity))
        `when`(
            emailRepository.findAllByEmailType(
                eq(EmailType.PASSWORD_RESET_EMAIL),
                any(Pageable::class.java)
            )
        ).thenReturn(page)

        val response = emailService.getEmailHistory(request)

        Assertions.assertEquals(1, response.historyList.size)
        verify(emailRepository).findAllByEmailType(
            eq(EmailType.PASSWORD_RESET_EMAIL),
            any(Pageable::class.java)
        )
    }

    /**
     * Тест проверяет фильтрацию истории писем по получателю.
     */
    @Test
    fun `getEmailHistory should filter by receiver`() {
        val request = GetEmailHistoryRequest.newBuilder().setReceiver("user@domain.com").build()
        val emailEntity = Email(
            id = UUID.randomUUID(),
            receiver = "user@domain.com",
            emailType = EmailType.REGISTRATION_EMAIL,
        )
        val page = PageImpl(listOf(emailEntity))
        `when`(
            emailRepository.findAllByReceiver(
                eq("user@domain.com"),
                any(Pageable::class.java)
            )
        ).thenReturn(page)

        val response = emailService.getEmailHistory(request)

        Assertions.assertEquals(1, response.historyList.size)
        verify(emailRepository).findAllByReceiver(eq("user@domain.com"), any(Pageable::class.java))
    }

    /**
     * Тест проверяет получение письма по ID, когда оно найдено.
     */
    @Test
    fun `getById should return email when found`() {
        val id = UUID.randomUUID()
        val request = GetEmailHistoryItemRequest.newBuilder().setId(id.toString()).build()
        val emailEntity = Email(
            id = id,
            receiver = "test@example.com",
            emailType = EmailType.REGISTRATION_EMAIL,
        )
        `when`(emailRepository.findById(id)).thenReturn(Optional.of(emailEntity))

        val response = emailService.getById(request)

        Assertions.assertEquals(id.toString(), response.id)
        Assertions.assertEquals("test@example.com", response.receiver)
    }

    /**
     * Тест проверяет, что выбрасывается [EmailNotFoundException], когда письмо не найдено.
     */
    @Test
    fun `getById should throw EmailNotFoundException when not found`() {
        val id = UUID.randomUUID()
        val request = GetEmailHistoryItemRequest.newBuilder().setId(id.toString()).build()
        `when`(emailRepository.findById(id)).thenReturn(Optional.empty())

        assertThrows(EmailNotFoundException::class.java) {
            emailService.getById(request)
        }
    }

    /**
     * Тест проверяет, что выбрасывается [InvalidRequestException] для невалидного UUID.
     */
    @Test
    fun `getById should throw InvalidRequestException for invalid UUID`() {
        val request = GetEmailHistoryItemRequest.newBuilder().setId("invalid-uuid").build()

        assertThrows(InvalidRequestException::class.java) {
            emailService.getById(request)
        }
    }
}
