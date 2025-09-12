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

    let input = vec![ResponseInput {
        input_type: "text".to_string(),
        text: Some("人生的意义是什么？".to_string()),
        image: None,
    }];

    let request = CreateResponseRequest {
        model: "gpt-4".to_string(),
        input,
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

            if let Some(error) = &response.error {
                println!("错误: {} - {}", error.code, error.message);
            }

            if let Some(output) = &response.output {
                println!("\n--- 输出内容 ---");
                println!("输出类型: {}", output.output_type);
                for content in &output.content {
                    if let Some(text) = &content.text {
                        println!("文本内容: {}", text);
                    }
                }
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
