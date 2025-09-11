use futures_util::{StreamExt, pin_mut};
use openai_rust::client::Client;
use openai_rust::models::{ChatCompletionRequest, ChatMessage, Role};
use std::env;
use std::io::{self, Write};

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
        model: "deepseek/deepseek-r1-0528:free".to_string(),
        messages,
        temperature: Some(0.7),
        stream: Some(true),
    };

    println!("正在发送流式请求....");

    let stream = client.chat_completions_stream(&request).await?;
    pin_mut!(stream);

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

                    // 收集推理过程
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
