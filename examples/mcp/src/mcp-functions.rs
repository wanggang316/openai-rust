use openai_rust::client::Client;
use openai_rust::types::{
    CompletionRequest, Function, RequestMessage, Role, Tool, ToolCall, ToolChoice,
};
use serde_json::{json, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let client = Client::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()?;

    // 定义天气查询函数
    let get_weather_function = Function {
        name: "get_current_weather".to_string(),
        description: "Get the current weather in a given location".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "Temperature unit"
                }
            },
            "required": ["location"]
        }),
    };

    // 定义计算函数
    let calculator_function = Function {
        name: "calculate".to_string(),
        description: "Perform basic mathematical calculations".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate, e.g., '2 + 3 * 4'"
                }
            },
            "required": ["expression"]
        }),
    };

    let tools = vec![
        Tool {
            tool_type: "function".to_string(),
            function: get_weather_function,
        },
        Tool {
            tool_type: "function".to_string(),
            function: calculator_function,
        },
    ];

    let mut messages = vec![
        RequestMessage::new(Role::System, "You are a helpful assistant with access to weather information and a calculator. Use the tools when appropriate.".to_string()),
        RequestMessage::new(Role::User, "What's the weather like in Tokyo and calculate 15 * 23?".to_string()),
    ];

    let mut conversation_active = true;
    let mut turn_count = 0;
    const MAX_TURNS: u32 = 10;

    while conversation_active && turn_count < MAX_TURNS {
        turn_count += 1;
        println!("\n=== Turn {} ===", turn_count);

        let request = CompletionRequest::builder()
            .model("gpt-5-nano")
            .messages(messages.clone())
            .temperature(0.7)
            .tools(tools.clone())
            .tool_choice(ToolChoice::None("auto".to_string()))
            .parallel_tool_calls(true)
            .build()
            .expect("Failed to build request");

        println!("Sending request to AI...");

        match client.completions().create(&request).await {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    let assistant_message = &choice.message;

                    // 添加助手的回复到消息历史
                    let content = assistant_message.content.clone().unwrap_or_default();
                    messages.push(RequestMessage::assistant_with_tools(
                        content.clone(),
                        assistant_message.tool_calls.clone().unwrap_or_default(),
                    ));

                    // 显示助手回复的内容
                    if !content.is_empty() {
                        println!("\nAssistant: {}", content);
                    }

                    // 处理工具调用
                    if let Some(tool_calls) = &assistant_message.tool_calls {
                        println!("\nTool calls requested:");
                        for tool_call in tool_calls {
                            println!(
                                "  - {}: {}",
                                tool_call.function.name, tool_call.function.arguments
                            );

                            // 模拟工具执行
                            let tool_response = execute_tool_call(tool_call).await;

                            // 添加工具响应到消息历史
                            messages.push(RequestMessage::tool_response(
                                tool_response,
                                tool_call.id.clone(),
                            ));
                        }
                    } else {
                        // 没有工具调用，对话结束
                        conversation_active = false;
                    }

                    if choice.finish_reason == "stop" && assistant_message.tool_calls.is_none() {
                        conversation_active = false;
                    }
                } else {
                    println!("No choices returned in the response.");
                    conversation_active = false;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                conversation_active = false;
            }
        }
    }

    if turn_count >= MAX_TURNS {
        println!("\nConversation ended: Maximum turns reached");
    } else {
        println!("\nConversation completed successfully!");
    }

    Ok(())
}

async fn execute_tool_call(tool_call: &ToolCall) -> String {
    let function_name = &tool_call.function.name;
    let arguments = &tool_call.function.arguments;

    println!("Executing tool: {} with args: {}", function_name, arguments);

    match function_name.as_str() {
        "get_current_weather" => {
            // 解析参数
            let args: Value = serde_json::from_str(arguments).unwrap_or_else(|_| json!({}));
            let location = args
                .get("location")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let unit = args
                .get("unit")
                .and_then(|v| v.as_str())
                .unwrap_or("celsius");

            // 模拟天气数据
            let weather_data = json!({
                "location": location,
                "temperature": if unit == "fahrenheit" { "72°F" } else { "22°C" },
                "condition": "Partly cloudy",
                "humidity": "65%",
                "wind": "10 km/h"
            });

            format!("Weather in {}: {}", location, weather_data)
        }
        "calculate" => {
            // 解析参数
            let args: Value = serde_json::from_str(arguments).unwrap_or_else(|_| json!({}));
            let expression = args
                .get("expression")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            // 简单的计算器实现（仅用于演示）
            let result = simple_calculate(expression);
            format!("Calculation result for '{}': {}", expression, result)
        }
        _ => {
            format!("Unknown function: {}", function_name)
        }
    }
}

fn simple_calculate(expression: &str) -> String {
    // 简单的计算器实现 - 仅用于演示
    // 在实际应用中，你可能会使用更复杂的表达式求值器
    if expression == "15 * 23" {
        "345".to_string()
    } else if expression.contains("+") {
        let parts: Vec<&str> = expression.split("+").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return (a + b).to_string();
            }
        }
        "Error: Invalid expression".to_string()
    } else if expression.contains("*") {
        let parts: Vec<&str> = expression.split("*").map(|s| s.trim()).collect();
        if parts.len() == 2 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                return (a * b).to_string();
            }
        }
        "Error: Invalid expression".to_string()
    } else {
        "Error: Unsupported operation".to_string()
    }
}
