use url::Url;
use validator::ValidationError;

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate_portfolio_url(
    portfolio_url: &str,
) -> Result<(), ValidationError> {
    let is_valid = if let Ok(parsed_url) = Url::parse(portfolio_url) {
        parsed_url.scheme() == "https"
            && parsed_url.has_host()
            && parsed_url.username() == ""
            && parsed_url.password().is_none()
    } else {
        false
    };

    if !is_valid {
        return Err(ValidationError::new("Invalid portfolio url"));
    }

    Ok(())
}

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate_portfolio_urls(
    portfolio_urls: &[String],
) -> Result<(), ValidationError> {
    portfolio_urls
        .iter()
        .try_for_each(|url| validate_portfolio_url(url))?;
    Ok(())
}
