use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to Ollama...");

    let ollama = Ollama::default();

    let request = ChatMessageRequest::new(
        "qwen3.5:9b".to_string(),
        vec![ChatMessage::user("Say hello from Ollama and tell me you're ready for agent development!".to_string())],
    );

    let response = ollama.send_chat_messages(request).await
        .map_err(|e| format!("Failed to connect to Ollama. Is `ollama serve` running?\nError: {}", e))?;

    println!("\n✅ Ollama is working!\n");
    println!("Response:\n{}", response.message.content);

    Ok(())
}