use std::{env, fmt};
use {
    http::StatusCode,
    nanoserde::{DeJson, DeJsonErr, SerJson},
    reqwest::header,
    vercel_lambda::{error::VercelError, Body, Response},
};

#[derive(Debug, PartialEq)]
pub enum RustyMastodonError {
    ExternalRequest,
    Configuration,
    Parsing,
}

impl fmt::Display for RustyMastodonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RustyMastodonError::ExternalRequest => {
                write!(f, "Failed reading data from external source")
            }
            RustyMastodonError::Parsing => {
                write!(f, "Failed parsing data from external source")
            }
            RustyMastodonError::Configuration => {
                write!(f, "Configuration error")
            }
        }
    }
}

impl From<reqwest::Error> for RustyMastodonError {
    fn from(_: reqwest::Error) -> Self {
        RustyMastodonError::ExternalRequest
    }
}

impl From<DeJsonErr> for RustyMastodonError {
    fn from(_: DeJsonErr) -> Self {
        RustyMastodonError::Parsing
    }
}

impl From<reqwest::header::InvalidHeaderValue> for RustyMastodonError {
    fn from(_: reqwest::header::InvalidHeaderValue) -> Self {
        RustyMastodonError::Configuration
    }
}

impl From<env::VarError> for RustyMastodonError {
    fn from(_: env::VarError) -> Self {
        RustyMastodonError::Configuration
    }
}

impl std::error::Error for RustyMastodonError {}

impl From<RustyMastodonError> for VercelError {
    fn from(error: RustyMastodonError) -> Self {
        VercelError::new(&format!("{}", error))
    }
}

#[derive(Clone, Debug, Default, SerJson)]
struct ErrorResponse {
    message: String,
}

#[derive(Clone, Debug, Default, SerJson)]
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

#[derive(Clone, Debug, SerJson)]
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

#[derive(Default, Debug, Clone, PartialEq, DeJson)]
pub struct Root {
    pub instances: Vec<Instance>,
    pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, DeJson)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub added_at: Option<String>,
    pub updated_at: String,
    pub checked_at: String,
    pub uptime: i64,
    pub up: bool,
    pub dead: bool,
    pub version: Option<String>,
    pub ipv6: bool,
    pub https_score: Option<i64>,
    pub https_rank: Option<String>,
    pub obs_score: Option<i64>,
    pub obs_rank: Option<String>,
    pub users: String,
    pub statuses: String,
    pub connections: String,
    pub open_registrations: bool,
    pub info: Option<Info>,
    pub thumbnail: Option<String>,
    pub thumbnail_proxy: Option<String>,
    pub active_users: Option<i64>,
    pub email: Option<String>,
    pub admin: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, DeJson)]
pub struct Info {
    pub short_description: String,
    pub full_description: String,
    pub topic: Option<String>,
    pub languages: Vec<String>,
    pub other_languages_accepted: bool,
    pub federates_with: Option<String>,
    pub prohibited_content: Vec<String>,
    pub categories: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, DeJson)]
pub struct Pagination {
    pub total: i64,
    pub next_id: String,
}

pub fn get_instances() -> Result<Vec<Instance>, RustyMastodonError> {
    let content = reqwest::blocking::Client::builder()
        .user_agent(env::var("INSTANCES_API_USER_AGENT")?)
        .build()?
        .get(env::var("INSTANCES_API_URL")?)
        .bearer_auth(env::var("INSTANCES_API_TOKEN")?)
        .send()?
        .text()?;
    let json: Root = DeJson::deserialize_json(&content)?;
    Ok(json.instances)
}

pub fn build_school(instances: Vec<Instance>) -> Vec<FishData> {
    instances
        .iter()
        .map(|instance| {
            let bubbles = if instance.open_registrations {
                1.0
            } else {
                0.0
            };
            let fish = match instance {
                Instance { ref users, .. } if users == "1" => "neontetra",
                Instance { dead: true, .. } => "turtle",
                _ => "clownfish",
            }
            .to_string();
            FishData {
                fish,
                size: 1.0,
                speed: 1.0,
                bubbles,
            }
        })
        .collect()
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
                env::var("HTTP_CACHE_IN_SECONDS").unwrap_or("60".to_string())
            ),
        )
        .body(body)
        .map_err(|_| VercelError::new("Couldn't build response"))
}

pub fn build_error_response<S>(error_message: S) -> Result<Response<Body>, VercelError>
where
    S: Into<String>,
{
    let error = ErrorResponse {
        message: error_message.into(),
    };
    let body = Body::Text(SerJson::serialize_json(&error));
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(body)
        .map_err(|_| VercelError::new("Couldn't build response"))
}
