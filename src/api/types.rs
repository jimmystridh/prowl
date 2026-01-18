use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SendRequest {
    pub apikey: String,
    pub application: String,
    pub event: String,
    pub description: String,
    pub priority: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providerkey: Option<String>,
}

impl SendRequest {
    pub const MAX_EVENT_LEN: usize = 1024;
    pub const MAX_DESCRIPTION_LEN: usize = 10000;
    pub const MAX_URL_LEN: usize = 512;
    pub const MAX_APPLICATION_LEN: usize = 256;

    pub fn validate(&self) -> crate::error::Result<()> {
        if self.event.len() > Self::MAX_EVENT_LEN {
            return Err(crate::error::ProwlError::MessageTooLong {
                length: self.event.len(),
                max: Self::MAX_EVENT_LEN,
            });
        }
        if self.description.len() > Self::MAX_DESCRIPTION_LEN {
            return Err(crate::error::ProwlError::MessageTooLong {
                length: self.description.len(),
                max: Self::MAX_DESCRIPTION_LEN,
            });
        }
        if let Some(ref url) = self.url
            && url.len() > Self::MAX_URL_LEN
        {
            return Err(crate::error::ProwlError::MessageTooLong {
                length: url.len(),
                max: Self::MAX_URL_LEN,
            });
        }
        if self.application.len() > Self::MAX_APPLICATION_LEN {
            return Err(crate::error::ProwlError::MessageTooLong {
                length: self.application.len(),
                max: Self::MAX_APPLICATION_LEN,
            });
        }
        if !(-2..=2).contains(&self.priority) {
            return Err(crate::error::ProwlError::InvalidPriority);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyRequest {
    pub apikey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub providerkey: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenRequest {
    pub providerkey: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterRequest {
    pub providerkey: String,
    pub token: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub code: i32,
    pub remaining: Option<i32>,
    pub reset_date: Option<String>,
    pub token: Option<String>,
    pub token_url: Option<String>,
    pub apikey: Option<String>,
    pub error_message: Option<String>,
}

impl ApiResponse {
    pub fn success(code: i32, remaining: Option<i32>, reset_date: Option<String>) -> Self {
        ApiResponse {
            success: true,
            code,
            remaining,
            reset_date,
            token: None,
            token_url: None,
            apikey: None,
            error_message: None,
        }
    }

    pub fn with_token(mut self, token: String, url: String) -> Self {
        self.token = Some(token);
        self.token_url = Some(url);
        self
    }

    pub fn with_apikey(mut self, apikey: String) -> Self {
        self.apikey = Some(apikey);
        self
    }
}
