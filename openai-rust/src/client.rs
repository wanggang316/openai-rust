use crate::models::{ChatCompletionChunkResponse, ChatCompletionRequest, ChatCompletionResponse};
use async_stream::stream;
use futures_util::{Stream, StreamExt};
use reqwest::Client as ReqwestClient;
use thiserror::Error;

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

impl Client {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            http_client: ReqwestClient::new(),
        }
    }

    pub async fn chat_completion(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.base_url);
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

        let chat_response = response.json::<ChatCompletionResponse>().await?;
        Ok(chat_response)
    }

    /// 发送一个流式的聊天补全请求。
    pub async fn chat_completions_stream(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<impl Stream<Item = Result<ChatCompletionChunkResponse>>> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
            return Err(Error::ApiError(format!("API request failed: {error_body}")));
        }

        let mut byte_stream = response.bytes_stream();

        // 使用 async-stream 宏来创建一个实现了 Stream trait 的异步代码块
        let s = stream! {
            let mut buffer = String::new();
            while let Some(chunk_result) = byte_stream.next().await {
                let chunk = match chunk_result {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        yield Err(Error::from(e));
                        break;
                    }
                };

                buffer.push_str(&String::from_utf8_lossy(&chunk));

                // 通过换行符分割来处理 Server-Sent Events (SSE)
                while let Some(newline_pos) = buffer.find('\n') {
                    let line: String = buffer.drain(..=newline_pos).collect();
                    if line.trim().starts_with("data: ") {
                        let data = &line.trim()[6..];
                        if data == "[DONE]" {
                            break; // 流结束
                        }
                        match serde_json::from_str::<ChatCompletionChunkResponse>(data) {
                            Ok(chunk_response) => yield Ok(chunk_response),
                            Err(e) => yield Err(Error::from(e)),
                        }
                    }
                }
            }
        };

        Ok(s)
    }
}
