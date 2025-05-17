package ru.udaltsovk.minizord.email.controller

import io.grpc.stub.StreamObserver
import org.springframework.grpc.server.service.GrpcService
import ru.udaltsovk.minizord.email.exception.GotNullRequestException
import ru.udaltsovk.minizord.email.exception.GotNullResponseObserverException
import ru.udaltsovk.minizord.email.proto.EmailGrpc
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemRequest
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryItemResponse
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryRequest
import ru.udaltsovk.minizord.email.proto.GetEmailHistoryResponse
import ru.udaltsovk.minizord.email.proto.SendEmailRequest
import ru.udaltsovk.minizord.email.proto.SendEmailResponse
import ru.udaltsovk.minizord.email.service.EmailService

@GrpcService
class EmailController(
    private val emailService: EmailService,
): EmailGrpc.EmailImplBase() {
    override fun sendEmail(request: SendEmailRequest?, responseObserver: StreamObserver<SendEmailResponse?>?) {
        if (request == null) {
            throw GotNullRequestException()
        }
        if (responseObserver == null) {
            throw GotNullResponseObserverException()
        }

        responseObserver.onNext(emailService.sendEmail(request))
        responseObserver.onCompleted()
    }

    override fun getEmailHistory(
        request: GetEmailHistoryRequest?,
        responseObserver: StreamObserver<GetEmailHistoryResponse?>?
    ) {
        if (request == null) {
            throw GotNullRequestException()
        }
        if (responseObserver == null) {
            throw GotNullResponseObserverException()
        }

        responseObserver.onNext(emailService.getEmailHistory(request))
        responseObserver.onCompleted()
    }

    override fun getEmailHistoryItem(
        request: GetEmailHistoryItemRequest?,
        responseObserver: StreamObserver<GetEmailHistoryItemResponse?>?
    ) {
        if (request == null) {
            throw GotNullRequestException()
        }
        if (responseObserver == null) {
            throw GotNullResponseObserverException()
        }

        responseObserver.onNext(emailService.getById(request))
        responseObserver.onCompleted()
    }
}