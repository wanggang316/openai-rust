use serde::Deserialize;

/// Response from the models list API
#[derive(Debug, Deserialize)]
pub struct ModelsResponse {
    /// The object type, always "list"
    pub object: String,
    /// Array of available models
    pub data: Vec<Model>,
}

/// Individual model information
#[derive(Debug, Deserialize)]
pub struct Model {
    /// The model identifier (e.g., "gpt-4", "text-embedding-3-large")
    pub id: String,
    /// The object type, always "model"
    pub object: String,
    /// Unix timestamp of when the model was created
    pub created: u64,
    /// The organization that owns the model (typically "system" for OpenAI models)
    pub owned_by: String,
}