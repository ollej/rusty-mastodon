use librustymastodon::{build_school, get_instances};
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request};

#[allow(dead_code)]
fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let data = get_instances().map(build_school);
    match data {
        Ok(data) => librustymastodon::build_response(data),
        Err(err) => librustymastodon::build_error_response(err.to_string()),
    }
}

// Start the runtime with the handler
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
