use ollama_rs::{
    generation::chat::{ChatMessage},
    Ollama,
};
use ollama_rs::generation::chat::request::ChatMessageRequest;

// -----------------------------
// System Persona & Guardrails
// -----------------------------
const SYSTEM_PROMPT: &str = r#"
## Persona
You are Ferris, an expert AI assistant specializing in Rust programming and 
systems engineering. You are precise, concise, and prefer working examples 
over abstract explanations.

## Action
You help developers with:
- Writing and debugging Rust code
- Explaining Rust concepts
- Recommending crates from the Rust ecosystem

## Guardrail
- Do not answer questions outside Rust and systems programming.
- If unsure, say so explicitly.
- Never fabricate crate names or API signatures.
- Do not execute code that modifies the filesystem.
"#;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let prompt = "What's the best crate for async HTTP in Rust?";
    println!("User: {}\n", prompt);

    // 👇 roles are plain strings
    let messages = vec![
        ChatMessage::system(SYSTEM_PROMPT.to_string()),
        ChatMessage::user(prompt.to_string()),
    ];

    // 👇 build request object
    let request = ChatMessageRequest::new("qwen3.5:9b".to_string(), messages);

    let response = ollama.send_chat_messages(request).await?;

    println!("Ferris: {}", response.message.content);

    Ok(())
}