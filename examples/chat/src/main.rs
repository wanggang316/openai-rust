use openai_rust::client::Client;
use openai_rust::types::{ChatCompletionRequest, ChatMessage, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    // 使用新的 builder 模式或直接创建
    let client = Client::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()?;

    let messages = vec![
        ChatMessage {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        ChatMessage {
            role: Role::User,
            content: "人生的意义是什么?".to_string(),
        },
    ];

    let request = ChatCompletionRequest {
        model: "deepseek-reasoner".to_string(),
        messages,
        temperature: Some(0.7),
        stream: None,
    };

    println!("正在发送请求....");

    // 使用新的链式调用 API
    match client.completions().create(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                // 显示推理过程（如果存在）
                if let Some(reasoning) = &choice.message.reasoning {
                    println!("\n--- 推理过程 ---");
                    println!("{reasoning}");
                }

                println!("\n--- AI 回复 ---");
                println!("{}", choice.message.content);

                println!("---------------------\n");
            } else {
                println!("No choices returned in the response.");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}")
        }
    }

    print!("运行结束");
    Ok(())
}
