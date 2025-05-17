package ru.udaltsovk.minizord.email.controller

import io.grpc.stub.StreamObserver
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith
import org.mockito.ArgumentCaptor
import org.mockito.Captor
import org.mockito.InjectMocks
import org.mockito.Mock
import org.mockito.Mockito.*
import org.mockito.junit.jupiter.MockitoExtension
import ru.udaltsovk.minizord.email.exception.GotNullRequestException
import ru.udaltsovk.minizord.email.exception.GotNullResponseObserverException
import ru.udaltsovk.minizord.email.proto.*
import ru.udaltsovk.minizord.email.service.EmailService

@ExtendWith(MockitoExtension::class)
class EmailControllerTest {

    @Mock
    private lateinit var emailService: EmailService

    @InjectMocks
    private lateinit var emailController: EmailController

    @Mock
    private lateinit var mockSendEmailResponseObserver: StreamObserver<SendEmailResponse?>

    @Mock
    private lateinit var mockGetEmailHistoryResponseObserver: StreamObserver<GetEmailHistoryResponse?>

    @Mock
    private lateinit var mockGetEmailHistoryItemResponseObserver: StreamObserver<GetEmailHistoryItemResponse?>

    @Captor
    private lateinit var sendEmailResponseCaptor: ArgumentCaptor<SendEmailResponse>

    @Captor
    private lateinit var getEmailHistoryResponseCaptor: ArgumentCaptor<GetEmailHistoryResponse>

    @Captor
    private lateinit var getEmailHistoryItemResponseCaptor: ArgumentCaptor<GetEmailHistoryItemResponse>

    @Test
    fun `sendEmail should call service and complete`() {
        val request = SendEmailRequest.newBuilder().setReceiver("test@example.com").build()
        val serviceResponse = SendEmailResponse.newBuilder().setSuccessful(true).build()

        `when`(emailService.sendEmail(request)).thenReturn(serviceResponse)

        emailController.sendEmail(request, mockSendEmailResponseObserver)

        verify(emailService).sendEmail(request)
        verify(mockSendEmailResponseObserver).onNext(sendEmailResponseCaptor.capture())
        verify(mockSendEmailResponseObserver).onCompleted()
        assertTrue(sendEmailResponseCaptor.value.successful)
    }

    @Test
    fun `sendEmail should throw GotNullRequestException when request is null`() {
        assertThrows(GotNullRequestException::class.java) {
            emailController.sendEmail(null, mockSendEmailResponseObserver)
        }
        verify(mockSendEmailResponseObserver, never()).onNext(any())
        verify(mockSendEmailResponseObserver, never()).onCompleted()
    }

    @Test
    fun `sendEmail should throw GotNullResponseObserverException when observer is null`() {
        val request = SendEmailRequest.newBuilder().build()
        assertThrows(GotNullResponseObserverException::class.java) {
            emailController.sendEmail(request, null)
        }
    }

    @Test
    fun `getEmailHistory should call service and complete`() {
        val request = GetEmailHistoryRequest.newBuilder().build()
        val serviceResponse = GetEmailHistoryResponse.newBuilder().addHistory(GetEmailHistoryItemResponse.newBuilder().setId("id")).build()

        `when`(emailService.getEmailHistory(request)).thenReturn(serviceResponse)

        emailController.getEmailHistory(request, mockGetEmailHistoryResponseObserver)

        verify(emailService).getEmailHistory(request)
        verify(mockGetEmailHistoryResponseObserver).onNext(getEmailHistoryResponseCaptor.capture())
        verify(mockGetEmailHistoryResponseObserver).onCompleted()
        assertEquals(1, getEmailHistoryResponseCaptor.value.historyCount)
        assertEquals("id", getEmailHistoryResponseCaptor.value.getHistory(0).id)
    }

    @Test
    fun `getEmailHistory should throw GotNullRequestException when request is null`() {
        assertThrows(GotNullRequestException::class.java) {
            emailController.getEmailHistory(null, mockGetEmailHistoryResponseObserver)
        }
        verify(mockGetEmailHistoryResponseObserver, never()).onNext(any())
        verify(mockGetEmailHistoryResponseObserver, never()).onCompleted()
    }

    @Test
    fun `getEmailHistory should throw GotNullResponseObserverException when observer is null`() {
        val request = GetEmailHistoryRequest.newBuilder().build()
        assertThrows(GotNullResponseObserverException::class.java) {
            emailController.getEmailHistory(request, null)
        }
    }

    @Test
    fun `getEmailHistoryItem should call service and complete`() {
        val request = GetEmailHistoryItemRequest.newBuilder().setId("test-id").build()
        val serviceResponse = GetEmailHistoryItemResponse.newBuilder().setId("test-id").setReceiver("test@example.com").build()

        `when`(emailService.getById(request)).thenReturn(serviceResponse)

        emailController.getEmailHistoryItem(request, mockGetEmailHistoryItemResponseObserver)

        verify(emailService).getById(request)
        verify(mockGetEmailHistoryItemResponseObserver).onNext(getEmailHistoryItemResponseCaptor.capture())
        verify(mockGetEmailHistoryItemResponseObserver).onCompleted()
        assertEquals("test-id", getEmailHistoryItemResponseCaptor.value.id)
        assertEquals("test@example.com", getEmailHistoryItemResponseCaptor.value.receiver)
    }

    @Test
    fun `getEmailHistoryItem should throw GotNullRequestException when request is null`() {
        assertThrows(GotNullRequestException::class.java) {
            emailController.getEmailHistoryItem(null, mockGetEmailHistoryItemResponseObserver)
        }
        verify(mockGetEmailHistoryItemResponseObserver, never()).onNext(any())
        verify(mockGetEmailHistoryItemResponseObserver, never()).onCompleted()
    }

    @Test
    fun `getEmailHistoryItem should throw GotNullResponseObserverException when observer is null`() {
        val request = GetEmailHistoryItemRequest.newBuilder().build()
        assertThrows(GotNullResponseObserverException::class.java) {
            emailController.getEmailHistoryItem(request, null)
        }
    }
}