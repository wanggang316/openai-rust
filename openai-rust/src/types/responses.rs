use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateResponseRequest {
    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub model: String,
    /// Text, image, or file inputs to the model, used to generate a response.
    pub input: ResponseInput,
    /// A system (or developer) message inserted into the model's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// The unique ID of the previous response to the model for multi-turn conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// An array of tools the model may call while generating a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Specify additional output data to include in the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// Whether to allow the model to run tool calls in parallel. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Whether to store the generated model response for later retrieval via API. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// If set to true, the model response data will be streamed to the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Options for streaming response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ResponseStreamOptions>,
    /// The conversation that this response belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationParam>,
    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// An integer between 0 and 20 specifying the number of most likely tokens to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    /// A stable identifier for your end-users (deprecated, use prompt_cache_key).
    #[deprecated(
        note = "This field is being replaced by `safety_identifier` and `prompt_cache_key`. Use `prompt_cache_key` instead."
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// A stable identifier used to help detect users that may be violating usage policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// Used by OpenAI to cache responses for similar requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Specifies a latency tier to use for processing the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// Reasoning configuration for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
    /// Whether to run the model response in the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// An upper bound for the number of tokens that can be generated for a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i32>,
    /// The maximum number of total calls to built-in tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i32>,
    /// Configuration options for a text response from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextConfiguration>,
    /// How the model should select which tool (or tools) to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Prompt configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
    /// The truncation strategy to use for the model response (auto or disabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationStrategy>,
}

impl CreateResponseRequest {
    /// Create a builder for constructing a request
    pub fn builder() -> CreateResponseRequestBuilder {
        CreateResponseRequestBuilder::new()
    }
}

/// Builder for CreateResponseRequest
#[derive(Debug, Clone, Default)]
pub struct CreateResponseRequestBuilder {
    model: Option<String>,
    input: Option<ResponseInput>,
    instructions: Option<String>,
    metadata: Option<Metadata>,
    previous_response_id: Option<String>,
    tools: Option<Vec<Tool>>,
    include: Option<Vec<String>>,
    parallel_tool_calls: Option<bool>,
    store: Option<bool>,
    stream: Option<bool>,
    stream_options: Option<ResponseStreamOptions>,
    conversation: Option<ConversationParam>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_logprobs: Option<i32>,
    user: Option<String>,
    safety_identifier: Option<String>,
    prompt_cache_key: Option<String>,
    service_tier: Option<ServiceTier>,
    reasoning: Option<Reasoning>,
    background: Option<bool>,
    max_output_tokens: Option<i32>,
    max_tool_calls: Option<i32>,
    text: Option<TextConfiguration>,
    tool_choice: Option<ToolChoice>,
    prompt: Option<Prompt>,
    truncation: Option<TruncationStrategy>,
}

impl CreateResponseRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the model ID (required)
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the input (required)
    pub fn input(mut self, input: impl Into<ResponseInput>) -> Self {
        self.input = Some(input.into());
        self
    }

    /// Set system instructions
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Enable or disable streaming
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Set temperature (0.0 to 2.0)
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set tools available to the model
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set tool choice strategy
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Set metadata
    pub fn metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set previous response ID for multi-turn conversations
    pub fn previous_response_id(mut self, id: impl Into<String>) -> Self {
        self.previous_response_id = Some(id.into());
        self
    }

    /// Set maximum output tokens
    pub fn max_output_tokens(mut self, max_tokens: i32) -> Self {
        self.max_output_tokens = Some(max_tokens);
        self
    }

    /// Set reasoning configuration
    pub fn reasoning(mut self, reasoning: Reasoning) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    /// Set service tier
    pub fn service_tier(mut self, tier: ServiceTier) -> Self {
        self.service_tier = Some(tier);
        self
    }

    /// Enable or disable parallel tool calls
    pub fn parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.parallel_tool_calls = Some(enabled);
        self
    }

    /// Set top_p for nucleus sampling
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set conversation parameter
    pub fn conversation(mut self, conversation: ConversationParam) -> Self {
        self.conversation = Some(conversation);
        self
    }

    /// Enable or disable background processing
    pub fn background(mut self, background: bool) -> Self {
        self.background = Some(background);
        self
    }

    /// Set whether to store the response
    pub fn store(mut self, store: bool) -> Self {
        self.store = Some(store);
        self
    }

    /// Set additional output data to include
    pub fn include(mut self, include: Vec<String>) -> Self {
        self.include = Some(include);
        self
    }

    /// Set stream options
    pub fn stream_options(mut self, options: ResponseStreamOptions) -> Self {
        self.stream_options = Some(options);
        self
    }

    /// Set top logprobs
    pub fn top_logprobs(mut self, top_logprobs: i32) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    /// Set safety identifier
    pub fn safety_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.safety_identifier = Some(identifier.into());
        self
    }

    /// Set prompt cache key
    pub fn prompt_cache_key(mut self, key: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(key.into());
        self
    }

    /// Set maximum tool calls
    pub fn max_tool_calls(mut self, max_calls: i32) -> Self {
        self.max_tool_calls = Some(max_calls);
        self
    }

    /// Set text configuration
    pub fn text(mut self, text: TextConfiguration) -> Self {
        self.text = Some(text);
        self
    }

    /// Set prompt configuration
    pub fn prompt(mut self, prompt: Prompt) -> Self {
        self.prompt = Some(prompt);
        self
    }

    /// Set truncation strategy
    pub fn truncation(mut self, truncation: TruncationStrategy) -> Self {
        self.truncation = Some(truncation);
        self
    }

    /// Build the final request
    ///
    /// # Errors
    ///
    /// Returns an error if required fields (model, input) are not set
    pub fn build(self) -> Result<CreateResponseRequest, String> {
        let model = self.model.ok_or("model is required")?;
        let input = self.input.ok_or("input is required")?;

        Ok(CreateResponseRequest {
            model,
            input,
            instructions: self.instructions,
            metadata: self.metadata,
            previous_response_id: self.previous_response_id,
            tools: self.tools,
            include: self.include,
            parallel_tool_calls: self.parallel_tool_calls,
            store: self.store,
            stream: self.stream,
            stream_options: self.stream_options,
            conversation: self.conversation,
            temperature: self.temperature,
            top_p: self.top_p,
            top_logprobs: self.top_logprobs,
            #[allow(deprecated)]
            user: self.user,
            safety_identifier: self.safety_identifier,
            prompt_cache_key: self.prompt_cache_key,
            service_tier: self.service_tier,
            reasoning: self.reasoning,
            background: self.background,
            max_output_tokens: self.max_output_tokens,
            max_tool_calls: self.max_tool_calls,
            text: self.text,
            tool_choice: self.tool_choice,
            prompt: self.prompt,
            truncation: self.truncation,
        })
    }
}

impl From<String> for ResponseInput {
    fn from(text: String) -> Self {
        ResponseInput::Text(text)
    }
}

impl From<&str> for ResponseInput {
    fn from(text: &str) -> Self {
        ResponseInput::Text(text.to_string())
    }
}

/// Conversation parameter
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ConversationParam {
    /// The unique ID of the conversation
    ConversationId(String),
    /// Conversation configuration object
    ConversationConfig(serde_json::Value),
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

/// Input item types - can be message, item, or item reference
///
/// InputItem can be one of three formats:
/// - EasyInputMessage: Simple message format with role and content (string or array)
/// - Item: Full item format supporting various message and tool call types
/// - ItemReference: Reference to an existing item by ID
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InputItem {
    /// Simple message format (role + content as string or array)
    /// Most convenient for basic text/multimodal input
    InputMessage(InputMessage),
    /// Full Item format (can be input message, output message, or tool calls)
    /// Used when including previous assistant responses or tool outputs
    Item(Item),
    /// Reference to an existing item by ID
    /// Useful for conversation continuation without repeating full content
    ItemReference(ItemReference),
}

/// Input message - simple format for user/system/developer messages
///
/// A message input to the model with a role indicating instruction following
/// hierarchy. Instructions given with the `developer` or `system` role take
/// precedence over instructions given with the `user` role. Messages with the
/// `assistant` role are presumed to have been generated by the model in previous
/// interactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMessage {
    /// The role of the message input
    /// One of `user`, `assistant`, `system`, or `developer`
    pub role: InputMessageRole,
    /// Text, image, or audio input to the model, used to generate a response.
    /// Can also contain previous assistant responses.
    /// Can be simple text or array of content parts
    pub content: InputMessageContent,
    /// The type of the message input. Always `message`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

/// Input message role
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InputMessageRole {
    /// User message
    User,
    /// Assistant message (from previous interactions)
    Assistant,
    /// System message
    System,
    /// Developer message (highest priority instructions)
    Developer,
}

/// Input message content - can be string or array of content items
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Simple text content
    Text(String),
    /// Array of content parts (text, image, file, audio)
    Parts(Vec<InputContent>),
}

/// Input content types - text, image, file, or audio
///
/// Content types that can be included in input messages.
/// Discriminated by the `type` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum InputContent {
    /// Text input to the model
    #[serde(rename = "input_text")]
    Text {
        /// The text input to the model
        text: String,
    },
    /// Image input to the model
    ///
    /// An image input to the model. Learn about image inputs:
    /// https://platform.openai.com/docs/guides/vision
    #[serde(rename = "input_image")]
    Image {
        /// The URL of the image to be sent to the model
        /// A fully qualified URL or base64 encoded image in a data URL
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        /// The ID of the file to be sent to the model
        #[serde(skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        /// The detail level of the image to be sent to the model
        /// One of `high`, `low`, or `auto`. Defaults to `auto`
        detail: ImageDetail,
    },
    /// File input to the model
    #[serde(rename = "input_file")]
    File {
        /// The ID of the file to be sent to the model
        #[serde(skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        /// The name of the file to be sent to the model
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        /// The URL of the file to be sent to the model
        #[serde(skip_serializing_if = "Option::is_none")]
        file_url: Option<String>,
        /// The content of the file to be sent to the model
        #[serde(skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
    },
    /// Audio input to the model
    #[serde(rename = "input_audio")]
    Audio {
        /// Audio data object
        input_audio: AudioData,
    },
}

/// Image detail level
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    /// Low detail (faster, less tokens)
    Low,
    /// High detail (slower, more tokens)
    High,
    /// Auto (model chooses)
    Auto,
}

/// Audio data for input audio content
///
/// Base64-encoded audio data with format specification.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioData {
    /// Base64-encoded audio data
    pub data: String,
    /// The format of the audio data
    /// Currently supported formats are `mp3` and `wav`
    pub format: AudioFormat,
}

/// Audio format enum
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    /// MP3 format
    Mp3,
    /// WAV format
    Wav,
}

/// Full Item type - content item used to generate a response
///
/// An item representing part of the context for the response to be generated by the model.
/// Can contain text, images, and audio inputs, as well as previous assistant responses
/// and tool call outputs.
///
/// Supports 19 different item types. Uses untagged enum to handle InputMessage/OutputMessage
/// both having type="message" but distinguished by presence of `id` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Item {
    /// Input message from user/system/developer (no id field)
    InputMessage(ItemInputMessage),
    /// Output message from assistant (has id field)
    OutputMessage(ItemOutputMessage),
    /// Function tool call
    FunctionCall(ItemFunctionCall),
    /// Function call output
    FunctionCallOutput(ItemFunctionCallOutput),
    /// File search tool call
    FileSearch(ItemFileSearch),
    /// Web search tool call
    WebSearch(ItemWebSearch),
    /// Reasoning item - chain of thought used by reasoning model
    Reasoning(ItemReasoning),
    /// Code interpreter tool call
    CodeInterpreter(ItemCodeInterpreter),
    /// Computer use tool call
    ComputerUse(ItemComputerUse),
    /// Computer call output
    ComputerCallOutput(ItemComputerCallOutput),
    /// Image generation tool call
    ImageGen(ItemImageGen),
    /// Local shell tool call
    LocalShell(ItemLocalShell),
    /// Local shell tool call output
    LocalShellOutput(ItemLocalShellOutput),
    /// MCP list tools
    MCPListTools(ItemMCPListTools),
    /// MCP approval request
    MCPApprovalRequest(ItemMCPApprovalRequest),
    /// MCP approval response
    MCPApprovalResponse(ItemMCPApprovalResponse),
    /// MCP tool call
    MCPToolCall(ItemMCPToolCall),
    /// Custom tool call
    Custom(ItemCustom),
    /// Custom tool call output
    CustomOutput(ItemCustomOutput),
}

/// Input message item (type="message", no id)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemInputMessage {
    /// Type is always "message"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Role (user, system, or developer)
    pub role: InputMessageRole,
    /// Content array
    pub content: Vec<InputContent>,
    /// Status (optional, for API returns)
    /// One of `in_progress`, `completed`, or `incomplete`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// Output message item (type="message", has id)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemOutputMessage {
    /// Type is always "message"
    #[serde(rename = "type")]
    pub item_type: String,
    /// ID of the message
    pub id: String,
    /// Role (always assistant for output)
    pub role: String,
    /// Content array
    #[serde(default)]
    pub content: Vec<OutputContent>,
    /// Status
    #[serde(default)]
    pub status: String,
}

/// Function call item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemFunctionCall {
    /// Type is always "function_call"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID generated by the model
    pub call_id: String,
    /// Function name
    pub name: String,
    /// JSON string of arguments
    pub arguments: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// Function call output item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemFunctionCallOutput {
    /// Type is always "function_call_output"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID (matches the function_call call_id)
    pub call_id: String,
    /// JSON string of output
    pub output: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// File search item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemFileSearch {
    /// Type is always "file_search"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<serde_json::Value>>,
}

/// Web search item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemWebSearch {
    /// Type is always "web_search"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<serde_json::Value>>,
}

/// Reasoning item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemReasoning {
    /// Type is always "reasoning"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Unique identifier
    pub id: String,
    /// Encrypted content (if included in request)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    /// Reasoning summary content
    #[serde(default)]
    pub summary: Vec<ReasoningSummaryPart>,
    /// Reasoning text content
    #[serde(default)]
    pub content: Vec<ReasoningContentPart>,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// Code interpreter item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemCodeInterpreter {
    /// Type is always "code_interpreter"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Code input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// Execution outputs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<serde_json::Value>>,
}

/// Computer use item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemComputerUse {
    /// Type is always "computer_use"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Computer call output item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemComputerCallOutput {
    /// Type is always "computer_call_output"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Output data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Image gen item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemImageGen {
    /// Type is always "image_gen"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Local shell item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemLocalShell {
    /// Type is always "local_shell"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Local shell output item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemLocalShellOutput {
    /// Type is always "local_shell_output"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Output data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// MCP list tools item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemMCPListTools {
    /// Type is always "mcp_list_tools"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// MCP approval request item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemMCPApprovalRequest {
    /// Type is always "mcp_approval_request"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// MCP approval response item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemMCPApprovalResponse {
    /// Type is always "mcp_approval_response"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// MCP tool call item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemMCPToolCall {
    /// Type is always "mcp"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Custom tool call item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemCustom {
    /// Type is always "custom"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Additional data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Custom output item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemCustomOutput {
    /// Type is always "custom_output"
    #[serde(rename = "type")]
    pub item_type: String,
    /// Call ID
    pub call_id: String,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Output data
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Item status enum
///
/// The status of item. Populated when items are returned via API.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    /// Item is being processed
    InProgress,
    /// Item processing completed successfully
    Completed,
    /// Item processing incomplete
    Incomplete,
}

/// Item reference - an internal identifier for an item to reference
///
/// Used to reference previously generated items without including full content.
/// Useful for efficient conversation continuation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemReference {
    /// The type of item to reference. Always `item_reference`
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ref_type: Option<String>,
    /// The ID of the item to reference
    pub id: String,
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
    pub billing: Option<serde_json::Value>,
    /// Any error that occurred during processing
    pub error: Option<ResponseError>,
    /// Details about incomplete responses
    pub incomplete_details: Option<serde_json::Value>,
    /// Instructions provided to the model
    pub instructions: Option<String>,
    /// Maximum output tokens
    pub max_output_tokens: Option<i32>,
    /// Maximum tool calls
    pub max_tool_calls: Option<i32>,
    /// Metadata associated with the response
    pub metadata: Option<Metadata>,
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
    pub reasoning: Option<Reasoning>,
    /// Safety identifier
    pub safety_identifier: Option<String>,
    /// Service tier
    pub service_tier: Option<String>,
    /// Store flag
    pub store: Option<bool>,
    /// Temperature setting
    pub temperature: Option<f32>,
    /// Text format settings
    pub text: Option<TextConfiguration>,
    /// Tool choice setting
    pub tool_choice: Option<ToolChoice>,
    /// Available tools
    pub tools: Option<Vec<Tool>>,
    /// Top logprobs setting
    pub top_logprobs: Option<i32>,
    /// Top P setting
    pub top_p: Option<f32>,
    /// Truncation setting
    pub truncation: Option<TruncationStrategy>,
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

/// Output item in the response - can be various types
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum OutputItem {
    /// Output message from the model
    #[serde(rename = "message")]
    Message {
        id: String,
        role: String,
        #[serde(default)]
        content: Vec<OutputContent>,
        #[serde(default)]
        status: String,
    },
    /// Function tool call
    #[serde(rename = "function_call")]
    FunctionCall {
        id: String,
        call_id: String,
        name: String,
        arguments: String,
        #[serde(default)]
        status: String,
    },
    /// File search tool call
    #[serde(rename = "file_search")]
    FileSearch {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        results: Option<Vec<serde_json::Value>>,
    },
    /// Web search tool call
    #[serde(rename = "web_search")]
    WebSearch {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        query: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        results: Option<Vec<serde_json::Value>>,
    },
    /// Reasoning item
    #[serde(rename = "reasoning")]
    Reasoning {
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        encrypted_content: Option<String>,
        #[serde(default)]
        summary: Vec<ReasoningSummaryPart>,
        #[serde(default)]
        content: Vec<ReasoningContentPart>,
        #[serde(default)]
        status: String,
    },
    /// Code interpreter tool call
    #[serde(rename = "code_interpreter")]
    CodeInterpreter {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        input: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        outputs: Option<Vec<serde_json::Value>>,
    },
    /// Computer use tool call
    #[serde(rename = "computer_use")]
    ComputerUse {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
    /// Image generation tool call
    #[serde(rename = "image_gen")]
    ImageGen {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
    /// Local shell tool call
    #[serde(rename = "local_shell")]
    LocalShell {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
    /// MCP tool call
    #[serde(rename = "mcp")]
    MCPToolCall {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
    /// MCP list tools
    #[serde(rename = "mcp_list_tools")]
    MCPListTools {
        id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
    /// MCP approval request
    #[serde(rename = "mcp_approval_request")]
    MCPApprovalRequest {
        id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
    /// Custom tool call
    #[serde(rename = "custom")]
    Custom {
        id: String,
        call_id: String,
        #[serde(default)]
        status: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
}

/// Reasoning summary part
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReasoningSummaryPart {
    /// Type is always "summary_text"
    #[serde(rename = "type")]
    pub part_type: String,
    /// Summary text
    pub text: String,
}

/// Reasoning content part
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReasoningContentPart {
    /// Type is always "reasoning_text"
    #[serde(rename = "type")]
    pub part_type: String,
    /// Reasoning text
    pub text: String,
}

impl OutputItem {
    /// Get the ID of the output item
    pub fn id(&self) -> &str {
        match self {
            OutputItem::Message { id, .. }
            | OutputItem::FunctionCall { id, .. }
            | OutputItem::FileSearch { id, .. }
            | OutputItem::WebSearch { id, .. }
            | OutputItem::Reasoning { id, .. }
            | OutputItem::CodeInterpreter { id, .. }
            | OutputItem::ComputerUse { id, .. }
            | OutputItem::ImageGen { id, .. }
            | OutputItem::LocalShell { id, .. }
            | OutputItem::MCPToolCall { id, .. }
            | OutputItem::MCPListTools { id, .. }
            | OutputItem::MCPApprovalRequest { id, .. }
            | OutputItem::Custom { id, .. } => id,
        }
    }

    /// Get the type of the output item as a string
    pub fn item_type(&self) -> &str {
        match self {
            OutputItem::Message { .. } => "message",
            OutputItem::FunctionCall { .. } => "function_call",
            OutputItem::FileSearch { .. } => "file_search",
            OutputItem::WebSearch { .. } => "web_search",
            OutputItem::Reasoning { .. } => "reasoning",
            OutputItem::CodeInterpreter { .. } => "code_interpreter",
            OutputItem::ComputerUse { .. } => "computer_use",
            OutputItem::ImageGen { .. } => "image_gen",
            OutputItem::LocalShell { .. } => "local_shell",
            OutputItem::MCPToolCall { .. } => "mcp",
            OutputItem::MCPListTools { .. } => "mcp_list_tools",
            OutputItem::MCPApprovalRequest { .. } => "mcp_approval_request",
            OutputItem::Custom { .. } => "custom",
        }
    }

    /// Get the status of the output item
    pub fn status(&self) -> &str {
        match self {
            OutputItem::Message { status, .. }
            | OutputItem::FunctionCall { status, .. }
            | OutputItem::FileSearch { status, .. }
            | OutputItem::WebSearch { status, .. }
            | OutputItem::Reasoning { status, .. }
            | OutputItem::CodeInterpreter { status, .. }
            | OutputItem::ComputerUse { status, .. }
            | OutputItem::ImageGen { status, .. }
            | OutputItem::LocalShell { status, .. }
            | OutputItem::MCPToolCall { status, .. }
            | OutputItem::MCPListTools { status, .. }
            | OutputItem::MCPApprovalRequest { status, .. }
            | OutputItem::Custom { status, .. } => status,
        }
    }

    /// Get the role if this is a message
    pub fn role(&self) -> Option<&str> {
        match self {
            OutputItem::Message { role, .. } => Some(role),
            _ => None,
        }
    }

    /// Get the content if this is a message
    pub fn content(&self) -> Option<&[OutputContent]> {
        match self {
            OutputItem::Message { content, .. } => Some(content),
            _ => None,
        }
    }
}

/// Output content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputContent {
    /// Type of content
    #[serde(rename = "type")]
    pub content_type: String,
    /// Text content
    pub text: Option<String>,
    /// Annotations
    pub annotations: Option<Vec<serde_json::Value>>,
    /// Logprobs
    pub logprobs: Option<Vec<serde_json::Value>>,
}

/// Usage statistics for Responses API
#[derive(Debug, Deserialize, Clone)]
pub struct ResponseUsage {
    /// Input tokens used
    pub input_tokens: i32,
    /// Input tokens details
    pub input_tokens_details: Option<serde_json::Value>,
    /// Output tokens generated
    pub output_tokens: i32,
    /// Output tokens details
    pub output_tokens_details: Option<serde_json::Value>,
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
    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        summary_index: u32,
        part: ReasoningSummaryPart,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        summary_index: u32,
        part: ReasoningSummaryPart,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryTextDelta {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        summary_index: u32,
        delta: String,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryTextDone {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        summary_index: u32,
        text: String,
    },
    #[serde(rename = "response.reasoning_text.delta")]
    ReasoningTextDelta {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        delta: String,
    },
    #[serde(rename = "response.reasoning_text.done")]
    ReasoningTextDone {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        text: String,
    },
    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        delta: String,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        sequence_number: u32,
        item_id: String,
        output_index: u32,
        content_index: u32,
        refusal: String,
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

// ========== Additional Type Definitions ==========

/// Set of 16 key-value pairs that can be attached to an object.
/// Keys are strings with a maximum length of 64 characters.
/// Values are strings with a maximum length of 512 characters.
pub type Metadata = HashMap<String, String>;

/// Configuration options for reasoning models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reasoning {
    /// Constrains effort on reasoning (minimal, low, medium, high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// Summary of the reasoning performed (auto, concise, detailed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummary>,
    /// Deprecated: use `summary` instead
    #[deprecated(note = "Use `summary` instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_summary: Option<ReasoningSummary>,
}

/// Configuration options for text response from the model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextConfiguration {
    /// Format configuration for text response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TextResponseFormat>,
    /// Verbosity level (low, medium, high)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,
}

/// Text response format configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TextResponseFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
    #[serde(rename = "json_schema")]
    JsonSchema {
        /// Description of what the response format is for
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Name of the response format
        name: String,
        /// The JSON schema
        schema: serde_json::Value,
        /// Whether to enable strict schema adherence
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
}

/// Tool that can be used to generate a response
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Tool {
    #[serde(rename = "function")]
    Function {
        /// The name of the function
        name: String,
        /// Description of what the function does
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Parameters the function accepts, described as a JSON Schema object
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
        /// Whether to enable strict schema adherence
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
    #[serde(rename = "file_search")]
    FileSearch {
        #[serde(skip_serializing_if = "Option::is_none")]
        file_search: Option<serde_json::Value>,
    },
    #[serde(rename = "web_search")]
    WebSearch {
        #[serde(skip_serializing_if = "Option::is_none")]
        web_search: Option<serde_json::Value>,
    },
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "computer_use")]
    ComputerUse {
        #[serde(skip_serializing_if = "Option::is_none")]
        computer_use: Option<serde_json::Value>,
    },
    #[serde(rename = "mcp")]
    MCP { mcp: serde_json::Value },
}

/// Function definition for function calling
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionDefinition {
    /// The name of the function
    pub name: String,
    /// Description of what the function does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Parameters the function accepts, described as a JSON Schema object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// Whether to enable strict schema adherence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// How the model should select which tool to use
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    /// String option (auto, required, none)
    String(String),
    /// Specific tool selection
    Object(ToolChoiceObject),
}

/// Object for specific tool selection
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ToolChoiceObject {
    #[serde(rename = "function")]
    Function { function: FunctionChoice },
    #[serde(rename = "allowed_tools")]
    AllowedTools { mode: String, tools: Vec<String> },
}

/// Function choice specification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionChoice {
    /// Name of the function to call
    pub name: String,
}

/// Options for streaming responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseStreamOptions {
    /// When true, stream obfuscation will be enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
}

/// Reference to a prompt template and its variables
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    /// The unique identifier of the prompt template to use
    pub id: String,
    /// Optional version of the prompt template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Variables for the prompt template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
}

// ========== Enum Type Definitions ==========

/// Reasoning effort level for reasoning models
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

impl Default for ReasoningEffort {
    fn default() -> Self {
        ReasoningEffort::Medium
    }
}

/// Reasoning summary level
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummary {
    Auto,
    Concise,
    Detailed,
}

/// Verbosity level for text responses
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    Medium,
    High,
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Medium
    }
}

/// Service tier for processing requests
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

impl Default for ServiceTier {
    fn default() -> Self {
        ServiceTier::Auto
    }
}

/// Truncation strategy for model responses
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TruncationStrategy {
    Auto,
    Disabled,
}

impl Default for TruncationStrategy {
    fn default() -> Self {
        TruncationStrategy::Disabled
    }
}
