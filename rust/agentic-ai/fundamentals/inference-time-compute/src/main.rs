// This code is not tested
// Only Ollama and Gemini based examples are tested. The rest are for illustrative purposes and may not compile or run as-is.
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let body = json!({
        "model": "o1",
        "messages": [{
            "role": "user",
            "content": "Design a fault-tolerant multi-agent system."
        }],
        "reasoning_effort": "high"
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", std::env::var("OPENAI_API_KEY")?))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let msg = &response["choices"][0]["message"]["content"];

    println!("=== Answer ===\n{}", msg);

    Ok(())
}