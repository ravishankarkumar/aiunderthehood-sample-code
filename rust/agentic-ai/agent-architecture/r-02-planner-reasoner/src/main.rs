use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    generation::parameters::FormatType,
    Ollama,
};
use serde::{Deserialize, Serialize};

const MODEL: &str = "qwen3.5:9b";

#[derive(Debug, Serialize, Deserialize)]
struct Step {
    number: usize,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Plan {
    steps: Vec<Step>,
}

async fn generate_plan(goal: &str) -> Result<Plan, Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let prompt = format!(
        r#"You are an expert agent planner.
Break the GOAL into a clear, logical, numbered sequence of steps.
Consider available tools, constraints, and possible uncertainties.

Goal: {}

Return **ONLY** valid JSON in this exact format (no extra text):
{{
  "steps": [
    {{"number": 1, "description": "First step description here"}},
    {{"number": 2, "description": "Second step description here"}}
  ]
}}"#,
        goal
    );

    let request = ChatMessageRequest::new(MODEL.to_string(), vec![ChatMessage::user(prompt)])
        .format(FormatType::Json);

    let response = ollama.send_chat_messages(request).await?;
    let content = response.message.content.trim();

    // Robust parsing
    let plan: Plan = serde_json::from_str(content)
        .map_err(|e| format!("Failed to parse plan JSON: {}. Raw output: {}", e, content))?;

    Ok(plan)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let goal = "Find the best laptop GPU choice for local ML and explain why.";

    println!("Generating plan with Ollama (model: {})...\n", MODEL);

    let plan = generate_plan(goal).await?;

    println!("=== GENERATED PLAN ===\n");
    for step in &plan.steps {
        println!("{}. {}", step.number, step.description);
    }
    println!("\n=== END OF PLAN ===");

    Ok(())
}