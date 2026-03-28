use murali::engine::render::RenderOptions;

pub fn run(opts: RenderOptions) -> anyhow::Result<()> {
    let video = opts.video.unwrap_or(true);
    let frames = opts.frames.unwrap_or(true);

    println!("video: {}, frames: {}", video, frames);

    Ok(())
}