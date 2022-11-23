use librustymastodon::{FishData, InputData, RustyMastodonError};
use std::{env, error::Error};
use {
    reqwest::header,
    vercel_lambda::{error::VercelError, lambda, IntoResponse, Request},
};

#[allow(dead_code)]
fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let token = format!("Bearer {}", env::var("INSTANCES_TOKEN").unwrap());
    let mut bearer =
        header::HeaderValue::from_str(&token).map_err(|_| RustyMastodonError::ExternalRequest)?;
    bearer.set_sensitive(true);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, bearer);
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .user_agent("Rusty Mastodon")
        .build()
        .map_err(|_| RustyMastodonError::ExternalRequest)?;

    let resp = client
        .get("https://instances.social/api/1.0/instances/list")
        .send()
        .map_err(|_| RustyMastodonError::ExternalRequest)?
        .text()
        .map_err(|_| RustyMastodonError::ExternalRequest)?;
    eprintln!("Data read: {}", resp);
    let data = InputData {
        legend: None,
        school: vec![FishData::default()],
    };
    librustymastodon::build_response(data)
}

// Start the runtime with the handler
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
