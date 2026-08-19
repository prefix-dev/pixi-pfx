use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum PfxError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("GraphQL error: {message}")]
    Graphql {
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("Authentication required")]
    #[allow(dead_code)]
    AuthRequired,

    #[error("Authentication storage error: {0}")]
    AuthStorage(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl PfxError {
    pub fn code(&self) -> &'static str {
        match self {
            PfxError::Http(_) => "HTTP_ERROR",
            PfxError::Graphql { .. } => "GRAPHQL_ERROR",
            PfxError::AuthRequired => "AUTH_REQUIRED",
            PfxError::AuthStorage(_) => "AUTH_STORAGE_ERROR",
            PfxError::InvalidArgument(_) => "INVALID_ARGUMENT",
            PfxError::Json(_) => "JSON_ERROR",
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl From<&PfxError> for ErrorResponse {
    fn from(err: &PfxError) -> Self {
        let details = match err {
            PfxError::Graphql { details, .. } => details.clone(),
            _ => None,
        };
        ErrorResponse {
            code: err.code().to_string(),
            message: err.to_string(),
            details,
        }
    }
}
