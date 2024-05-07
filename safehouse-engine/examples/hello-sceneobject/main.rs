use std::time::{Duration, Instant};
pub use safehouse_engine as engine;
use engine::{gpu::winit::{dpi::{LogicalSize, Size}, event_loop::EventLoop, window::WindowBuilder}, model::ModelData, vertex_type::ColorVertex};

fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop!");
    let wb = WindowBuilder::new();
    let window = wb
    .with_title("SceneObject Example")
    .with_inner_size(Size::new(LogicalSize::new(800,600)))
    .build(&event_loop)
    .expect("Could not create window!");

    let mut engine = engine::Engine::new(&window); 

    engine.add_model("triangle", ModelData::create_model(
        &engine.gpu_state, 
        None, 
        &[
            ColorVertex::new([0.0,0.5,0.0],[1.0,0.0,0.0]),
            ColorVertex::new([0.5,-0.5,0.0],[0.0,1.0,0.0]),
            ColorVertex::new([-0.5,-0.5,0.0],[0.0,0.0,1.0]),
        ], 
        None
    ));

    engine.add_scene_object("test triangle", "triangle", "default");

    let mut last_rendered = Instant::now();

    let _ = event_loop.run(move |root_event, ewt|{
        match root_event {
            engine::gpu::winit::event::Event::WindowEvent { window_id, event } => match event {
                engine::gpu::winit::event::WindowEvent::Resized(size) => engine.gpu_state.set_resize(size.width, size.height),
                engine::gpu::winit::event::WindowEvent::CloseRequested => ewt.exit(),
                engine::gpu::winit::event::WindowEvent::Destroyed => ewt.exit(),
                engine::gpu::winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    // pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
                },
                engine::gpu::winit::event::WindowEvent::RedrawRequested => {
                    if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {
                        // println!("draw");
                        engine.gpu_state.update_resize();
                        engine.update_delta_time();
                        engine.render();
                    }
                    engine.window.request_redraw();
                }

                _ => (),
            },
            // engine::gpu::winit::event::Event::LoopExiting => todo!(),
            _ => ()
        }
    });
}
