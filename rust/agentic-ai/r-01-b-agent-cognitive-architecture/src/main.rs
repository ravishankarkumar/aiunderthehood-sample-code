use dotenvy::from_path;
use gemini_client_api::gemini::ask::Gemini;
use gemini_client_api::gemini::types::request::Tool;
use gemini_client_api::gemini::types::sessions::Session;
use gemini_client_api::gemini::utils::execute_function_calls;
use gemini_client_api::serde_json::{json, Value};

use std::env;

const SECRET_NUMBER: i32 = 12;

#[allow(non_camel_case_types)]
pub struct greater_than;

impl greater_than {
    pub async fn execute(input: Value) -> Result<Value, String> {
        let n = input
            .get("n")
            .and_then(|v| v.as_i64())
            .ok_or("Missing or invalid 'n'")? as i32;

        Ok(json!({
            "tool": "greater_than",
            "input": n,
            "result": SECRET_NUMBER > n
        }))
    }
}

#[allow(non_camel_case_types)]
pub struct is_divisible_by;

impl is_divisible_by {
    pub async fn execute(input: Value) -> Result<Value, String> {
        let n = input
            .get("n")
            .and_then(|v| v.as_i64())
            .ok_or("Missing or invalid 'n'")? as i32;

        if n == 0 {
            return Ok(json!({
                "tool": "is_divisible_by",
                "input": n,
                "error": "Division by zero is not allowed"
            }));
        }

        Ok(json!({
            "tool": "is_divisible_by",
            "input": n,
            "result": SECRET_NUMBER % n == 0
        }))
    }
}

#[allow(non_camel_case_types)]
pub struct guess_number;

impl guess_number {
    pub async fn execute(input: Value) -> Result<Value, String> {
        let n = input
            .get("n")
            .and_then(|v| v.as_i64())
            .ok_or("Missing or invalid 'n'")? as i32;

        Ok(json!({
            "tool": "guess_number",
            "input": n,
            "correct": SECRET_NUMBER == n,
            "message": if SECRET_NUMBER == n {
                "Correct! You found the secret number."
            } else {
                "Incorrect guess."
            }
        }))
    }
}

fn get_tool_schemas() -> Vec<Value> {
    vec![
        json!({
            "name": "greater_than",
            "description": "Check whether the secret number is greater than n.",
            "parameters": {
                "type": "OBJECT",
                "properties": {
                    "n": {
                        "type": "NUMBER",
                        "description": "A number to compare against the secret number."
                    }
                },
                "required": ["n"]
            }
        }),
        json!({
            "name": "is_divisible_by",
            "description": "Check whether the secret number is divisible by n.",
            "parameters": {
                "type": "OBJECT",
                "properties": {
                    "n": {
                        "type": "NUMBER",
                        "description": "A divisor to test against the secret number."
                    }
                },
                "required": ["n"]
            }
        }),
        json!({
            "name": "guess_number",
            "description": "Make a final guess for the secret number.",
            "parameters": {
                "type": "OBJECT",
                "properties": {
                    "n": {
                        "type": "NUMBER",
                        "description": "Your guess for the secret number."
                    }
                },
                "required": ["n"]
            }
        }),
    ]
}

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
            observations: Vec::new(),
            iteration: 0,
            max_iterations,
        }
    }

    fn build_prompt(&self) -> String {
        let mut prompt = format!(
            "You are a careful puzzle-solving agent.\n\
             Goal: {}\n\
             Iteration: {}/{}\n\
             The secret number is an integer between 1 and 20.\n\
             You must discover it by using tools strategically.\n\
             Do not guess too early.\n\
             Use the observations you already have to narrow the possibilities.\n\
             When you are confident enough, call guess_number.\n\n",
            self.goal, self.iteration, self.max_iterations
        );

        if self.observations.is_empty() {
            prompt.push_str("Observations so far: none.\n");
        } else {
            prompt.push_str("Observations so far:\n");
            for (i, obs) in self.observations.iter().enumerate() {
                prompt.push_str(&format!("{}. {}\n", i + 1, obs));
            }
        }

        prompt.push_str(
            "\nThink step by step.\n\
             Choose the single best next tool call.\n\
             If you already have enough evidence, use guess_number.\n",
        );

        prompt
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    from_path("../.env").ok();

    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in the workspace root .env");

    let ai = Gemini::new(
        api_key,
        "gemini-2.5-flash",
        Some(
            "You are an autonomous puzzle-solving agent. \
             Solve the hidden number game by calling tools one step at a time. \
             Be systematic, avoid random guessing, and stop once the number is found."
                .into(),
        ),
    )
    .set_tools(vec![Tool::FunctionDeclarations(get_tool_schemas())]);

    let mut session = Session::new(30);
    let mut state = AgentState::new("Find the secret number between 1 and 20.", 8);

    loop {
        if state.iteration >= state.max_iterations {
            println!("\nStopped: maximum iterations reached.");
            println!("Observations collected:");
            for obs in &state.observations {
                println!("- {}", obs);
            }
            break;
        }

        state.iteration += 1;
        println!("\n=== Iteration {} ===", state.iteration);

        let prompt = state.build_prompt();
        println!("Agent prompt:\n{}\n", prompt);

        let mut response = ai.ask(session.ask(prompt)).await?;

        let mut made_tool_call = false;

        loop {
            if response.get_chat().has_function_call() {
                made_tool_call = true;
                println!("Gemini requested a function call...");

                let results = execute_function_calls!(
                    session,
                    greater_than,
                    is_divisible_by,
                    guess_number
                );

                for (idx, res) in results.iter().enumerate() {
                    if let Some(r) = res {
                        let observation = format!("Tool result {}: {:?}", idx + 1, r);
                        println!("{}", observation);
                        state.observations.push(observation.clone());

                        if observation.contains("\"correct\":true") {
                            println!("\nAgent solved the game!");
                            println!("Final observation: {}", observation);
                            return Ok(());
                        }
                    }
                }

                response = ai.ask(&mut session).await?;
            } else {
                let text = response.get_chat().get_text_no_think("\n");
                println!("Agent says:\n{}", text);
                break;
            }
        }

        if !made_tool_call {
            println!("No tool was called in this iteration. Stopping.");
            break;
        }
    }

    Ok(())
}