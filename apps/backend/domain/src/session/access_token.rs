use lib::DomainType;

#[derive(DomainType)]
pub struct SessionAccessToken(String);

impl From<String> for SessionAccessToken {
    fn from(token: String) -> Self {
        Self(token)
    }
}
