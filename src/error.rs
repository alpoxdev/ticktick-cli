use thiserror::Error;

#[derive(Debug, Error)]
pub enum TickTickError {
    #[error("missing access token: pass --token or set TICKTICK_ACCESS_TOKEN")]
    MissingAccessToken,
    #[error("missing required OAuth client id: pass --client-id or set TICKTICK_CLIENT_ID")]
    MissingClientId,
    #[error(
        "missing required OAuth client secret: pass --client-secret or set TICKTICK_CLIENT_SECRET"
    )]
    MissingClientSecret,
    #[error("exactly one of --json, --json-file, or --json-stdin may be used")]
    ConflictingJsonInput,
    #[error("this command requires JSON input via --json, --json-file, or --json-stdin")]
    MissingJsonInput,
    #[error("failed to read JSON file {path}: {source}")]
    ReadJsonFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read JSON from stdin: {0}")]
    ReadStdin(std::io::Error),
    #[error("invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("api request failed with status {status}: {body}")]
    ApiStatus {
        status: reqwest::StatusCode,
        body: String,
    },
}

pub type Result<T> = std::result::Result<T, TickTickError>;
