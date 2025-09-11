use crate::client::{Client, Error, Result};
use crate::types::{ChatCompletionChunkResponse, ChatCompletionRequest, ChatCompletionResponse};
use async_stream::stream;
use futures_util::{Stream, StreamExt};

pub struct Completion<'a> {
    client: &'a Client,
}

impl<'a> Completion<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = format!("{}/chat/completions", self.client.base_url());
        let response = self
            .client
            .http_client()
            .post(&url)
            .bearer_auth(self.client.api_key())
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

    pub async fn create_stream(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<impl Stream<Item = Result<ChatCompletionChunkResponse>>> {
        let url = format!("{}/chat/completions", self.client.base_url());

        let response = self
            .client
            .http_client()
            .post(&url)
            .bearer_auth(self.client.api_key())
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

                while let Some(newline_pos) = buffer.find('\n') {
                    let line: String = buffer.drain(..=newline_pos).collect();
                    if line.trim().starts_with("data: ") {
                        let data = &line.trim()[6..];
                        if data == "[DONE]" {
                            break;
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