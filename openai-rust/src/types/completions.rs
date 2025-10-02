use core::str;

use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RequestMessage {
    #[serde(default)]
    pub role: Role,
    #[serde(default)]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tool_call_id: Option<String>,
}

impl RequestMessage {
    pub fn new(role: Role, content: String) -> Self {
        Self {
            role,
            content,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool_response(content: String, tool_call_id: String) -> Self {
        Self {
            role: Role::Tool,
            content,
            tool_calls: None,
            tool_call_id: Some(tool_call_id),
        }
    }

    pub fn assistant_with_tools(content: String, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            role: Role::Assistant,
            content,
            tool_calls: Some(tool_calls),
            tool_call_id: None,
        }
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

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub role: Role,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(alias = "reasoning_content")]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
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
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChunkChoice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Delta {
    pub role: Option<Role>,
    pub content: Option<String>,
    #[serde(alias = "reasoning_content")]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<DeltaToolCall>>,
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
