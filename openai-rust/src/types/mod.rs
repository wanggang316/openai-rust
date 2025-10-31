mod completions;
mod models;
mod responses;

// Re-export from completions module
pub use completions::{
    AudioFormat, AudioOptions, Choice, ChunkChoice, CompletionChunkResponse, CompletionRequest,
    CompletionResponse, Delta, DeltaFunction, DeltaToolCall, Function, FunctionCall,
    ReasoningEffort, RequestMessage, ResponseMessage, ResponseModality, Role, ServiceTier,
    StopConfig, StreamOptions, Tool, ToolCall, ToolChoice, ToolChoiceFunction, Usage, Verbosity,
    WebSearchApproximateLocation, WebSearchContextSize, WebSearchLocationType, WebSearchOptions,
    WebSearchUserLocation,
};

// Re-export from models module
pub use models::{Model, ModelsResponse};

// Re-export from responses module (excluding Usage to avoid conflict)
pub use responses::{
    Content, ContentPart, ConversationParam, CreateResponseRequest, CreateResponseRequestBuilder,
    CreateResponseResponse, ErrorDetails, FunctionChoice, FunctionDefinition, ImageContent,
    InputItem, MessageContent, Metadata, OutputContent, OutputItem, Prompt, Reasoning,
    ReasoningEffort as ResponsesReasoningEffort, ReasoningSummary, Response, ResponseChunk,
    ResponseError, ResponseInput, ResponseOutput, ResponseStreamEvent, ResponseStreamOptions,
    ResponseUsage, ServiceTier as ResponsesServiceTier, TextConfiguration, TextResponseFormat,
    Tool as ResponsesTool, ToolChoice as ResponsesToolChoice, ToolChoiceObject, TruncationStrategy,
    UsageStats, Verbosity as ResponsesVerbosity,
};
