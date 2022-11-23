use librustymastodon::{build_school, get_instances, FishLegend, InputData, Legend};
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request};

#[allow(dead_code)]
fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let data = get_instances().map(build_school);
    let legend = Legend {
        description: "Top 100 Mastodon instances\nFishes without bubbles don't accept new signups."
            .to_string(),
        fish_legends: vec![
            FishLegend {
                fish: "clownfish".to_string(),
                description: "A running Mastodon instance".to_string(),
            },
            FishLegend {
                fish: "seahorse".to_string(),
                description: "A Mastodon instance that's currently down".to_string(),
            },
        ],
    };
    match data {
        Ok(school) => librustymastodon::build_response(InputData {
            legend: Some(legend),
            school,
        }),
        Err(err) => librustymastodon::build_error_response(err.to_string()),
    }
}

// Start the runtime with the handler
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
