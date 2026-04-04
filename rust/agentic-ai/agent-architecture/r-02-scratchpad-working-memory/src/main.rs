use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage},
    generation::parameters::FormatType,
    Ollama,
};
use serde::{Deserialize, Serialize};

const MODEL: &str = "qwen3.5:9b";
const MAX_STEPS: usize = 6;

#[derive(Clone, Debug)]
struct Step {
    thought: String,
    action: String,
    observation: String,
}

#[derive(Default)]
struct Scratchpad {
    steps: Vec<Step>,
    max_steps: usize,
}

impl Scratchpad {
    fn new(max_steps: usize) -> Self {
        Self {
            steps: Vec::new(),
            max_steps,
        }
    }

    fn add(&mut self, thought: String, action: String, observation: String) {
        self.steps.push(Step {
            thought,
            action,
            observation,
        });
        // Sliding window memory.
        if self.steps.len() > self.max_steps {
            let keep_from = self.steps.len() - self.max_steps;
            self.steps = self.steps.split_off(keep_from);
        }
    }

    fn build_prompt(&self, goal: &str) -> String {
        let mut memory = String::new();
        if self.steps.is_empty() {
            memory.push_str("(empty)\n");
        } else {
            for step in &self.steps {
                memory.push_str(&format!(
                    "Thought: {}\nAction: {}\nObservation: {}\n\n",
                    step.thought, step.action, step.observation
                ));
            }
        }

        format!(
            r#"You are an agent planner.
Return ONLY JSON:
{{
  "thought": string,
  "action": "web_search" | "finish",
  "input": string
}}

GOAL:
{}

WORKING MEMORY:
{}"#,
            goal, memory
        )
    }

    fn render(&self) -> String {
        if self.steps.is_empty() {
            return "(empty)".to_string();
        }

        let mut text = String::new();
        for (i, step) in self.steps.iter().enumerate() {
            text.push_str(&format!(
                "{}. Thought: {}\n   Action: {}\n   Observation: {}\n",
                i + 1,
                step.thought,
                step.action,
                step.observation
            ));
        }
        text
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentAction {
    thought: String,
    action: String,
    input: String,
}

fn parse_agent_action(raw: &str) -> Result<AgentAction, Box<dyn std::error::Error>> {
    // First try direct JSON parsing.
    if let Ok(action) = serde_json::from_str::<AgentAction>(raw) {
        return Ok(action);
    }

    // Fallback: extract first JSON object from mixed output.
    if let (Some(start), Some(end)) = (raw.find('{'), raw.rfind('}')) {
        if end > start {
            let candidate = &raw[start..=end];
            let action = serde_json::from_str::<AgentAction>(candidate)?;
            return Ok(action);
        }
    }

    Err(format!("Model returned non-JSON output: {}", &raw.chars().take(200).collect::<String>()).into())
}

fn web_search(query: &str) -> String {
    format!(
        "[mock-search-result] {query}: RTX 4090 has strong local value; H100 leads datacenter throughput."
    )
}

async fn run_agent(goal: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ollama = Ollama::default();
    let mut scratchpad = Scratchpad::new(10);

    for step_no in 1..=MAX_STEPS {
        let prompt = scratchpad.build_prompt(goal);
        let request = ChatMessageRequest::new(MODEL.to_string(), vec![ChatMessage::user(prompt)])
            .format(FormatType::Json);
        let response = ollama.send_chat_messages(request).await?;
        let action = match parse_agent_action(&response.message.content) {
            Ok(action) => action,
            Err(_) => {
                scratchpad.add(
                    "Parser failure".to_string(),
                    "finish".to_string(),
                    "Model output was not valid JSON. Retrying with stricter prompt is recommended.".to_string(),
                );
                println!("[step {step_no}/{MAX_STEPS}] parse_error");
                println!("[scratchpad]\n{}", scratchpad.render());
                return Ok("Stopped: model did not return valid structured output.".to_string());
            }
        };

        if action.action == "finish" {
            scratchpad.add(action.thought, "finish".to_string(), action.input.clone());
            println!("[step {step_no}/{MAX_STEPS}] finished");
            println!("[scratchpad]\n{}", scratchpad.render());
            return Ok(action.input);
        }

        if action.action == "web_search" {
            let tool_result = web_search(&action.input);
            scratchpad.add(
                action.thought,
                format!("web_search({})", action.input),
                tool_result,
            );
            println!("[step {step_no}/{MAX_STEPS}] tool=web_search");
            println!("[scratchpad]\n{}", scratchpad.render());
            continue;
        }

        scratchpad.add(
            action.thought,
            action.action,
            "Unknown action. Ask model to finish.".to_string(),
        );
        println!("[step {step_no}/{MAX_STEPS}] tool=unknown");
        println!("[scratchpad]\n{}", scratchpad.render());
    }

    Ok("Stopped: reached max iterations without final answer.".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let final_answer = run_agent("Find the best laptop GPU choice for local ML and explain why.").await?;
    println!("\nFinal answer:\n{final_answer}");
    Ok(())
}