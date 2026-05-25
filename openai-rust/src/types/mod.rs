mod completions;
mod models;
mod responses;

// Re-export from completions module
pub use completions::{
    AudioFormat, AudioOptions, Choice, ChunkChoice, CompletionChunkResponse, CompletionRequest,
    CompletionResponse, Content, ContentPart, Delta, DeltaFunction, DeltaToolCall, Function,
    FunctionCall, ImageUrl, PromptTokensDetails, ReasoningEffort, RequestMessage, ResponseMessage,
    ResponseModality, Role, ServiceTier, StopConfig, StreamOptions, Tool, ToolCall, ToolChoice,
    ToolChoiceFunction, Usage, Verbosity, WebSearchApproximateLocation, WebSearchContextSize,
    WebSearchLocationType, WebSearchOptions, WebSearchUserLocation,
};

// Re-export from models module
pub use models::{Model, ModelsResponse};

// Re-export from responses module (excluding Usage to avoid conflict)
pub use responses::{
    AudioData, AudioFormat as ResponsesAudioFormat, ConversationParam, CreateResponseRequest,
    CreateResponseRequestBuilder, CreateResponseResponse, ErrorDetails, FunctionChoice,
    FunctionDefinition, ImageDetail, InputContent, InputItem, InputMessage, InputMessageContent,
    InputMessageRole, Item, ItemCodeInterpreter, ItemComputerCallOutput, ItemComputerUse,
    ItemCustom, ItemCustomOutput, ItemFileSearch, ItemFunctionCall, ItemFunctionCallOutput,
    ItemImageGen, ItemInputMessage, ItemLocalShell, ItemLocalShellOutput, ItemMCPApprovalRequest,
    ItemMCPApprovalResponse, ItemMCPListTools, ItemMCPToolCall, ItemOutputMessage, ItemReasoning,
    ItemReference, ItemStatus, ItemWebSearch, Metadata, OutputContent, OutputItem, Prompt,
    Reasoning, ReasoningContentPart, ReasoningEffort as ResponsesReasoningEffort, ReasoningSummary,
    ReasoningSummaryPart, Response, ResponseChunk, ResponseError, ResponseInput, ResponseOutput,
    ResponseStreamEvent, ResponseStreamOptions, ResponseUsage, ServiceTier as ResponsesServiceTier,
    TextConfiguration, TextResponseFormat, Tool as ResponsesTool,
    ToolChoice as ResponsesToolChoice, ToolChoiceObject, TruncationStrategy, UsageStats,
    Verbosity as ResponsesVerbosity,
};
