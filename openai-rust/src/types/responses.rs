use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request to create a response
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponseRequest {
    /// The model to use
    pub model: String,
    /// Input to the model
    pub input: ResponseInput,
    /// System instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    /// Previous response ID for conversation continuity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
}

/// Input format for responses API
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseInput {
    /// Simple text input
    Text(String),
    /// Array of input items
    Items(Vec<InputItem>),
}

/// Input item types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputItem {
    /// Message input
    #[serde(rename = "message")]
    Message {
        /// Role of the message
        role: String,
        /// Message content
        content: MessageContent,
    },
    /// Reference to previous item
    #[serde(rename = "item_reference")]
    ItemReference {
        /// ID of the referenced item
        id: String,
    },
}

/// Message content format
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content
    Text(String),
    /// Multi-part content
    Parts(Vec<ContentPart>),
}

/// Content part types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
    },
    /// Image content
    #[serde(rename = "image")]
    Image {
        /// Image data
        image: ImageContent,
    },
}

/// Image content structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageContent {
    /// URL of the image
    pub url: String,
}

/// Response object for the Responses API
#[derive(Debug, Deserialize)]
pub struct Response {
    /// The response ID
    pub id: String,
    /// The object type, always "response"
    pub object: String,
    /// Unix timestamp of when the response was created
    pub created_at: u64,
    /// Response status
    pub status: String,
    /// Whether response is in background
    pub background: bool,
    /// Billing information
    pub billing: Option<Value>,
    /// Any error that occurred during processing
    pub error: Option<ResponseError>,
    /// Details about incomplete responses
    pub incomplete_details: Option<Value>,
    /// Instructions provided to the model
    pub instructions: Option<String>,
    /// Maximum output tokens
    pub max_output_tokens: Option<i32>,
    /// Maximum tool calls
    pub max_tool_calls: Option<i32>,
    /// Metadata associated with the response
    pub metadata: Option<Value>,
    /// The model used for this response
    pub model: String,
    /// The response output (array of output items)
    pub output: Vec<OutputItem>,
    /// Whether parallel tool calls are enabled
    pub parallel_tool_calls: Option<bool>,
    /// Previous response ID
    pub previous_response_id: Option<String>,
    /// Prompt cache key
    pub prompt_cache_key: Option<String>,
    /// Reasoning information
    pub reasoning: Option<Value>,
    /// Safety identifier
    pub safety_identifier: Option<String>,
    /// Service tier
    pub service_tier: Option<String>,
    /// Store flag
    pub store: Option<bool>,
    /// Temperature setting
    pub temperature: Option<f32>,
    /// Text format settings
    pub text: Option<Value>,
    /// Tool choice setting
    pub tool_choice: Option<Value>,
    /// Available tools
    pub tools: Option<Vec<Value>>,
    /// Top logprobs setting
    pub top_logprobs: Option<i32>,
    /// Top P setting
    pub top_p: Option<f32>,
    /// Truncation setting
    pub truncation: Option<Value>,
    /// Usage statistics
    pub usage: Option<ResponseUsage>,
    /// User identifier
    pub user: Option<String>,
}

/// Alias for backward compatibility
pub type CreateResponseResponse = Response;

/// Error information for failed responses
#[derive(Debug, Deserialize)]
pub struct ResponseError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
}

/// Alias for backward compatibility
pub type ErrorDetails = ResponseError;

/// Output item in the response
#[derive(Debug, Deserialize)]
pub struct OutputItem {
    /// Item ID
    pub id: String,
    /// Type of output item
    #[serde(rename = "type")]
    pub item_type: String,
    /// Status of the item
    pub status: String,
    /// Content array for message items
    pub content: Vec<OutputContent>,
    /// Role for message items
    pub role: Option<String>,
}

/// Output content
#[derive(Debug, Deserialize)]
pub struct OutputContent {
    /// Type of content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: Option<String>,
    /// Annotations
    pub annotations: Option<Vec<Value>>,
    /// Logprobs
    pub logprobs: Option<Vec<Value>>,
}

/// Alias for backward compatibility
pub type Content = OutputContent;

/// Usage statistics for Responses API
#[derive(Debug, Deserialize)]
pub struct ResponseUsage {
    /// Input tokens used
    pub input_tokens: i32,
    /// Input tokens details
    pub input_tokens_details: Option<Value>,
    /// Output tokens generated
    pub output_tokens: i32,
    /// Output tokens details
    pub output_tokens_details: Option<Value>,
    /// Total tokens
    pub total_tokens: i32,
}

/// Alias for backward compatibility
pub type Usage = ResponseUsage;
pub type UsageStats = ResponseUsage;

/// Response output (for backward compatibility)
pub type ResponseOutput = Vec<OutputItem>;
