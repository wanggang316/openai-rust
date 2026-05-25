use core::str;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Array(Vec<ContentPart>),
}

impl Default for Content {
    fn default() -> Self {
        Content::Text(String::new())
    }
}

impl From<String> for Content {
    fn from(s: String) -> Self {
        Content::Text(s)
    }
}

impl From<&str> for Content {
    fn from(s: &str) -> Self {
        Content::Text(s.to_string())
    }
}

impl From<Vec<ContentPart>> for Content {
    fn from(parts: Vec<ContentPart>) -> Self {
        Content::Array(parts)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestMessage {
    #[serde(default)]
    pub role: Role,
    #[serde(default)]
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}

impl RequestMessage {
    pub fn new(role: Role, content: impl Into<Content>) -> Self {
        Self {
            role,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        }
    }

    pub fn tool_response(content: impl Into<Content>, tool_call_id: String) -> Self {
        Self {
            role: Role::Tool,
            content: content.into(),
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
            name: None,
        }
    }

    pub fn assistant_with_tools(content: impl Into<Content>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
            tool_calls: Some(tool_calls),
            tool_call_id: None,
            name: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// 控制响应模态（text/audio）。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseModality {
    Text,
    Audio,
}

/// 约束回复长度的档位设置。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    Medium,
    High,
}

/// 推理模型的推理强度预设。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

/// 控制搜索上下文窗口使用量。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchContextSize {
    Low,
    Medium,
    High,
}

/// 表示近似位置类型，目前仅支持 `approximate`。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebSearchLocationType {
    Approximate,
}

/// 近似定位信息，可协助优化网络搜索。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WebSearchApproximateLocation {
    /// 用户所在国家的 ISO 3166-1 代码，例如 `US`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub country: Option<String>,
    /// 用户所在的州/省等区域信息，例如 `California`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub region: Option<String>,
    /// 用户所在城市，例如 `San Francisco`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub city: Option<String>,
    /// 用户所在的 IANA 时区，例如 `America/Los_Angeles`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timezone: Option<String>,
}

/// 用户的大致位置信息，会附带类型标识。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSearchUserLocation {
    /// 位置类型，固定为 `approximate`。
    #[serde(rename = "type")]
    pub location_type: WebSearchLocationType,
    /// 真正的位置信息明细。
    pub approximate: WebSearchApproximateLocation,
}

/// Web 搜索工具的配置项。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WebSearchOptions {
    /// 提供用户的近似位置以改进结果排序。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_location: Option<WebSearchUserLocation>,
    /// 控制搜索占用上下文窗口的大小，默认 `medium`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub search_context_size: Option<WebSearchContextSize>,
}

/// 支持的音频输出编码格式。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    Wav,
    Aac,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

/// 音频输出配置，需在请求音频模态时提供。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioOptions {
    /// 指定使用的合成声音，例如 `alloy`、`nova` 等。
    pub voice: String,
    /// 音频返回格式，支持 `wav`、`mp3`、`flac`、`opus`、`pcm16` 等。
    pub format: AudioFormat,
}

/// 服务等级，影响延迟与计费模式。
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

/// 停止序列配置，支持单个或多个终止字符串。
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum StopConfig {
    /// 单个停止标记。
    Single(String),
    /// 多个停止标记，最多 4 个。
    Multiple(Vec<String>),
}

/// 流式返回的补充控制项。
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StreamOptions {
    /// 是否在结束前单独推送一条包含 `usage` 的统计 chunk。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_usage: Option<bool>,
    /// 是否保留随机噪声字段以抵御侧信道攻击，默认开启。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_obfuscation: Option<bool>,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct CompletionRequest {
    /// 模型 ID，例如 `gpt-4o` 或 `o3-mini`。
    #[serde(default)]
    pub model: String,
    /// 会话历史，每条消息描述一次轮次的输入输出。
    #[serde(default)]
    pub messages: Vec<RequestMessage>,
    /// 期望模型产出的模态，默认仅返回文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub modalities: Option<Vec<ResponseModality>>,
    /// 控制回复冗长程度，`medium` 为默认值。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub verbosity: Option<Verbosity>,
    /// 对推理模型设置的推理计算预算，用于平衡成本与质量。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reasoning_effort: Option<ReasoningEffort>,
    /// 限制生成部分（不含提示）的最大 token 数，包括可见输出与推理 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_completion_tokens: Option<u32>,
    /// 频率惩罚，-2 到 2，越高越不重复已有 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frequency_penalty: Option<f32>,
    /// 存在惩罚，-2 到 2，促使模型谈论新主题。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub presence_penalty: Option<f32>,
    /// Web 搜索工具设置，用于允许模型联网查找信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub web_search_options: Option<WebSearchOptions>,
    /// 每个位置返回的备选 token 数量，需同时启用 `logprobs`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top_logprobs: Option<u32>,
    /// 结构化输出配置，例如 JSON Schema。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub response_format: Option<Value>,
    /// 请求音频输出时的附加参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub audio: Option<AudioOptions>,
    /// 是否持久化存储该次补全结果用于会话 API。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub store: Option<bool>,
    /// 如果设置为 true，API 将以 SSE 推送增量结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream: Option<bool>,
    /// 自定义停止序列，匹配后停止继续生成。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stop: Option<StopConfig>,
    /// 为特定 token 调整采样偏置，键为 token ID，值范围 -100~100。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logit_bias: Option<HashMap<String, i32>>,
    /// 是否返回每个输出 token 的对数概率信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logprobs: Option<bool>,
    /// 最大输出 token 数（含理由 token），与上下文长度受限。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_tokens: Option<u32>,
    /// 为每条输入生成的候选数，默认 1，增加会提升成本。
    #[serde(rename = "n")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub n: Option<u32>,
    /// 预测输出配置，可通过预知内容加速相似响应。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prediction: Option<Value>,
    /// 指定种子值以尽量获得确定性输出（Beta 功能）。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub seed: Option<i64>,
    /// 流式模式的附加选项，例如是否返回用量统计。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream_options: Option<StreamOptions>,
    /// 模型可调用的工具列表，当前实现支持 `function` 类型的函数工具。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tools: Option<Vec<Tool>>,
    /// 工具调用策略，可要求必须调用或指定某个工具。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tool_choice: Option<ToolChoice>,
    /// 是否允许模型一次并行调用多个工具，默认 true。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parallel_tool_calls: Option<bool>,
    /// **已弃用**：旧版函数调用控制，请改用 `tool_choice`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub function_call: Option<Value>,
    /// **已弃用**：旧式函数列表，请改用 `tools`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub functions: Option<Vec<Function>>,
    /// 附加的键值元数据，最多 16 个键，每个值最长 512 字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub metadata: Option<HashMap<String, String>>,
    /// 采样温度 0~2，越高输出越随机，建议与 `top_p` 二选一调整。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub temperature: Option<f32>,
    /// nucleus 采样参数 0~1，定义累积概率阈值。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top_p: Option<f32>,
    /// **已弃用**：旧的终端用户标识，请改用 `prompt_cache_key`。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user: Option<String>,
    /// 风险控制用的稳定用户标识，推荐用散列后的 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub safety_identifier: Option<String>,
    /// 用于提示缓存分桶的键，可提升响应缓存命中率。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prompt_cache_key: Option<String>,
    /// 请求使用的服务等级，`auto` 将遵循项目级默认。
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub service_tier: Option<ServiceTier>,
    /// 附加的自定义参数，会被直接合并进顶层请求。
    #[serde(flatten)]
    #[serde(default)]
    pub extra_params: HashMap<String, Value>,
}

impl CompletionRequest {
    /// Create a builder for constructing a request
    pub fn builder() -> CompletionRequestBuilder {
        CompletionRequestBuilder::new()
    }
}

/// Builder for CompletionRequest
#[derive(Debug, Clone, Default)]
pub struct CompletionRequestBuilder {
    model: Option<String>,
    messages: Vec<RequestMessage>,
    modalities: Option<Vec<ResponseModality>>,
    verbosity: Option<Verbosity>,
    reasoning_effort: Option<ReasoningEffort>,
    max_completion_tokens: Option<u32>,
    frequency_penalty: Option<f32>,
    presence_penalty: Option<f32>,
    web_search_options: Option<WebSearchOptions>,
    top_logprobs: Option<u32>,
    response_format: Option<Value>,
    audio: Option<AudioOptions>,
    store: Option<bool>,
    stream: Option<bool>,
    stop: Option<StopConfig>,
    logit_bias: Option<HashMap<String, i32>>,
    logprobs: Option<bool>,
    max_tokens: Option<u32>,
    n: Option<u32>,
    prediction: Option<Value>,
    seed: Option<i64>,
    stream_options: Option<StreamOptions>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    parallel_tool_calls: Option<bool>,
    function_call: Option<Value>,
    functions: Option<Vec<Function>>,
    metadata: Option<HashMap<String, String>>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    user: Option<String>,
    safety_identifier: Option<String>,
    prompt_cache_key: Option<String>,
    service_tier: Option<ServiceTier>,
    extra_params: HashMap<String, Value>,
}

impl CompletionRequestBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the model ID (required)
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Set the messages (required)
    pub fn messages(mut self, messages: Vec<RequestMessage>) -> Self {
        self.messages = messages;
        self
    }

    /// Add a single message
    pub fn add_message(mut self, message: RequestMessage) -> Self {
        self.messages.push(message);
        self
    }

    /// Set temperature (0.0 to 2.0)
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Enable or disable streaming
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
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

    /// Enable or disable parallel tool calls
    pub fn parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.parallel_tool_calls = Some(enabled);
        self
    }

    /// Set maximum completion tokens
    pub fn max_completion_tokens(mut self, max_tokens: u32) -> Self {
        self.max_completion_tokens = Some(max_tokens);
        self
    }

    /// Set reasoning effort level
    pub fn reasoning_effort(mut self, effort: ReasoningEffort) -> Self {
        self.reasoning_effort = Some(effort);
        self
    }

    /// Set verbosity level
    pub fn verbosity(mut self, verbosity: Verbosity) -> Self {
        self.verbosity = Some(verbosity);
        self
    }

    /// Set top_p for nucleus sampling
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set service tier
    pub fn service_tier(mut self, tier: ServiceTier) -> Self {
        self.service_tier = Some(tier);
        self
    }

    /// Set modalities
    pub fn modalities(mut self, modalities: Vec<ResponseModality>) -> Self {
        self.modalities = Some(modalities);
        self
    }

    /// Set frequency penalty
    pub fn frequency_penalty(mut self, penalty: f32) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    /// Set presence penalty
    pub fn presence_penalty(mut self, penalty: f32) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    /// Set web search options
    pub fn web_search_options(mut self, options: WebSearchOptions) -> Self {
        self.web_search_options = Some(options);
        self
    }

    /// Set top logprobs
    pub fn top_logprobs(mut self, top_logprobs: u32) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    /// Set response format
    pub fn response_format(mut self, format: Value) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set audio options
    pub fn audio(mut self, audio: AudioOptions) -> Self {
        self.audio = Some(audio);
        self
    }

    /// Set whether to store the completion
    pub fn store(mut self, store: bool) -> Self {
        self.store = Some(store);
        self
    }

    /// Set stop sequences
    pub fn stop(mut self, stop: StopConfig) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Set logit bias
    pub fn logit_bias(mut self, bias: HashMap<String, i32>) -> Self {
        self.logit_bias = Some(bias);
        self
    }

    /// Enable or disable logprobs
    pub fn logprobs(mut self, enabled: bool) -> Self {
        self.logprobs = Some(enabled);
        self
    }

    /// Set maximum tokens
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set number of completions to generate
    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    /// Set prediction configuration
    pub fn prediction(mut self, prediction: Value) -> Self {
        self.prediction = Some(prediction);
        self
    }

    /// Set seed for deterministic output
    pub fn seed(mut self, seed: i64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set stream options
    pub fn stream_options(mut self, options: StreamOptions) -> Self {
        self.stream_options = Some(options);
        self
    }

    /// Set deprecated function_call
    #[deprecated(note = "Use tool_choice instead")]
    pub fn function_call(mut self, function_call: Value) -> Self {
        self.function_call = Some(function_call);
        self
    }

    /// Set deprecated functions
    #[deprecated(note = "Use tools instead")]
    pub fn functions(mut self, functions: Vec<Function>) -> Self {
        self.functions = Some(functions);
        self
    }

    /// Set metadata
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
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

    /// Replace the entire extra params map
    pub fn extra_params(mut self, params: HashMap<String, Value>) -> Self {
        self.extra_params = params;
        self
    }

    /// Insert or override a single extra param
    pub fn insert_extra_param(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra_params.insert(key.into(), value);
        self
    }

    /// Build the final request
    ///
    /// # Errors
    ///
    /// Returns an error if required fields (model) are not set
    pub fn build(self) -> Result<CompletionRequest, String> {
        let model = self.model.ok_or("model is required")?;

        Ok(CompletionRequest {
            model,
            messages: self.messages,
            modalities: self.modalities,
            verbosity: self.verbosity,
            reasoning_effort: self.reasoning_effort,
            max_completion_tokens: self.max_completion_tokens,
            frequency_penalty: self.frequency_penalty,
            presence_penalty: self.presence_penalty,
            web_search_options: self.web_search_options,
            top_logprobs: self.top_logprobs,
            response_format: self.response_format,
            audio: self.audio,
            store: self.store,
            stream: self.stream,
            stop: self.stop,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            prediction: self.prediction,
            seed: self.seed,
            stream_options: self.stream_options,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            function_call: self.function_call,
            functions: self.functions,
            metadata: self.metadata,
            temperature: self.temperature,
            top_p: self.top_p,
            user: self.user,
            safety_identifier: self.safety_identifier,
            prompt_cache_key: self.prompt_cache_key,
            service_tier: self.service_tier,
            extra_params: self.extra_params,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ToolChoice {
    None(String),     // "none" or "auto"
    Required(String), // "required"
    Function {
        #[serde(rename = "type")]
        tool_type: String,
        function: ToolChoiceFunction,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolChoiceFunction {
    pub name: String,
}

fn deserialize_images<'de, D>(deserializer: D) -> Result<Option<Vec<ContentPart>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ImagesField {
        Many(Vec<ContentPart>),
        Single(ContentPart),
    }

    let images = Option::<ImagesField>::deserialize(deserializer)?;

    Ok(match images {
        Some(ImagesField::Many(parts)) => Some(parts),
        Some(ImagesField::Single(part)) => Some(vec![part]),
        None => None,
    })
}

fn merge_content_and_images(
    content: Option<Content>,
    images: Option<Vec<ContentPart>>,
) -> Option<Content> {
    match (content, images) {
        (None, None) => None,
        (Some(content), None) => Some(content),
        (None, Some(images)) => Some(Content::Array(images)),
        (Some(Content::Text(text)), Some(mut images)) => {
            let mut parts = Vec::with_capacity(images.len() + if text.is_empty() { 0 } else { 1 });
            if !text.is_empty() {
                parts.push(ContentPart::Text { text });
            }
            parts.append(&mut images);
            Some(Content::Array(parts))
        }
        (Some(Content::Array(mut parts)), Some(mut images)) => {
            parts.append(&mut images);
            Some(Content::Array(parts))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseMessage {
    pub role: Role,
    pub content: Option<Content>,
    pub reasoning: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

impl<'de> Deserialize<'de> for ResponseMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawMessage {
            role: Role,
            #[serde(default)]
            content: Option<Content>,
            #[serde(default, alias = "reasoning_content")]
            reasoning: Option<String>,
            #[serde(default)]
            tool_calls: Option<Vec<ToolCall>>,
            #[serde(default, alias = "images", deserialize_with = "deserialize_images")]
            images: Option<Vec<ContentPart>>,
        }

        let raw = RawMessage::deserialize(deserializer)?;
        let content = merge_content_and_images(raw.content, raw.images);

        Ok(ResponseMessage {
            role: raw.role,
            content,
            reasoning: raw.reasoning,
            tool_calls: raw.tool_calls,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    /// Per-call breakdown of prompt tokens. Present on OpenAI and many
    /// compatible providers (OpenRouter, DeepSeek, …) to report
    /// cached-prompt accounting separately from billed input tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Sub-object of [`Usage`] carrying the cached-token breakdown.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct PromptTokensDetails {
    /// Tokens served from the provider's prompt cache. Required by
    /// callers that compute cache-aware cost.
    #[serde(default)]
    pub cached_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompletionChunkResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChunkChoice>,
    /// Token-usage statistics. Only emitted by providers when the
    /// caller sets `stream_options.include_usage = true`, and even
    /// then only on the terminal chunk of a stream. Optional so that
    /// providers/proxies that never send it (or chunks that aren't
    /// the terminal one) deserialize cleanly.
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Delta {
    pub role: Option<Role>,
    pub content: Option<Content>,
    pub reasoning: Option<String>,
    pub tool_calls: Option<Vec<DeltaToolCall>>,
}

impl<'de> Deserialize<'de> for Delta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawDelta {
            #[serde(default)]
            role: Option<Role>,
            #[serde(default)]
            content: Option<Content>,
            #[serde(default, alias = "reasoning_content")]
            reasoning: Option<String>,
            #[serde(default)]
            tool_calls: Option<Vec<DeltaToolCall>>,
            #[serde(default, alias = "images", deserialize_with = "deserialize_images")]
            images: Option<Vec<ContentPart>>,
        }

        let raw = RawDelta::deserialize(deserializer)?;
        let content = merge_content_and_images(raw.content, raw.images);

        Ok(Delta {
            role: raw.role,
            content,
            reasoning: raw.reasoning,
            tool_calls: raw.tool_calls,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeltaToolCall {
    pub index: u32,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    pub function: Option<DeltaFunction>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeltaFunction {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vast majority of streaming chunks carry no usage object.
    /// They must continue to deserialize cleanly — the field is
    /// `Option<Usage>` with `#[serde(default)]`, so a missing key is
    /// not an error.
    #[test]
    fn chunk_without_usage_deserializes_as_none() {
        let json = r#"{
            "id": "chatcmpl-1",
            "object": "chat.completion.chunk",
            "created": 1700000000,
            "model": "gpt-4o-mini",
            "choices": [
                {"index": 0, "delta": {"content": "hi"}, "finish_reason": null}
            ]
        }"#;
        let chunk: CompletionChunkResponse =
            serde_json::from_str(json).expect("chunk without usage must parse");
        assert!(chunk.usage.is_none());
    }

    /// The terminal chunk of an OpenAI stream (when the caller set
    /// `stream_options.include_usage = true`) carries the call's
    /// token counts. Caller code can then write them back to its own
    /// usage tracker and compute cost — regression target for
    /// downstream issue: `--mode json` reporting zero usage because
    /// the chunk's usage was silently dropped on deserialize.
    #[test]
    fn terminal_chunk_exposes_usage_to_caller() {
        let json = r#"{
            "id": "chatcmpl-1",
            "object": "chat.completion.chunk",
            "created": 1700000000,
            "model": "gpt-4o-mini",
            "choices": [],
            "usage": {
                "prompt_tokens": 101,
                "completion_tokens": 7,
                "total_tokens": 108
            }
        }"#;
        let chunk: CompletionChunkResponse =
            serde_json::from_str(json).expect("terminal chunk must parse");
        let usage = chunk.usage.expect("usage field must round-trip");
        assert_eq!(usage.prompt_tokens, 101);
        assert_eq!(usage.completion_tokens, 7);
        assert_eq!(usage.total_tokens, 108);
        // Providers that don't break out cached tokens leave the
        // sub-object absent; that's fine — caller treats it as zero.
        assert!(usage.prompt_tokens_details.is_none());
    }

    /// OpenAI / OpenRouter / DeepSeek also report cached-prompt
    /// tokens via `prompt_tokens_details.cached_tokens`. Cost-aware
    /// callers need this number to subtract cache hits from billed
    /// input tokens, so the sub-object must round-trip too.
    #[test]
    fn terminal_chunk_exposes_cached_tokens_breakdown() {
        let json = r#"{
            "id": "chatcmpl-1",
            "object": "chat.completion.chunk",
            "created": 1700000000,
            "model": "gpt-4o-mini",
            "choices": [],
            "usage": {
                "prompt_tokens": 500,
                "completion_tokens": 30,
                "total_tokens": 530,
                "prompt_tokens_details": {"cached_tokens": 384}
            }
        }"#;
        let chunk: CompletionChunkResponse =
            serde_json::from_str(json).expect("chunk with details must parse");
        let usage = chunk.usage.expect("usage must round-trip");
        let details = usage
            .prompt_tokens_details
            .expect("prompt_tokens_details must round-trip");
        assert_eq!(details.cached_tokens, 384);
    }
}
