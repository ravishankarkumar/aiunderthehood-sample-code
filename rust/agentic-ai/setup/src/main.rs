use dotenvy::dotenv_override; // or dotenv()
use std::env;

fn main() {
    // This will look for .env in the current dir, then parent, then grandparent...
    // Perfect for a workspace setup!
    // dotenvy::from_path("../../../../../.env").ok(); 
    dotenvy::from_path("../.env").ok(); 

    let api_key = env::var("GEMIN_API_KEY")
        .expect("GEMIN_API_KEY must be set in the workspace root .env");
        
    println!("Key loaded: {}", &api_key[..4]); 
}