use openai_rust::client::Client;
use openai_rust::types::{CreateResponseRequest, ResponseInput};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    // 使用新的 builder 模式
    let client = Client::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()?;

    // Simple text input - just pass a string
    let request = CreateResponseRequest {
        model: "gpt-4o-mini".to_string(),
        input: ResponseInput::Text("今天星期几？".to_string()),
        instructions: Some("你是一个哲学助手，请深入思考问题。".to_string()),
        metadata: None,
        previous_response_id: None,
        tools: None,
    };

    println!("正在发送 Response API 请求....");

    // 使用新的链式调用 API
    match client.responses().create(&request).await {
        Ok(response) => {
            println!("\n--- Response API 响应 ---");
            println!("ID: {}", response.id);
            println!("对象类型: {}", response.object);
            println!("创建时间: {}", response.created_at);
            println!("模型: {}", response.model);
            println!("状态: {}", response.status);

            if let Some(error) = &response.error {
                println!("错误: {} - {}", error.code, error.message);
            }

            // Process output items
            println!("\n--- 输出内容 ---");
            for item in &response.output {
                println!("Item ID: {}", item.id);
                println!("类型: {}", item.item_type);
                if let Some(role) = &item.role {
                    println!("角色: {}", role);
                }

                // Process content within each item
                for content in &item.content {
                    if content.content_type == "output_text" {
                        if let Some(text) = &content.text {
                            println!("\n文本内容:");
                            println!("{}", text);
                        }
                    }
                }
            }

            // Show usage statistics
            if let Some(usage) = &response.usage {
                println!("\n--- 使用统计 ---");
                println!("输入 tokens: {}", usage.input_tokens);
                println!("输出 tokens: {}", usage.output_tokens);
                println!("总计 tokens: {}", usage.total_tokens);
            }

            println!("---------------------\n");
        }
        Err(e) => {
            eprintln!("Error: {e}")
        }
    }

    println!("运行结束");
    Ok(())
}
