use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProwlError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("XML parsing failed: {0}")]
    XmlParse(#[from] quick_xml::DeError),

    #[error("Config file error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("API error ({code}): {message}")]
    Api { code: i32, message: String },

    #[error("No API key provided. Set via --api-key, PROWL_API_KEY env var, or config file")]
    MissingApiKey,

    #[error("No provider key provided. Set via --provider-key or PROWL_PROVIDER_KEY env var")]
    MissingProviderKey,

    #[error("Invalid priority: must be between -2 and 2")]
    InvalidPriority,

    #[error("Message too long: {length} bytes (max {max})")]
    MessageTooLong { length: usize, max: usize },

    #[error("Token not yet approved")]
    TokenNotApproved,
}

impl ProwlError {
    pub fn from_api_code(code: i32, message: Option<String>) -> Self {
        let message = message.unwrap_or_else(|| match code {
            400 => "Bad request - invalid parameters".to_string(),
            401 => "Unauthorized - invalid API key".to_string(),
            406 => "Not acceptable - rate limit exceeded".to_string(),
            409 => "Not approved - token has not been approved yet".to_string(),
            500 => "Internal server error".to_string(),
            _ => format!("Unknown error code: {code}"),
        });
        ProwlError::Api { code, message }
    }

    pub fn exit_code(&self) -> i32 {
        match self {
            ProwlError::Api { code, .. } => match code {
                401 => 2,
                406 => 3,
                409 => 4,
                _ => 1,
            },
            ProwlError::MissingApiKey | ProwlError::MissingProviderKey => 2,
            ProwlError::TokenNotApproved => 4,
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, ProwlError>;
