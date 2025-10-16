use application::{
    repository::RepositoriesModuleExt,
    service::ServicesModuleExt,
    usecase::{session::SessionUseCase, user::UserUseCase},
};

pub trait ModulesExt: Clone + Send + Sync + 'static {
    type RepositoriesModule: RepositoriesModuleExt;
    type ServicesModule: ServicesModuleExt;

    fn base_api_url(&self) -> &str;

    fn user_usecase(
        &self,
    ) -> &impl UserUseCase<Self::RepositoriesModule, Self::ServicesModule>;

    fn session_usecase(
        &self,
    ) -> &impl SessionUseCase<Self::RepositoriesModule, Self::ServicesModule>;
}
