use crate::client::{Client, Error, Result};
use crate::types::ModelsResponse;

pub struct Model<'a> {
    client: &'a Client,
}

impl<'a> Model<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<ModelsResponse> {
        let url = format!("{}/models", self.client.base_url());
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

        let models_response = response.json::<ModelsResponse>().await?;
        Ok(models_response)
    }
}