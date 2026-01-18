use reqwest::Client;

use crate::api::types::{ApiResponse, RegisterRequest, SendRequest, TokenRequest, VerifyRequest};
use crate::api::xml::{parse_response, parse_token_response};
use crate::error::Result;

const BASE_URL: &str = "https://api.prowlapp.com/publicapi";

pub struct ProwlClient {
    client: Client,
}

impl ProwlClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent(format!("prowl-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(ProwlClient { client })
    }

    pub async fn send(&self, request: &SendRequest) -> Result<ApiResponse> {
        request.validate()?;

        let mut form = vec![
            ("apikey", request.apikey.clone()),
            ("application", request.application.clone()),
            ("event", request.event.clone()),
            ("description", request.description.clone()),
            ("priority", request.priority.to_string()),
        ];

        if let Some(ref url) = request.url {
            form.push(("url", url.clone()));
        }

        if let Some(ref providerkey) = request.providerkey {
            form.push(("providerkey", providerkey.clone()));
        }

        let response = self
            .client
            .post(format!("{BASE_URL}/add"))
            .form(&form)
            .send()
            .await?;

        let body = response.text().await?;
        parse_response(&body)
    }

    pub async fn verify(&self, request: &VerifyRequest) -> Result<ApiResponse> {
        let mut url = format!("{BASE_URL}/verify?apikey={}", request.apikey);

        if let Some(ref providerkey) = request.providerkey {
            url.push_str(&format!("&providerkey={providerkey}"));
        }

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        parse_response(&body)
    }

    pub async fn retrieve_token(&self, request: &TokenRequest) -> Result<ApiResponse> {
        let url = format!(
            "{BASE_URL}/retrieve/token?providerkey={}",
            request.providerkey
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        parse_token_response(&body)
    }

    pub async fn retrieve_apikey(&self, request: &RegisterRequest) -> Result<ApiResponse> {
        let url = format!(
            "{BASE_URL}/retrieve/apikey?providerkey={}&token={}",
            request.providerkey, request.token
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        parse_token_response(&body)
    }
}

impl Default for ProwlClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
