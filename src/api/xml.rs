use quick_xml::de::from_str;
use serde::Deserialize;

use crate::api::types::ApiResponse;
use crate::error::{ProwlError, Result};

#[derive(Debug, Deserialize)]
struct ProwlResponse {
    #[serde(rename = "$value")]
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ResponseContent {
    Success(SuccessElement),
    Error(ErrorElement),
}

#[derive(Debug, Deserialize)]
struct SuccessElement {
    #[serde(rename = "@code")]
    code: i32,
    #[serde(rename = "@remaining")]
    remaining: Option<i32>,
    #[serde(rename = "@resetdate")]
    resetdate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ErrorElement {
    #[serde(rename = "@code")]
    code: i32,
    #[serde(rename = "$text")]
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RetrieveTokenResponse {
    #[serde(rename = "$value")]
    content: RetrieveTokenContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RetrieveTokenContent {
    Success(SuccessElement),
    Retrieve(RetrieveElement),
    Error(ErrorElement),
}

#[derive(Debug, Deserialize)]
struct RetrieveElement {
    #[serde(rename = "@token")]
    token: Option<String>,
    #[serde(rename = "@url")]
    url: Option<String>,
    #[serde(rename = "@apikey")]
    apikey: Option<String>,
}

pub fn parse_response(xml: &str) -> Result<ApiResponse> {
    let response: ProwlResponse = from_str(xml)?;

    match response.content {
        ResponseContent::Success(s) => Ok(ApiResponse::success(s.code, s.remaining, s.resetdate)),
        ResponseContent::Error(e) => Err(ProwlError::from_api_code(e.code, e.message)),
    }
}

pub fn parse_token_response(xml: &str) -> Result<ApiResponse> {
    let response: RetrieveTokenResponse = from_str(xml)?;

    match response.content {
        RetrieveTokenContent::Success(s) => {
            Ok(ApiResponse::success(s.code, s.remaining, s.resetdate))
        }
        RetrieveTokenContent::Retrieve(r) => {
            if let (Some(token), Some(url)) = (r.token, r.url) {
                Ok(ApiResponse::success(200, None, None).with_token(token, url))
            } else if let Some(apikey) = r.apikey {
                Ok(ApiResponse::success(200, None, None).with_apikey(apikey))
            } else {
                Err(ProwlError::Api {
                    code: 500,
                    message: "Invalid retrieve response".to_string(),
                })
            }
        }
        RetrieveTokenContent::Error(e) => {
            if e.code == 409 {
                Err(ProwlError::TokenNotApproved)
            } else {
                Err(ProwlError::from_api_code(e.code, e.message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_success() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<prowl>
    <success code="200" remaining="999" resetdate="1234567890"/>
</prowl>"#;
        let response = parse_response(xml).unwrap();
        assert!(response.success);
        assert_eq!(response.code, 200);
        assert_eq!(response.remaining, Some(999));
    }

    #[test]
    fn test_parse_error() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<prowl>
    <error code="401">Invalid API key</error>
</prowl>"#;
        let err = parse_response(xml).unwrap_err();
        match err {
            ProwlError::Api { code, message } => {
                assert_eq!(code, 401);
                assert!(message.contains("Invalid API key"));
            }
            _ => panic!("Expected Api error"),
        }
    }

    #[test]
    fn test_parse_token_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<prowl>
    <retrieve token="abc123" url="https://www.prowlapp.com/retrieve.php?token=abc123"/>
</prowl>"#;
        let response = parse_token_response(xml).unwrap();
        assert_eq!(response.token, Some("abc123".to_string()));
        assert!(response.token_url.is_some());
    }

    #[test]
    fn test_parse_apikey_response() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<prowl>
    <retrieve apikey="xyz789"/>
</prowl>"#;
        let response = parse_token_response(xml).unwrap();
        assert_eq!(response.apikey, Some("xyz789".to_string()));
    }
}
