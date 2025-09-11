use crate::client::{Client, Error, Result};
use crate::types::{CreateResponseRequest, Response as ResponseData};

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
}