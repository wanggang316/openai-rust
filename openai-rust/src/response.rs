use crate::client::{Client, Error, Result};
use crate::types::{
    CreateResponseRequest, Response as ResponseData, ResponseChunk, ResponseStreamEvent,
};
use async_stream::stream;
use futures_util::{Stream, StreamExt};

pub struct Response<'a> {
    client: &'a Client,
}

impl<'a> Response<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, request: &CreateResponseRequest) -> Result<ResponseData> {
        let url = format!("{}/responses", self.client.base_url());
        let response = self
            .client
            .http_client()
            .post(&url)
            .bearer_auth(self.client.api_key())
            .json(request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
            return Err(Error::ApiError(format!("API request failed: {error_text}")));
        }

        let response_data = response.json::<ResponseData>().await?;
        Ok(response_data)
    }

    pub async fn get(&self, response_id: &str) -> Result<ResponseData> {
        let url = format!("{}/responses/{}", self.client.base_url(), response_id);
        let response = self
            .client
            .http_client()
            .get(&url)
            .bearer_auth(self.client.api_key())
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read error body".to_string());
            return Err(Error::ApiError(format!("API request failed: {error_text}")));
        }

        let response_data = response.json::<ResponseData>().await?;
        Ok(response_data)
    }

    pub async fn create_stream(
        &self,
        request: &CreateResponseRequest,
    ) -> Result<impl Stream<Item = Result<ResponseChunk>>> {
        let url = format!("{}/responses", self.client.base_url());

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
                        match serde_json::from_str::<ResponseStreamEvent>(data) {
                            Ok(event) => yield Ok(ResponseChunk { event }),
                            Err(e) => yield Err(Error::from(e)),
                        }
                    }
                }
            }
        };

        Ok(s)
    }
}
