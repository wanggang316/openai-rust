use serde::{Deserialize, Serialize};

/// Response object for the Responses API
#[derive(Debug, Deserialize)]
pub struct Response {
    /// The response ID
    pub id: String,
    /// The object type, always "response"
    pub object: String,
    /// Unix timestamp of when the response was created
    pub created_at: u64,
    /// Any error that occurred during processing
    pub error: Option<ResponseError>,
    /// Details about incomplete responses
    pub incomplete_details: Option<IncompleteDetails>,
    /// Instructions provided to the model
    pub instructions: Option<String>,
    /// Metadata associated with the response
    pub metadata: Option<serde_json::Value>,
    /// The model used for this response
    pub model: String,
    /// The response output
    pub output: Option<ResponseOutput>,
}

/// Error information for failed responses
#[derive(Debug, Deserialize)]
pub struct ResponseError {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
}

/// Details about incomplete responses
#[derive(Debug, Deserialize)]
pub struct IncompleteDetails {
    /// The reason the response was incomplete
    pub reason: String,
}

/// The output of a response
#[derive(Debug, Deserialize)]
pub struct ResponseOutput {
    /// The type of output
    #[serde(rename = "type")]
    pub output_type: String,
    /// The content of the output
    pub content: Vec<OutputContent>,
}

/// Individual piece of output content
#[derive(Debug, Deserialize)]
pub struct OutputContent {
    /// The type of content (e.g., "text")
    #[serde(rename = "type")]
    pub content_type: String,
    /// The actual text content
    pub text: Option<String>,
}

/// Request to create a response
#[derive(Debug, Serialize)]
pub struct CreateResponseRequest {
    /// The model to use
    pub model: String,
    /// The input messages or content
    pub input: Vec<ResponseInput>,
    /// Instructions for the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Metadata to associate with the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Previous response ID for context continuation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Tools available to the model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
}

/// Input content for a response request
#[derive(Debug, Serialize)]
pub struct ResponseInput {
    /// The type of input (e.g., "text", "image")
    #[serde(rename = "type")]
    pub input_type: String,
    /// Text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Image content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageInput>,
}

/// Image input content
#[derive(Debug, Serialize)]
pub struct ImageInput {
    /// The image data or URL
    pub url: String,
    /// Optional detail level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Tool definition for the Responses API
#[derive(Debug, Serialize)]
pub struct Tool {
    /// The type of tool
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function definition for function tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<ToolFunction>,
    /// Web search configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<serde_json::Value>,
    /// File search configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_search: Option<serde_json::Value>,
    /// Computer use configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_use: Option<serde_json::Value>,
}

/// Function tool definition
#[derive(Debug, Serialize)]
pub struct ToolFunction {
    /// The name of the function
    pub name: String,
    /// Description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameters schema for the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
