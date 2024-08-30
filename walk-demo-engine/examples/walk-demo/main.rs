use safehouse_render::gpu::winit::dpi::LogicalSize;
use walk_demo_engine::{scene, Engine};

fn main() {
    let (window, event_loop) = Engine::create_window("Walk Demo", safehouse_render::gpu::winit::dpi::Size::Logical(LogicalSize::new(800.0, 600.0)));

    let mut init_scene = scene::walking::WalkingScene {};

    let mut engine = Engine::new(&window, &mut init_scene);

    event_loop.run(move |root_event, ewt| {
        engine.event_loop(&mut init_scene, root_event, ewt)
    });
}