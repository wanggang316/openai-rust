use crate::completion::Completion;
use crate::model::Model;
use crate::response::Response;
use reqwest::Client as ReqwestClient;
use thiserror::Error;

const API_BASE: &str = "https://api.openai.com/v1";

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("OpenAI API error: {0}")]
    ApiError(String),

    #[error("Failed to parse JSON: {0}")]
    JsonParser(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    base_url: String,
    http_client: ReqwestClient,
}

#[derive(Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    http_client: Option<ReqwestClient>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn base_url<S: Into<String>>(mut self, base_url: S) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn http_client(mut self, http_client: ReqwestClient) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn build(self) -> std::result::Result<Client, String> {
        let api_key = self.api_key.ok_or("API key is required")?;
        let base_url = self.base_url.unwrap_or_else(|| API_BASE.to_string());
        let http_client = self.http_client.unwrap_or_else(|| ReqwestClient::new());

        Ok(Client {
            api_key,
            base_url,
            http_client,
        })
    }
}

impl Client {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            http_client: ReqwestClient::new(),
        }
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    // Getter methods for managers to access private fields
    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }

    pub(crate) fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn http_client(&self) -> &ReqwestClient {
        &self.http_client
    }

    // Manager accessor methods
    pub fn completions(&self) -> Completion {
        Completion::new(self)
    }

    pub fn models(&self) -> Model {
        Model::new(self)
    }

    pub fn responses(&self) -> Response {
        Response::new(self)
    }
}
