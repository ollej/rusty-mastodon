use librustymastodon::{get_instances, FishData, InputData};
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request};

#[allow(dead_code)]
fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let instances = get_instances();
    match instances {
        Ok(instances) => println!("Instances: {:?}", instances),
        Err(err) => eprintln!("Error: {}", err),
    }
    //eprintln!("Instances: {:?}", instances);
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
