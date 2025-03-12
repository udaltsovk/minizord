use macros::implementation;
use service::organizator::OrganizatorService;
use utoipa_actix_web::service_config::ServiceConfig;

implementation! {
    OrganizatorHandler {
        organizator_service: dyn OrganizatorService
    } as Implementation {
        config(_cfg: &mut ServiceConfig) {

        }
    }
}
