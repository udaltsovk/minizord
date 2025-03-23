use super::ParticipantAuthResponse;
use crate::common::{ApiError, middleware::auth::AuthEntity, validate};
use actix_web::{
    HttpResponse, delete, get, patch, post, put,
    web::{Data, Json, ReqData},
};
use actix_web_lab::extract::Path;
use dto::{
    auth::{LoginRequest, PasswordChangeRequest},
    participant::{CreateParticipant, Participant, ParticipantUpdate},
};
use macros::handler_implementation;
use service::participant::ParticipantServiceDependency;
use ulid::Ulid;
use utoipa::path as openapi;

handler_implementation! {
    ParticipantHandler as Implemented {
        #[openapi(
            tag = "Participants",
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
                (status = 400, description = "", body = ApiError),
            ),
        )]
        #[post("/register")]
        ///
        ///
        ///
        register(
            participant_service: Data<ParticipantServiceDependency>,
            Json(body): Json<CreateParticipant>
        ) -> HttpResponse {
            validate(&body)?;
            let resp: ParticipantAuthResponse = participant_service
                .register(body)
                .await?
                .into();
            HttpResponse::Created().json(resp)
        }

        #[openapi(
            tag = "Participants",
            operation_id = "participant_login",
            request_body(
                description = "",
                content = LoginRequest
            ),
            responses(
                (status = 200, description = "", body = ParticipantAuthResponse),
                (status = 401, description = "", body = ApiError),
                (status = 400, description = "", body = ApiError),
            ),
        )]
        #[post("/login")]
        ///
        ///
        ///
        login(
            participant_service: Data<ParticipantServiceDependency>,
            Json(body): Json<LoginRequest>,
        ) -> Json<ParticipantAuthResponse> {
            validate(&body)?;
            let res = participant_service
                .login(body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Participants",
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
        #[get("")]
        ///
        ///
        ///
        get_current(
            entity: ReqData<AuthEntity>,
        ) -> Json<Participant> {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;
            Json(participant.into())
        }

        #[openapi(
            tag = "Participants",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("")]
        ///
        ///
        ///
        update_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
            Json(body): Json<ParticipantUpdate>,
        ) -> Json<Participant> {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;
            validate(&body)?;


            if body.name.as_ref().unwrap_or(&participant.name) == &participant.name
                && body.surname.as_ref().unwrap_or(&participant.surname) == &participant.surname
                && body.bio.as_ref().unwrap_or(&participant.bio) == &participant.bio
                && body.portfolio_urls.as_ref().unwrap_or(&participant.portfolio_urls) == &participant.portfolio_urls
            {
                return Ok(Json(participant.into()));
            }
            let res = participant_service
                .update_by_id(participant.id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Participants",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/password")]
        ///
        ///
        ///
        change_password_current(
            participant_service: Data<ParticipantServiceDependency>,
            entity: ReqData<AuthEntity>,
            Json(body): Json<PasswordChangeRequest>,
        ) -> Json<ParticipantAuthResponse> {
            let participant: Participant = entity
                .into_inner()
                .try_into()?;
            validate(&body)?;
            let res = participant_service
                .change_password_by_id(participant.id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Participants",
            operation_id = "delete_current_participant",
            security(
                ("participant" = []),
            ),
            responses(
                (status = 204, description = ""),
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[delete("")]
        ///
        ///
        ///
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

        #[openapi(
            tag = "Participants",
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
        #[get("")]
        ///
        ///
        ///
        get_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
        ) -> Json<Participant> {
            let res = participant_service
                .get_by_id(participant_id)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Participants",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[patch("")]
        ///
        ///
        ///
        update_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
            Json(body): Json<ParticipantUpdate>,
        ) -> Json<Participant> {
            validate(&body)?;
            let res = participant_service
                .update_by_id(participant_id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Participants",
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
                (status = 400, description = "", body = ApiError),
                (status = 403, description = "", body = ApiError),
                (status = 401, description = "", body = ApiError),
            ),
        )]
        #[put("/password")]
        ///
        ///
        ///
        change_password_by_id(
            participant_service: Data<ParticipantServiceDependency>,
            Path(participant_id): Path<Ulid>,
            Json(body): Json<PasswordChangeRequest>,
        ) -> Json<ParticipantAuthResponse> {
            validate(&body)?;
            let res = participant_service
                .change_password_by_id(participant_id, body)
                .await?;
            Json(res.into())
        }

        #[openapi(
            tag = "Participants",
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
        #[delete("")]
        ///
        ///
        ///
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
