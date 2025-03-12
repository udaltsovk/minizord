use macros::implementation;
use repository::organizator::OrganizatorRepository;

implementation! {
    OrganizatorService {
        organizator_repository: dyn OrganizatorRepository
    } as Implementation {

    }
}
