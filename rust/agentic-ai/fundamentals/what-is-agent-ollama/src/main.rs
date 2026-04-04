use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

// -----------------------------
// Tool: Count Files in Directory
// -----------------------------
pub struct CountFiles;

impl CountFiles {
    pub async fn execute(directory: &Value) -> Value {
        let directory_str = directory.as_str().unwrap_or(".");
        let path = Path::new(directory_str);

        if !path.exists() {
            return json!({
                "error": format!("directory '{}' does not exist", directory_str)
            });
        }

        let count = match fs::read_dir(path) {
            Ok(entries) => entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    entry.path().is_file() && !name.starts_with('.')
                })
                .count(),
            Err(e) => {
                return json!({
                    "error": format!("failed to read directory: {}", e)
                })
            }
        };

        json!({
            "directory": directory_str,
            "count": count
        })
    }
}

// -----------------------------
// Main Agent Loop
// -----------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "qwen3.5:9b".to_string();

    let system_prompt = r#"
You are an AI agent.

You have access to a tool:

Tool: count_files
Description: Count files in a directory

When needed, respond with a JSON tool call:

{
  "tool": "count_files",
  "arguments": { "directory": "." }
}

Otherwise, respond normally.
"#;

    let user_prompt = "Count the number of files in the current folder and tell me a math fact about the number multiplied by a random prime number.";

    println!("User: {}\n", user_prompt);

    let mut messages = vec![
        ChatMessage::system(system_prompt.to_string()),
        ChatMessage::user(user_prompt.to_string()),
    ];

    loop {
        let request = ChatMessageRequest::new(model.clone(), messages.clone());
        let res = ollama.send_chat_messages(request).await?;

        let response_message = res.message;
        let content = response_message.content.clone();

        println!("Model: {}\n", content);

        // Try parsing tool call
        let parsed: Result<Value, _> = serde_json::from_str(&content);

        if let Ok(json) = parsed {
            if json.get("tool") == Some(&Value::String("count_files".to_string())) {
                let args = &json["arguments"];

                let result = CountFiles::execute(args).await;

                println!("📦 Tool Result: {}\n", result);

                // feed result back
                messages.push(ChatMessage::assistant(content));
                messages.push(ChatMessage::user(format!(
                    "Tool result: {}. Now complete the task.",
                    result
                )));

                continue;
            }
        }

        // No tool call → final answer
        println!("✅ Final Answer: {}\n", content);
        break;
    }

    Ok(())
}