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
    /// If set, the API will stream partial message deltas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Input format for responses API
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ResponseInput {
    /// Simple text input
    Text(String),
    /// Array of input items
    Items(Vec<InputItem>),
}

/// Input item types
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content
    Text(String),
    /// Multi-part content
    Parts(Vec<ContentPart>),
}

/// Content part types
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageContent {
    /// URL of the image
    pub url: String,
}

/// Response object for the Responses API
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    /// The response ID
    pub id: String,
    /// The object type, always "response"
    pub object: String,
    /// Unix timestamp of when the response was created
    pub created_at: u64,
    /// Response status
    #[serde(default)]
    pub status: String,
    /// Whether response is in background
    #[serde(default)]
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
    #[serde(default)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct ResponseError {
    /// Error code
    #[serde(default)]
    pub code: String,
    /// Error message
    #[serde(default)]
    pub message: String,
}

/// Alias for backward compatibility
pub type ErrorDetails = ResponseError;

/// Output item in the response
#[derive(Debug, Deserialize, Clone)]
pub struct OutputItem {
    /// Item ID
    pub id: String,
    /// Type of output item
    #[serde(rename = "type")]
    pub item_type: String,
    /// Status of the item
    #[serde(default)]
    pub status: String,
    /// Content array for message items
    #[serde(default)]
    pub content: Vec<OutputContent>,
    /// Role for message items
    pub role: Option<String>,
}

/// Output content
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
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
pub type UsageStats = ResponseUsage;

/// Response output (for backward compatibility)
pub type ResponseOutput = Vec<OutputItem>;

// Streaming-related structs

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ResponseStreamEvent {
    #[serde(rename = "response.created")]
    ResponseCreated {
        sequence_number: u32,
        response: Response,
    },
    #[serde(rename = "response.in_progress")]
    ResponseInProgress {
        sequence_number: u32,
        response: Response,
    },
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        sequence_number: u32,
        output_index: u32,
        item: OutputItem,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        part: OutputContent,
    },
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        delta: String,
        logprobs: Vec<serde_json::Value>,
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        text: String,
        logprobs: Vec<serde_json::Value>,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        part: OutputContent,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        sequence_number: u32,
        output_index: u32,
        item: OutputItem,
    },
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        sequence_number: u32,
        response: Response,
    },
    #[serde(rename = "error")]
    Error { error: ResponseError },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseChunk {
    pub event: ResponseStreamEvent,
}
