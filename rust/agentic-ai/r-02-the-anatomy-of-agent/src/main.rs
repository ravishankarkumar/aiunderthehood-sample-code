use std::collections::HashMap;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const MODEL: &str = "qwen3.5:9b";
const MAX_STEPS: usize = 6;

#[derive(Debug, Deserialize, Serialize)]
struct Action {
    #[serde(rename = "type")]
    action_type: String, // "tool" | "final"
    name: Option<String>,
    #[serde(default)]
    args: Value,
    answer: Option<String>,
}

type ToolFn = fn(Value) -> Result<String, String>;

struct Agent {
    ollama: Ollama,
    tools: HashMap<String, ToolFn>,
    history: Vec<ChatMessage>,
}

impl Agent {
    fn new(tools: HashMap<String, ToolFn>) -> Self {
        Self { ollama: Ollama::default(), tools, history: vec![] }
    }

    async fn step(&mut self, observation: String) -> Result<(String, bool), Box<dyn std::error::Error>> {
        let system = ChatMessage::system(
            "Return ONLY JSON: {\"type\":\"tool|final\",\"name\":string|null,\"args\":object,\"answer\":string|null}".to_string()
        );
        let mut messages = vec![system];
        messages.extend(self.history.clone());
        messages.push(ChatMessage::user(observation));

        let request = ChatMessageRequest::new(MODEL.to_string(), messages);
        let response = self.ollama.send_chat_messages(request).await?;
        let raw = response.message.content;
        let action: Action = serde_json::from_str(&raw)?;
        self.history.push(ChatMessage::assistant(raw.clone()));

        if action.action_type == "final" {
            return Ok((action.answer.unwrap_or_else(|| "No answer provided.".to_string()), true));
        }

        if action.action_type == "tool" {
            let tool_name = action.name.unwrap_or_default();
            let tool_result = if let Some(tool) = self.tools.get(&tool_name) {
                tool(action.args).unwrap_or_else(|e| format!("Tool error: {e}"))
            } else {
                format!("Tool '{tool_name}' not available.")
            };
            self.history.push(ChatMessage::user(format!("Tool result: {tool_result}")));
            return Ok((tool_result, false));
        }

        Ok((format!("Unknown action type: {}", action.action_type), false))
    }

    async fn run(&mut self, goal: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut observation = goal.to_string();
        for step_no in 1..=MAX_STEPS {
            let (next_observation, done) = self.step(observation).await?;
            observation = next_observation;
            println!("[step {step_no}/{MAX_STEPS}] completed");
            if done {
                return Ok(observation);
            }
        }
        Ok("Stopped: reached max iterations without final answer.".to_string())
    }
}

fn web_search(args: Value) -> Result<String, String> {
    let query = args.get("query").and_then(Value::as_str).unwrap_or("unknown query");
    Ok(format!("[mock] top search results for: {query}"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tools: HashMap<String, ToolFn> = HashMap::new();
    tools.insert("web_search".to_string(), web_search);

    let mut agent = Agent::new(tools);
    let final_answer = agent.run("Research the current state of MCP and summarize key updates.").await?;
    println!("{final_answer}");
    Ok(())
}