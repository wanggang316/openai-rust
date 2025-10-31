use futures::StreamExt;
use openai_rust::client::Client;
use openai_rust::types::{CreateResponseRequest, ResponseStreamEvent};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let base_url =
        env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let client = Client::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()?;

    let request = CreateResponseRequest::builder()
        .model("gpt-5-nano")
        .input("人为什么而活")
        .stream(true)
        .build()
        .expect("Failed to build request");

    println!("🚀 正在发送流式请求到 Responses API...\n");
    println!("模型: {}", request.model);
    println!("输入: {:?}", request.input);
    println!("{}", "=".repeat(50));
    println!();

    let responses = client.responses();
    let mut stream = Box::pin(responses.create_stream(&request).await?);

    let mut event_count = 0;
    let mut text_buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                event_count += 1;

                match chunk.event {
                    ResponseStreamEvent::ResponseCreated { response, .. } => {
                        println!("📝 响应创建: ID={}", response.id);
                        println!("   模型: {}", response.model);
                        println!("   状态: {}", response.status);
                        println!();
                    }
                    ResponseStreamEvent::ResponseInProgress { .. } => {
                        // 静默处理进度事件
                    }
                    ResponseStreamEvent::OutputItemAdded { item, .. } => {
                        println!("➕ 输出项添加: 类型={}, ID={}", item.item_type(), item.id());
                    }
                    ResponseStreamEvent::ContentPartAdded { .. } => {
                        // 静默处理内容部分添加
                    }
                    ResponseStreamEvent::OutputTextDelta { delta, .. } => {
                        print!("{delta}");
                        text_buffer.push_str(&delta);
                        io::stdout().flush()?;
                    }
                    ResponseStreamEvent::OutputTextDone { text, .. } => {
                        println!("\n\n✅ 文本输出完成");
                        if text != text_buffer {
                            println!("⚠️  文本不一致，使用完整文本");
                        }
                    }
                    ResponseStreamEvent::ContentPartDone { .. } => {
                        // 静默处理内容部分完成
                    }
                    ResponseStreamEvent::OutputItemDone { item, .. } => {
                        println!("✔️  输出项完成: {}", item.id());
                    }
                    ResponseStreamEvent::ResponseCompleted { response, .. } => {
                        println!("\n\n");
                        println!("{}", "=".repeat(50));
                        println!("🎉 响应完成!");
                        println!("   ID: {}", response.id);
                        println!("   状态: {}", response.status);

                        if let Some(usage) = response.usage {
                            println!("\n📊 Token 使用统计:");
                            println!("   输入 tokens: {}", usage.input_tokens);
                            println!("   输出 tokens: {}", usage.output_tokens);
                            println!("   总计 tokens: {}", usage.total_tokens);
                        }

                        // 流完成，退出循环
                        break;
                    }
                    ResponseStreamEvent::Error { error } => {
                        eprintln!("❌ 流事件错误: {} - {}", error.code, error.message);
                        // 继续处理下一个事件，不退出
                    }
                    ResponseStreamEvent::Unknown => {
                        // 静默处理未知事件类型
                    }
                }
            }
            Err(e) => {
                eprintln!("\n❌ 流处理错误: {}", e);
                eprintln!("   事件序号: {}", event_count);
                // 继续处理下一个事件，不立即退出
            }
        }
    }

    println!("\n\n");
    println!("📈 统计信息:");
    println!("   总事件数: {}", event_count);
    println!("   文本长度: {} 字符", text_buffer.len());
    println!();
    println!("✨ 运行结束");

    Ok(())
}
