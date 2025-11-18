use lib::DomainType;

#[derive(DomainType)]
pub struct SessionRefreshToken(String);

impl From<String> for SessionRefreshToken {
    fn from(token: String) -> Self {
        Self(token)
    }
}
