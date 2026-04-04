use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    Ollama,
};
use serde_json::{json, Value};

// -----------------------------
// Secret
// -----------------------------
const SECRET_NUMBER: i32 = 12;

// -----------------------------
// Tools
// -----------------------------
fn greater_than(n: i32) -> Value {
    json!({
        "tool": "greater_than",
        "input": n,
        "result": SECRET_NUMBER > n
    })
}

fn is_divisible_by(n: i32) -> Value {
    if n == 0 {
        return json!({
            "tool": "is_divisible_by",
            "input": n,
            "error": "Division by zero"
        });
    }

    json!({
        "tool": "is_divisible_by",
        "input": n,
        "result": SECRET_NUMBER % n == 0
    })
}

fn guess_number(n: i32) -> Value {
    json!({
        "tool": "guess_number",
        "input": n,
        "correct": SECRET_NUMBER == n,
        "message": if SECRET_NUMBER == n {
            "Correct! You found the secret number."
        } else {
            "Incorrect guess."
        }
    })
}

// -----------------------------
// Agent State
// -----------------------------
struct AgentState {
    goal: String,
    observations: Vec<String>,
    iteration: usize,
    max_iterations: usize,
}

impl AgentState {
    fn new(goal: &str, max_iterations: usize) -> Self {
        Self {
            goal: goal.to_string(),
            observations: vec![],
            iteration: 0,
            max_iterations,
        }
    }

    fn build_prompt(&self) -> String {
        let mut prompt = format!(
            "You are a careful puzzle-solving agent.\n\
             Goal: {}\n\
             Iteration: {}/{}\n\
             The secret number is between 1 and 20.\n\
             Use tools strategically. Do not guess early.\n\n",
            self.goal, self.iteration, self.max_iterations
        );

        if self.observations.is_empty() {
            prompt.push_str("Observations: none\n");
        } else {
            prompt.push_str("Observations:\n");
            for (i, obs) in self.observations.iter().enumerate() {
                prompt.push_str(&format!("{}. {}\n", i + 1, obs));
            }
        }

        prompt.push_str(
            "\nAvailable tools:\n\
            1. greater_than(n)\n\
            2. is_divisible_by(n)\n\
            3. guess_number(n)\n\n\
            Respond ONLY with JSON:\n\
            { \"tool\": \"name\", \"arguments\": { \"n\": number } }\n"
        );

        prompt
    }
}

// -----------------------------
// Main
// -----------------------------
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let model = "qwen3.5:9b".to_string();

    let mut state = AgentState::new(
        "Find the secret number between 1 and 20.",
        8,
    );

    let mut messages = vec![
        ChatMessage::system(
            "You are an autonomous puzzle-solving agent. Use tools step by step.".into(),
        ),
    ];

    loop {
        if state.iteration >= state.max_iterations {
            println!("❌ Max iterations reached");
            break;
        }

        state.iteration += 1;

        println!("\n=== Iteration {} ===", state.iteration);

        let prompt = state.build_prompt();
        println!("Prompt:\n{}\n", prompt);

        messages.push(ChatMessage::user(prompt));

        let request = ChatMessageRequest::new(model.clone(), messages.clone());
        let res = ollama.send_chat_messages(request).await?;

        let msg = res.message;
        let content = msg.content.clone();

        println!("Model:\n{}\n", content);

        // -----------------------------
        // Parse tool call
        // -----------------------------
        let parsed: Result<Value, _> = serde_json::from_str(&content);

        if let Ok(json) = parsed {
            if let Some(tool) = json.get("tool").and_then(|t| t.as_str()) {
                let n = json["arguments"]["n"].as_i64().unwrap_or(0) as i32;

                let result = match tool {
                    "greater_than" => greater_than(n),
                    "is_divisible_by" => is_divisible_by(n),
                    "guess_number" => {
                        let res = guess_number(n);

                        let observation = format!("{}", res);
                        println!("🎯 {}", observation);

                        if res["correct"] == true {
                            println!("✅ Solved!");
                            return Ok(());
                        }

                        res
                    }
                    _ => json!({"error": "unknown tool"}),
                };

                let observation = format!("{}", result);
                println!("📦 {}", observation);

                state.observations.push(observation.clone());

                messages.push(ChatMessage::assistant(content));
                messages.push(ChatMessage::user(format!(
                    "Tool result: {}",
                    observation
                )));

                continue;
            }
        }

        println!("⚠️ No valid tool call. Stopping.");
        break;
    }

    Ok(())
}