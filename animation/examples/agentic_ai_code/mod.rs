use murali::engine::render::RenderOptions;

pub mod agent_vs_pipeline;

const SCENES: &[&str] = &["agent_vs_pipeline"];

pub fn run(scene: &str, opts: RenderOptions) -> anyhow::Result<()> {
    match scene {
        "agent_vs_pipeline" => agent_vs_pipeline::run(opts),
        _ => {
            eprintln!("Unknown scene: {}", scene);
            eprintln!("Available:");
            for s in SCENES {
                println!(" - {}", s);
            }
            Ok(())
        }
    }
}