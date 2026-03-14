use dotenvy::dotenv_override;
use std::env;

fn main() {
    // Please ensure you are putting in the actual path of the .env file as mentioned in the README, and not the placeholder path shown below.
    dotenvy::from_path("../.env").ok(); 

    let api_key = env::var("GEMIN_API_KEY")
        .expect("GEMIN_API_KEY must be set in the workspace root .env");
        
    println!("Key loaded: {}", &api_key[..4]); 
}