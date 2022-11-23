use librustymastodon::{FishData, InputData};
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request};

#[allow(dead_code)]
fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
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
