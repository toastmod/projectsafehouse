use std::time::{Duration, Instant};

use ball::Ball;
use engine::{entity::Entity, gpu::winit::{dpi::{LogicalSize, Size}, event::KeyEvent, event_loop::EventLoop, window::WindowBuilder}, model::ModelData, vertex_type::ColorVertex};
use paddle::Paddle;
use pong::Pong;
pub use safehouse_engine as engine;

mod pong;
mod paddle;
mod ball;

fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop!");
    let mut wb = WindowBuilder::new();
    let window = wb
    .with_title("Pong")
    .with_inner_size(Size::new(LogicalSize::new(800,600)))
    .build(&event_loop)
    .expect("Could not create window!");

    let mut engine = engine::Engine::new(&window); 
    let mut pong = Pong::load_game(&mut engine);

    let mut last_rendered = Instant::now();

    let _ = event_loop.run(move |root_event, ewt|{
        match root_event {
            engine::gpu::winit::event::Event::WindowEvent { window_id, event } => match event {
                engine::gpu::winit::event::WindowEvent::Resized(size) => engine.gpu_state.set_resize(size.width, size.height),
                engine::gpu::winit::event::WindowEvent::CloseRequested => ewt.exit(),
                engine::gpu::winit::event::WindowEvent::Destroyed => ewt.exit(),
                engine::gpu::winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    match event.physical_key {
                        engine::gpu::winit::keyboard::PhysicalKey::Code(c) => match c {

                            engine::gpu::winit::keyboard::KeyCode::KeyA => {

                            },
                            engine::gpu::winit::keyboard::KeyCode::KeyD => {

                            },
                            engine::gpu::winit::keyboard::KeyCode::KeyS => {

                            },
                            engine::gpu::winit::keyboard::KeyCode::KeyW => {

                            },

                            _ => (),
                        },
                        engine::gpu::winit::keyboard::PhysicalKey::Unidentified(_) => (),
                    }
                },
                engine::gpu::winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
                },
                engine::gpu::winit::event::WindowEvent::RedrawRequested => {
                    if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {
                        engine.gpu_state.update_resize();
                        engine.render();
                        last_rendered = Instant::now();
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
