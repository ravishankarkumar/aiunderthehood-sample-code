use glam::{Vec3, Vec4};
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::ai::{
    neural_network_diagram::NeuralNetworkDiagram,
    signal_flow::SignalFlow,
};
use murali::frontend::collection::text::label::Label;
use murali::frontend::layout::Direction;
use murali::frontend::Tattva;
use murali::App;
use murali::engine::export::{export_scene, ExportSettings};
use murali::engine::render::RenderOptions;
use murali::frontend::theme::Theme;

fn add_tattva<T>(scene: &mut Scene, state: T, position: Vec3) -> usize
where
    T: murali::projection::Project + murali::frontend::layout::Bounded + Send + Sync + 'static,
{
    let tattva = Tattva::new(0, state);
    let id = scene.add(tattva);

    if let Some(t) = scene.get_tattva_any_mut(id) {
        let mut props = t.props().write();
        props.position = position;
    }

    id
}

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    add_tattva(
        &mut scene,
        Label::new("Neural Signal Flow", 0.34).with_color(Vec4::new(0.96, 0.98, 0.99, 1.0)),
        Vec3::ZERO,
    );
    let title_id = scene.tattvas.keys().copied().max().unwrap();
    scene.to_edge(title_id, Direction::Up, 0.35);

    let diagram = NeuralNetworkDiagram::new(vec![3, 5, 4, 2]);
    let flow_points = diagram
        .path_points(&[1, 3, 1, 0])
        .expect("signal flow path should match layer count");

    let network_id = add_tattva(&mut scene, diagram, Vec3::new(0.0, 0.4, 0.0));

    let signal_id = add_tattva(
        &mut scene,
        SignalFlow::new(flow_points)
            .with_progress(0.0)
            .with_edge_color(Vec4::new(0.98, 0.74, 0.28, 0.95))
            .with_pulse_color(Vec4::new(1.0, 0.96, 0.82, 1.0)),
        Vec3::new(0.0, 0.4, 0.0),
    );

    add_tattva(
        &mut scene,
        Label::new("A pulse travels along one selected activation path.", 0.22)
            .with_color(Vec4::new(0.79, 0.83, 0.88, 1.0)),
        Vec3::new(0.0, -3.0, 0.0),
    );

    let mut timeline = Timeline::new();
    timeline
        .animate(signal_id)
        .at(0.2)
        .for_duration(1.8)
        .ease(Ease::InOutQuad)
        .propagate()
        .spawn();

    timeline
        .animate(signal_id)
        .at(2.4)
        .for_duration(1.6)
        .ease(Ease::InOutQuad)
        .propagate_to(0.0)
        .spawn();

    scene.timelines.insert("main".to_string(), timeline);
    scene.camera_mut().position = Vec3::new(0.0, 0.0, 9.5);

    scene
    // let _ = network_id;
    // App::new()?.with_scene(scene).run_app()
}






pub fn run(opts: RenderOptions) -> anyhow::Result<()> {

    //
    let video = opts.video.unwrap_or(true);
    let frames = opts.frames.unwrap_or(true);

    println!("video: {}, frames: {}", video, frames);
    //
    let theme = Theme::ai_under_the_hood();

    let mut settings = ExportSettings {
        duration_seconds: 4.0,
        output_dir: "renders/aiu_attention_frames".into(),
        basename: "aiu_attention".to_string(),
        video_path: Some("renders/ke_aiu_attention.mp4".into()),
        clear_color: theme.background,
        ..ExportSettings::default()
    };

    // 👇 Control behavior via flags
    if !video {
        settings.video_path = None;
    }

    if !frames {
        // Future: when Murali supports it
        // settings.export_frames = false;
        println!("(frames disabled — placeholder)");
    }

    export_scene(build_scene(), &settings)
}