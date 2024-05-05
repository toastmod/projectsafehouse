use ball::Ball;
use engine::{gpu::winit::{dpi::{LogicalSize, Size}, event_loop::EventLoop, window::WindowBuilder}, vertex_type::ColorVertex};
use paddle::Paddle;
pub use safehouse_engine as engine;

mod paddle;
mod ball;

#[derive(Debug,Default)]
struct Pong {
    player: Paddle,
    cpu: Paddle,
    ball: Ball,
}


fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop!");
    let mut wb = WindowBuilder::new();
    let window = wb
    .with_title("Pong")
    .with_inner_size(Size::new(LogicalSize::new(800,600)))
    .build(&event_loop)
    .expect("Could not create window!");

    let pong = Pong::default();

    let mut engine = engine::Engine::<Pong>::new(&window, &mut pong); 

    engine.create_model::<ColorVertex>(
        "paddle",
        None,
        &[
            ColorVertex { pos: [0.0,0.0,0.0], color: [1.0,1.0,1.0] }
        ],
        None 
    );
    
    event_loop.run(move |root_event, ewt|{
        match root_event {
            engine::gpu::winit::event::Event::WindowEvent { window_id, event } => match event {
                engine::gpu::winit::event::WindowEvent::Resized(_) => (),
                engine::gpu::winit::event::WindowEvent::CloseRequested => (),
                engine::gpu::winit::event::WindowEvent::Destroyed => (),
                // engine::gpu::winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => todo!(),
                engine::gpu::winit::event::WindowEvent::CursorMoved { device_id, position } => (),
                engine::gpu::winit::event::WindowEvent::RedrawRequested => engine.render(&pong),
                _ => (),
            },
            // engine::gpu::winit::event::Event::LoopExiting => todo!(),
            _ => ()
        }
    });
}
