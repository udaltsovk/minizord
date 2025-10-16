use std::fmt::Debug;

use domain::session::{Session, SessionTokenPair};
use lib::domain::Id;

pub trait TokenService {
    type AdapterError: Debug + Send + Sync;

    fn generate_pair(
        &self,
        session_id: Id<Session>,
    ) -> Result<SessionTokenPair, Self::AdapterError>;

    fn parse(&self, token: &str) -> Result<Id<Session>, Self::AdapterError>;
}
