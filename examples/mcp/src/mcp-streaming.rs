use futures::StreamExt;
use openai_rust::client::Client;
use openai_rust::types::{
    CompletionRequest, Function, FunctionCall, RequestMessage, Role, Tool, ToolCall, ToolChoice,
};
use serde_json::{json, Value};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let client = Client::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()?;

    // 模拟从 MCP 服务器获取工具列表
    let mcp_tools = get_mcp_tools();
    println!("=== MCP Tools Available ===");
    for tool in &mcp_tools {
        println!("- {}: {}", tool.function.name, tool.function.description);
    }
    println!();

    let query = "What time is it in Tokyo, New York, and London? Also, what's the weather like in San Francisco?";
    println!("=== User Query ===");
    println!("{}\n", query);

    // 处理带工具的查询
    let final_response = process_with_tools(&client, query, &mcp_tools).await?;

    println!("=== Final Response ===");
    println!("{}", final_response);

    Ok(())
}

/// 模拟获取 MCP 工具列表
fn get_mcp_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: Function {
                name: "get_current_time".to_string(),
                description: "Get the current time in different timezones".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "timezone": {
                            "type": "string",
                            "description": "Timezone (e.g., 'UTC', 'Asia/Tokyo', 'America/New_York')",
                            "default": "UTC"
                        }
                    }
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: Function {
                name: "get_weather".to_string(),
                description: "Get current weather information for a location".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state/country, e.g. San Francisco, CA"
                        }
                    },
                    "required": ["location"]
                }),
            },
        },
    ]
}

/// 处理带工具的查询 - 完整的 MCP 流程
async fn process_with_tools(
    client: &Client,
    query: &str,
    tools: &[Tool],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut messages = vec![
        RequestMessage::new(
            Role::System,
            "You are a helpful assistant with access to real-time tools. Use them when appropriate.".to_string()
        ),
        RequestMessage::new(Role::User, query.to_string()),
    ];

    // 第一次请求 - 获取工具调用
    let request = CompletionRequest::builder()
        .model(env::var("MODEL").unwrap_or_else(|_| "gpt-4".to_string()))
        .messages(messages.clone())
        .temperature(0.7)
        .stream(true)
        .tools(tools.to_vec())
        .tool_choice(ToolChoice::None("auto".to_string()))
        .parallel_tool_calls(true)
        .build()
        .expect("Failed to build request");

    println!("=== Step 1: Initial AI Request (Streaming) ===");
    let tool_calls = stream_and_get_tool_calls(client, &request).await?;

    if tool_calls.is_empty() {
        println!("No tool calls needed - returning direct response");
        return Ok(
            "I apologize, but I need access to real-time data to answer your question accurately."
                .to_string(),
        );
    }

    println!("\n=== Step 2: Executing Tool Calls ===");
    // 执行所有工具调用
    for (index, tool_call) in tool_calls.iter().enumerate() {
        println!(
            "Executing tool {} of {}: {}",
            index + 1,
            tool_calls.len(),
            &tool_call.function_name
        );

        let result = execute_mcp_tool(tool_call).await;
        println!("✓ Result: {}\n", result);

        // 添加工具调用到消息历史
        messages.push(RequestMessage::assistant_with_tools(
            String::new(),
            vec![tool_call.to_tool_call()],
        ));

        // 添加工具结果到消息历史
        messages.push(RequestMessage::tool_response(result, tool_call.id.clone()));
    }

    println!("=== Step 3: Follow-up AI Request ===");
    // 第二次请求 - 基于工具结果生成最终回复
    let follow_up_request = CompletionRequest::builder()
        .model(env::var("MODEL").unwrap_or_else(|_| "gpt-4".to_string()))
        .messages(messages)
        .temperature(0.7)
        .stream(true)
        .build()
        .expect("Failed to build request");

    let final_response = stream_and_get_content(client, &follow_up_request).await?;
    Ok(final_response)
}

/// 流式处理并获取工具调用
async fn stream_and_get_tool_calls(
    client: &Client,
    request: &CompletionRequest,
) -> Result<Vec<CompletedToolCall>, Box<dyn std::error::Error>> {
    let completions = client.completions();
    let mut stream = Box::pin(completions.create_stream(request).await?);
    let mut chunk_count = 0;
    let mut all_tool_calls: Vec<PartialToolCall> = Vec::new();

    // 流式输出阶段：打印内容并累积工具调用
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                chunk_count += 1;

                if let Some(choice) = chunk.choices.first() {
                    // 流式输出：只打印内容
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        io::stdout().flush()?;
                    }

                    // 累积工具调用信息
                    if let Some(tool_calls) = &choice.delta.tool_calls {
                        for tool_call_delta in tool_calls {
                            let index = tool_call_delta.index as usize;

                            // 确保有足够的空间
                            while all_tool_calls.len() <= index {
                                all_tool_calls.push(PartialToolCall {
                                    id: None,
                                    tool_type: None,
                                    function_name: None,
                                    function_arguments: String::new(),
                                });
                            }

                            let partial_call = &mut all_tool_calls[index];

                            // 更新工具调用信息
                            if let Some(id) = &tool_call_delta.id {
                                partial_call.id = Some(id.clone());
                            }
                            if let Some(tool_type) = &tool_call_delta.tool_type {
                                partial_call.tool_type = Some(tool_type.clone());
                            }
                            if let Some(function) = &tool_call_delta.function {
                                if let Some(name) = &function.name {
                                    partial_call.function_name = Some(name.clone());
                                }
                                if let Some(arguments) = &function.arguments {
                                    partial_call.function_arguments.push_str(arguments);
                                }
                            }
                        }
                    }

                    // 检查是否完成
                    if choice.finish_reason.is_some() {
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error in stream: {}", e);
                break;
            }
        }
    }

    println!("\n=== Stream Summary ===");
    println!("Total chunks processed: {}", chunk_count);

    // 转换为 CompletedToolCall
    let completed_tool_calls: Vec<CompletedToolCall> = all_tool_calls
        .into_iter()
        .filter_map(|partial| {
            if let (Some(ref id), Some(ref function_name)) = (&partial.id, &partial.function_name) {
                Some(CompletedToolCall {
                    id: id.clone(),
                    tool_type: partial.tool_type.unwrap_or_else(|| "function".to_string()),
                    function_name: function_name.clone(),
                    function_arguments: partial.function_arguments,
                })
            } else {
                None
            }
        })
        .collect();

    if !completed_tool_calls.is_empty() {
        println!("Tool calls collected: {}", completed_tool_calls.len());
        for (index, tool_call) in completed_tool_calls.iter().enumerate() {
            println!(
                "Tool call {}: {} with args: {}",
                index + 1,
                tool_call.function_name,
                tool_call.function_arguments
            );
        }
    } else {
        println!("No tool calls needed");
    }

    Ok(completed_tool_calls)
}

#[derive(Debug)]
struct PartialToolCall {
    id: Option<String>,
    tool_type: Option<String>,
    function_name: Option<String>,
    function_arguments: String,
}

#[derive(Debug, Clone)]
struct CompletedToolCall {
    id: String,
    tool_type: String,
    function_name: String,
    function_arguments: String,
}

impl CompletedToolCall {
    fn to_tool_call(&self) -> ToolCall {
        ToolCall {
            id: self.id.clone(),
            tool_type: self.tool_type.clone(),
            function: FunctionCall {
                name: self.function_name.clone(),
                arguments: self.function_arguments.clone(),
            },
        }
    }
}

/// 流式处理并获取文本内容
async fn stream_and_get_content(
    client: &Client,
    request: &CompletionRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    let completions = client.completions();
    let mut stream = Box::pin(completions.create_stream(request).await?);
    let mut chunk_count = 0;

    // 流式输出阶段：只打印内容
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                chunk_count += 1;
                if let Some(choice) = chunk.choices.first() {
                    // 流式输出：只打印内容
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        io::stdout().flush()?;
                    }

                    if choice.finish_reason.is_some() {
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error in stream: {}", e);
                break;
            }
        }
    }

    println!("\n=== Content Stream Summary ===");
    println!("Total chunks processed: {}", chunk_count);

    // 完成后获取完整内容
    let mut non_stream_request = request.clone();
    non_stream_request.stream = Some(false);

    let completions = client.completions();
    let response = completions.create(&non_stream_request).await?;

    if let Some(choice) = response.choices.first() {
        let content = choice.message.content.clone().unwrap_or_default();
        println!("Final content length: {} chars", content.len());
        Ok(content)
    } else {
        Ok(String::new())
    }
}

/// 执行 MCP 工具调用
async fn execute_mcp_tool(tool_call: &CompletedToolCall) -> String {
    let function_name = &tool_call.function_name;
    let arguments = &tool_call.function_arguments;

    println!(
        "[MCP] Executing tool: {} with args: {}",
        function_name, arguments
    );

    match function_name.as_str() {
        "get_current_time" => {
            // 解析参数
            let args: Value = serde_json::from_str(arguments).unwrap_or_else(|_| json!({}));
            let timezone = args
                .get("timezone")
                .and_then(|v| v.as_str())
                .unwrap_or("UTC");

            // 模拟时间数据（在实际应用中，你会使用真实的时间库）
            match timezone {
                "Asia/Tokyo" => "Current time in Tokyo: 2024-01-15 15:30:00 JST".to_string(),
                "America/New_York" => {
                    "Current time in New York: 2024-01-15 01:30:00 EST".to_string()
                }
                "Europe/London" => "Current time in London: 2024-01-15 06:30:00 GMT".to_string(),
                "UTC" | _ => "Current time in UTC: 2024-01-15 06:30:00 UTC".to_string(),
            }
        }
        "get_weather" => {
            // 解析参数
            let args: Value = serde_json::from_str(arguments).unwrap_or_else(|_| json!({}));
            let location = args
                .get("location")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown location");

            // 模拟天气数据
            let weather_data = json!({
                "location": location,
                "temperature": "22°C (72°F)",
                "condition": "Partly cloudy",
                "humidity": "65%",
                "wind": "10 km/h NW",
                "visibility": "16 km"
            });

            format!(
                "Weather in {}: Temperature {}, {}, Humidity {}, Wind {}, Visibility {}",
                location,
                weather_data["temperature"],
                weather_data["condition"],
                weather_data["humidity"],
                weather_data["wind"],
                weather_data["visibility"]
            )
        }
        _ => {
            format!("[ERROR] Unknown MCP function: {}", function_name)
        }
    }
}
