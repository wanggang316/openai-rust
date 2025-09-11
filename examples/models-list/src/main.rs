use openai_rust::client::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in .env file");
    let base_url = env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    
    // 使用新的 builder 模式
    let client = Client::builder()
        .api_key(api_key)
        .base_url(base_url)
        .build()?;

    println!("正在获取模型列表....");

    // 使用新的链式调用 API
    match client.models().list().await {
        Ok(models_response) => {
            println!("\n--- 可用模型列表 ---");
            println!("对象类型: {}", models_response.object);
            println!("模型数量: {}\n", models_response.data.len());

            for model in models_response.data {
                println!("ID: {}", model.id);
                println!("对象类型: {}", model.object);
                println!("创建时间: {}", model.created);
                println!("拥有者: {}", model.owned_by);
                println!("---");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}")
        }
    }

    println!("运行结束");
    Ok(())
}