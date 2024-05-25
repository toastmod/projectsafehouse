pub use safehouse_render as render;
use std::time::{Duration, Instant};
use ball::Ball;
use render::{entity::Entity, gpu::winit, model::ModelData, vertex_type::ColorVertex, RenderManager};
use winit::{dpi::{LogicalSize, Size}, event::KeyEvent, event_loop::EventLoop, window::WindowBuilder};

use paddle::Paddle;
use pong::Pong;

mod pong;
mod paddle;
mod ball;

fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop!");
    let mut wb = WindowBuilder::new();
    let window = wb
    .with_title("Pong")
    .with_inner_size(Size::new(LogicalSize::new(pong::SCREEN_WIDTH,pong::SCREEN_HEIGHT)))
    .build(&event_loop)
    .expect("Could not create window!");

    let mut rm = RenderManager::new(&window); 

    let mut pong = Pong::load_game(&mut rm);

    pong.init(&mut rm);

    let mut last_rendered = Instant::now();

    let _ = event_loop.run(move |root_event, ewt|{
        match root_event {
            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::Resized(size) => rm.gpu_state.set_resize(size.width, size.height),
                winit::event::WindowEvent::CloseRequested => ewt.exit(),
                winit::event::WindowEvent::Destroyed => ewt.exit(),
                winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    match event.physical_key {
                        winit::keyboard::PhysicalKey::Code(c) => match c {

                            winit::keyboard::KeyCode::KeyA => {

                            },
                            winit::keyboard::KeyCode::KeyD => {

                            },
                            winit::keyboard::KeyCode::KeyS => {

                            },
                            winit::keyboard::KeyCode::KeyW => {

                            },

                            _ => (),
                        },
                        winit::keyboard::PhysicalKey::Unidentified(_) => (),
                    }
                },
                winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    pong.mouse_moved(&mut rm, position.x as f32, position.y as f32);
                },
                winit::event::WindowEvent::RedrawRequested => {
                    if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {
                        rm.render();
                        last_rendered = Instant::now();
                    }
                    rm.window.request_redraw();
                }

                _ => (),
            },
            // engine::gpu::winit::event::Event::LoopExiting => todo!(),
            _ => ()
        }
    });
}
