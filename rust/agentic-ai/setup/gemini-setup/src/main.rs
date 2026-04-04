use dotenvy::dotenv_override;
use std::env;

use gemini_client_api::gemini::ask::Gemini;
use gemini_client_api::gemini::types::sessions::Session;

#[tokio::main]
async fn main() {
    // Please ensure you are putting in the actual path of the .env file as mentioned in the README, and not the placeholder path shown below.
    dotenvy::from_path("../../../../.env").ok(); 

    let api_key = env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY must be set in the workspace root .env");
        
    let mut session = Session::new(10); // Keep last 10 messages
    let ai = Gemini::new(
        api_key, 
        "gemini-2.5-flash", 
        None, // Optional system instruction
    );

    let response = ai.ask(session.ask("Hello, Gemini!")).await.unwrap();
    println!("Gemini: {}", response.get_chat().get_text_no_think("\n"));
}