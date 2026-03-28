use murali::engine::render::RenderOptions;

pub mod export_example;
pub mod export2;

const SCENES: &[&str] = &["export-example", "export2"];

pub fn run(scene: &str, opts: RenderOptions) -> anyhow::Result<()> {
    match scene {
        "export-example" => export_example::run(opts),
        "export2" => export2::run(opts),
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