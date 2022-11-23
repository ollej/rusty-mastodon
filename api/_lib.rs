use std::env;
use {
    http::StatusCode,
    nanoserde::SerJson,
    vercel_lambda::{error::VercelError, Body, Response},
};

#[derive(Clone, Default, SerJson)]
pub struct InputData {
    pub legend: Option<Legend>,
    pub school: Vec<FishData>,
}

#[derive(Clone, Debug, SerJson)]
pub struct Legend {
    pub description: String,
    pub fish_legends: Vec<FishLegend>,
}

#[derive(Clone, Debug, SerJson)]
pub struct FishLegend {
    pub fish: String,
    pub description: String,
}

#[derive(Clone, SerJson)]
#[nserde(default)]
pub struct FishData {
    pub fish: String,
    pub size: f32,
    pub speed: f32,
    pub bubbles: f32,
}

impl Default for FishData {
    fn default() -> FishData {
        FishData {
            fish: "clownfish".to_string(),
            size: 1.0,
            speed: 1.0,
            bubbles: 1.0,
        }
    }
}

/// Build a Vercel Response from a serializeable body
///
/// # Arguments
///
/// * `body` - Serialize into a JSON body
///
/// # Examples
/// ```
/// use {libquiz::{build_response}, vercel_lambda::Body};
/// let response = build_response("body string").unwrap();
/// assert_eq!(&Body::Text("\"body string\"".to_string()), response.body());
/// assert_eq!("application/json", response.headers().get(http::header::CONTENT_TYPE).unwrap());
/// ```
pub fn build_response<S>(body: S) -> Result<Response<Body>, VercelError>
where
    S: SerJson,
{
    let body = Body::Text(SerJson::serialize_json(&body));
    Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(
            http::header::CACHE_CONTROL,
            format!(
                "max-age={}, public",
                env::var("HTTP_CACHE_IN_SECONDS").unwrap_or("15".to_string())
            ),
        )
        .body(body)
        .map_err(|_| VercelError::new("Couldn't build response"))
}
