use murali::engine::render::RenderOptions;

mod agentic_ai_code;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // First arg = scene
    let scene = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("agent_vs_pipeline");

    // Build RenderOptions
    let mut opts = RenderOptions::default();

    if args.iter().any(|a| a == "--video") {
        opts.video = Some(true);
    }

    if args.iter().any(|a| a == "--frames") {
        opts.frames = Some(true);
    }

    agentic_ai_code::run(scene, opts)
}