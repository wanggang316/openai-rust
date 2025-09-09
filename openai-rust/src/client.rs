use crate::models::{ChatCompletionRequest, ChatCompletionResponse};
use reqwest::Client as ReqwestClient;
use thiserror::Error;

const API_BASE: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("OpenAI API error: {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    http_client: ReqwestClient,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: ReqwestClient::new(),
        }
    }

    pub async fn chat_completion(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{API_BASE}/chat/completions");
        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
            return Err(Error::ApiError(format!("API request failed: {error_text}")));
        }

        // println!("{}", response.text().await?);
        let chat_response = response.json::<ChatCompletionResponse>().await?;
        Ok(chat_response)
        // let r = ChatCompletionResponse {
        //     id: "chatcmpl-123".to_string(),
        //     object: "chat.completion".to_string(),
        //     created: 1677652288,
        //     model: "z-ai/glm-4.5-air:free".to_string(),
        //     choices: vec![],
        //     usage: crate::models::Usage {
        //         prompt_token: 13,
        //         completion_tokens: 7,
        //         total_tokens: 20,
        //     },
        // };
        // Ok(r)
    }
}
