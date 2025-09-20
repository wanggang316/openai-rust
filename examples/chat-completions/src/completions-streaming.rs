use futures::StreamExt;
use openai_rust::client::Client;
use openai_rust::types::{ChatCompletionRequest, ChatMessage, Role};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    // 使用新的 builder 模式
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
        model: "deepseek/deepseek-r1-0528:free".to_string(),
        messages,
        temperature: Some(0.7),
        stream: Some(true),
    };

    println!("正在发送流式请求....");

    // 使用新的链式调用 API
    let completions = client.completions();
    let mut stream = Box::pin(completions.create_stream(&request).await?);

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                // 从数据块中获取第一个选择的增量内容
                if let Some(choice) = chunk.choices.first() {
                    if let Some(reasoning) = &choice.delta.reasoning {
                        print!("{reasoning}");
                        io::stdout().flush()?;
                    }

                    if let Some(content) = &choice.delta.content {
                        // 打印内容并立即刷新标准输出，以实现打字机效果
                        print!("{content}");
                        io::stdout().flush()?;
                    }
                }
            }
            Err(e) => {
                eprintln!("\n流处理出错: {e}");
            }
        }
    }

    println!("\n\n-------------------------\n");
    println!("运行结束");
    Ok(())
}
