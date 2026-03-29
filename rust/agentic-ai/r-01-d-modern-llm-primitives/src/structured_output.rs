use ollama_rs::{
    generation::chat::{ChatMessage, request::ChatMessageRequest},
    Ollama,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ResearchResult {
    topic: String,
    summary: String,
    sources: Vec<String>,
    confidence: f32,
    needs_more_research: bool,
}

// -----------------------------
// System Prompt (force JSON)
// -----------------------------
const SYSTEM_PROMPT: &str = r#"
You are a research assistant.

Return ONLY valid JSON matching this schema:

{
  "topic": string,
  "summary": string,
  "sources": string[],
  "confidence": number (0 to 1),
  "needs_more_research": boolean
}

Do not include any explanation or extra text.
"#;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let messages = vec![
        ChatMessage::system(SYSTEM_PROMPT.to_string()),
        ChatMessage::user("Research the current state of MCP (Model Context Protocol).".to_string()),
    ];

    let request = ChatMessageRequest::new("qwen3.5:9b".to_string(), messages);

    let response = ollama.send_chat_messages(request).await?;

    let content = response.message.content;

    // -----------------------------
    // Parse JSON → serde
    // -----------------------------
    let result: ResearchResult = serde_json::from_str(&content)?;

    println!("Topic: {}", result.topic);
    println!("Summary: {}", result.summary);
    println!("Confidence: {:.0}%", result.confidence * 100.0);
    println!("Needs more research: {}", result.needs_more_research);
    println!("Sources: {:?}", result.sources);

    Ok(())
}