use safehouse_render::gpu::winit::dpi::LogicalSize;
use walk_demo_engine::{scene::{self, walking::WalkingScene, Scene}, Engine};

fn main() {
    let (window, event_loop) = Engine::create_window("Walk Demo", safehouse_render::gpu::winit::dpi::Size::Logical(LogicalSize::new(800.0, 600.0)));

    let mut engine = Engine::new(&window);

    let mut scene: Box<dyn Scene> = engine.load_scene::<WalkingScene>();

    event_loop.run(move |root_event, ewt| {
        match engine.event_loop(&mut scene, root_event, ewt) {
            scene::SceneEvent::Continue => (),
            scene::SceneEvent::Exit => ewt.exit(),
            scene::SceneEvent::MutScene(f) => {
                f(&mut scene, &mut engine);
            },
        }
    });
}