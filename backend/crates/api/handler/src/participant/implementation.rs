use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use actix_web_validation::Validated;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    participant::{CreateParticipant, Participant, ParticipantUpdate},
};
use macros::handler_implementation;
use service::participant::ParticipantServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

use super::ParticipantAuthResponse;
use crate::common::{ApiError, ValidationError, middleware::auth::AuthEntity};

handler_implementation! {
    ParticipantHandler as Implemented {
        ///
        ///
        ///
        #[openapi(
            operation_id = "register_participant",
            request_body(
                description = "",
                content = CreateParticipant
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 201, description = "", body = ParticipantAuthResponse),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/register")]
        register(
            participant_service: Data<ParticipantServiceDependency>,
            Validated(Json(body)): Validated<Json<CreateParticipant>>
        ) -> HttpResponse {
            let resp: ParticipantAuthResponse = participant_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "participant_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = ParticipantAuthResponse),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
            ),
        )]
        #[post("/login")]
        login(
            participant_service: Data<ParticipantServiceDependency>,
            Validated(Json(body)): Validated<Json<LoginRequest>>,
        ) -> Json<ParticipantAuthResponse> {
            let res = participant_service
                .login(body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_current_participant",
            security(
                ("participant" = []),
            ),
            responses(
                (status = 200, description = "", body = Participant),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),

        )]
        #[get("/me")]
        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Participant> {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;
            Json(participant)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_current_participant",
            request_body(
                description = "",
                content = ParticipantUpdate
            ),
            security(
                ("participant" = []),
            ),
            responses(
                (status = 200, description = "", body = Participant),
                (status = 409, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/me")]
        update_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
            Validated(Json(body)): Validated<Json<ParticipantUpdate>>,
        ) -> Json<Participant> {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;


            if body.name.as_ref().unwrap_or(&participant.name) == &participant.name
                && body.surname.as_ref().unwrap_or(&participant.surname) == &participant.surname
                && body.bio.as_ref().unwrap_or(&participant.bio) == &participant.bio
                && body.portfolio_urls.as_ref().unwrap_or(&participant.portfolio_urls) == &participant.portfolio_urls
            {
                return Ok(Json(participant));
            }
            let res = participant_service
                .update_by_id(participant.id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "change_current_participant_password",
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("participant" = []),
            ),
            responses(
                (status = 200, description = "", body = ParticipantAuthResponse),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/me/password")]
        change_password_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<ParticipantAuthResponse> {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;
            let res = participant_service
                .change_password_by_id(participant.id, body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_current_participant",
            security(
                ("participant" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("/me")]
        delete_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
        ) -> HttpResponse {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;
            participant_service
                .delete_by_id(participant.id)
                .await?;
            HttpResponse::NoContent().into()
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "get_participant_by_id",
            params(
                ("participant_id" = Ulid, description = "")
            ),
            security(
                ("organizator" = []),
                ("participant" = []),
                ("participant" = [])
            ),
            responses(
                (status = 200, description = "", body = Participant),
                (status = 404, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[get("/{participant_id}")]
        get_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
        ) -> Json<Participant> {
            let res = participant_service
                .get_by_id(participant_id)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "update_participant_by_id",
            params(
                ("participant_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = ParticipantUpdate
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = Participant),
                (status = 409, description = "", body = ApiError),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("/{participant_id}")]
        update_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<ParticipantUpdate>>,
        ) -> Json<Participant> {
            let res = participant_service
                .update_by_id(participant_id, body)
                .await?;
            Json(res)
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "change_participant_password_by_id",
            params(
                ("participant_id" = Ulid, description = "")
            ),
            request_body(
                description = "",
                content = PasswordChangeRequest
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 200, description = "", body = ParticipantAuthResponse),
                (status = 404, description = "", body = ApiError),
                (status = 400, description = "", body = ValidationError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/{participant_id}/password")]
        change_password_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
            Validated(Json(body)): Validated<Json<PasswordChangeRequest>>,
        ) -> Json<ParticipantAuthResponse> {
            let res = participant_service
                .change_password_by_id(participant_id, body)
                .await?;
            Json(res.into())
        }

        ///
        ///
        ///
        #[openapi(
            operation_id = "delete_participant_by_id",
            params(
                ("participant_id" = Ulid, description = "")
            ),
            security(
                ("organizator" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 404, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
            ),
        )]
        #[delete("/{participant_id}")]
        delete_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
        ) -> HttpResponse {
            participant_service
                .delete_by_id(participant_id)
                .await?;
            HttpResponse::NoContent().into()
        }
    }
}
