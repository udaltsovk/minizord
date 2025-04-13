use passwords::{analyzer::analyze, scorer::score};

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate_password(password: &str, _: &()) -> garde::Result {
    score(&analyze(password))
        .ge(&57.0)
        .then_some(())
        .ok_or(garde::Error::new("password is too weak"))
}
