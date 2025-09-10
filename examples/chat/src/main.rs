use openai_rust::client::Client;
use openai_rust::models::{ChatCompletionRequest, ChatMessage, Role};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());
    let client = Client::new(api_key, base_url);

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
        model: "z-ai/glm-4.5-air:free".to_string(),
        messages,
        temperature: Some(0.7),
        stream: None,
    };

    println!("正在发送请求....");

    match client.chat_completion(&request).await {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
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
