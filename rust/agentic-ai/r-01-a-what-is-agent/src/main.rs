use dotenvy::from_path;
use gemini_client_api::gemini::types::request::Tool;
use std::env;
use std::fs;
use std::path::Path;

use gemini_client_api::gemini::utils::execute_function_calls;
use gemini_client_api::gemini::ask::Gemini;
use gemini_client_api::gemini::types::sessions::Session;
use gemini_client_api::serde_json::{json, Value};

#[allow(non_camel_case_types)]
pub struct count_files;

impl count_files {
    // Change the return type to Result<Value, String>
    pub async fn execute(directory: Value) -> Result<Value, String> {
        // Extract the string, default to current directory if not provided
        let directory_str = directory.as_str().unwrap_or(".");

        let path = Path::new(directory_str);

        if !path.exists() {
            // We return Ok here so the AI receives the message about the missing folder
            return Ok(json!({
                "error": format!("directory '{}' does not exist", directory_str)
            }));
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
                return Ok(json!({
                    "error": format!("failed to read directory: {}", e)
                }));
            }
        };

        // Wrap the final JSON in Ok()
        Ok(json!({
            "directory": directory_str,
            "count": count
        }))
    }
}

fn get_count_files_schema() -> Value {
    json!({
        "name": "count_files",
        "description": "Count the number of regular non-hidden files in a directory.",
        "parameters": {
            "type": "OBJECT",
            "properties": {
                "directory": {
                    "type": "STRING",
                    "description": "Directory path. Use '.' for the current folder."
                }
            },
            "required": ["directory"]
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    from_path("../.env").ok();

    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in the workspace root .env");

    let mut session = Session::new(10);

    let ai = Gemini::new(
        api_key,
        "gemini-2.5-flash", 
        Some("You are a helpful assistant. Use tools when asked about files.".into()),
    )
    .set_tools(vec![Tool::FunctionDeclarations(vec![get_count_files_schema()])]);

    let prompt = "Count the number of files in the current folder, and tell me one interesting mathematical fact about that number.";
    println!("User: {}\n", prompt);

    let mut response = ai.ask(session.ask(prompt)).await?;

    loop {
        if response.get_chat().has_function_call() {
            println!("Gemini requested a function call...");

            // The macro now finds count_files::execute()
            let results = execute_function_calls!(session, count_files);

            for (idx, res) in results.iter().enumerate() {
                if let Some(r) = res {
                    println!("  Call #{} result: {:?}", idx + 1, r);
                }
            }

            response = ai.ask(&mut session).await?;
        } else {
            println!("\nGemini: {}", response.get_chat().get_text_no_think("\n"));
            break;
        }
    }

    Ok(())
}