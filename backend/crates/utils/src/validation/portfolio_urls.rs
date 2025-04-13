use url::Url;

#[tracing::instrument(skip_all, level = "debug")]
pub fn validate_portfolio_url(portfolio_url: &str, _: &()) -> garde::Result {
    let is_valid = Url::parse(portfolio_url).is_ok_and(|parsed_url| {
        parsed_url.scheme() == "https"
            && parsed_url.has_host()
            && parsed_url.username() == ""
            && parsed_url.password().is_none()
    });

    if !is_valid {
        return Err(garde::Error::new("invalid portfolio url"));
    }

    Ok(())
}
