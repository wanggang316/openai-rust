use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponseRequest {
    pub model: String,
    pub input: ResponseInput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseInput {
    /// Simple text input as string
    Text(String),
    /// Array of input items
    Items(Vec<InputItem>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputItem {
    #[serde(rename = "message")]
    Message {
        role: String,
        content: MessageContent,
    },
    #[serde(rename = "item_reference")]
    ItemReference { id: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { image: ImageContent },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageContent {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateResponseResponse {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub model: String,
    pub error: Option<ErrorDetails>,
    pub output: Option<OutputContent>,
    pub usage: Option<UsageStats>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct OutputContent {
    #[serde(rename = "type")]
    pub output_type: String,
    pub content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsageStats {
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub total_tokens: i32,
}
