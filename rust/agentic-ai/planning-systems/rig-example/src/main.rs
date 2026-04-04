use rig::client::{CompletionClient, Nothing};
use rig::completion::Prompt;
use rig::providers::ollama;

const MODEL: &str = "qwen3.5:9b";

#[derive(Debug, Default)]
struct AgentState {
    goal: String,
    messages: Vec<String>,
    next_action: String,
    final_report: Option<String>,
}

async fn reasoning_node(state: &mut AgentState, agent: &impl Prompt) {
    let prompt = format!(
        "Goal: {}\n\nHistory:\n{}\n\nDecide next action. Reply with 'use tool' to call a tool, or 'final_answer' to finish.",
        state.goal,
        state.messages.join("\n")
    );

    match agent.prompt(&prompt).await {
        Ok(response) => {
            state.next_action = if response.to_lowercase().contains("tool") {
                "tools".to_string()
            } else {
                "final_answer".to_string()
            };
            state.messages.push(format!("Assistant: {response}"));
        }
        Err(e) => {
            eprintln!("Reasoning error: {e}");
            state.next_action = "final_answer".to_string();
        }
    }
}

async fn tool_node(state: &mut AgentState) {
    let result = "H100 outperforms RTX 4090 in training throughput by 2-3x.";
    state.messages.push(format!("Tool result: {result}"));
}

async fn final_answer_node(state: &mut AgentState, agent: &impl Prompt) {
    let prompt = format!(
        "Summarize a clear final answer based on:\n{}",
        state.messages.join("\n")
    );

    match agent.prompt(&prompt).await {
        Ok(report) => state.final_report = Some(report),
        Err(e) => eprintln!("Final answer error: {e}"),
    }
}

async fn run_agent_graph(goal: String) {
    let client = ollama::Client::new(Nothing).expect("Failed to create Ollama client");
    let agent = client
        .agent(MODEL)
        .preamble("You are a GPU expert.")
        .build();

    let mut state = AgentState {
        goal,
        ..Default::default()
    };

    for i in 0..10 {
        println!("--- Iteration {} ---", i + 1);
        reasoning_node(&mut state, &agent).await;

        match state.next_action.as_str() {
            "tools" => tool_node(&mut state).await,
            "final_answer" => {
                final_answer_node(&mut state, &agent).await;
                break;
            }
            _ => break,
        }
    }

    println!(
        "\n=== FINAL REPORT ===\n{}",
        state.final_report.unwrap_or_else(|| "No report generated.".to_string())
    );
}

#[tokio::main]
async fn main() {
    run_agent_graph("Compare RTX 4090 and H100 for machine learning workloads".to_string()).await;
}
