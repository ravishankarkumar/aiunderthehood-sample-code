mod pag;
mod structured_output;


#[tokio::main]
async fn main() {
println!("Running examples for Modern LLM Primitives!");
    
    // Await the result and handle errors
    // if let Err(e) = pag::run().await {
    //     eprintln!("Error running example: {}", e);
    // }
    if let Err(e) = structured_output::run().await {
        eprintln!("Error running example: {}", e);
    }

}
