use std::time::{Duration, Instant};
pub use safehouse_render as render;
use render::{RenderManager, gpu::winit, model::ModelData, vertex_type::ColorVertex};
use winit::{dpi::{LogicalSize, Size}, event_loop::EventLoop, window::WindowBuilder};

fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop!");
    let wb = WindowBuilder::new();
    let window = wb
    .with_title("SceneObject Example")
    .with_inner_size(Size::new(LogicalSize::new(800,600)))
    .build(&event_loop)
    .expect("Could not create window!");

    let mut rm = RenderManager::new(&window); 

    rm.add_model("triangle", ModelData::create_model(
        &rm.gpu_state, 
        None, 
        &[
            ColorVertex::new([0.0,0.5,0.0,1.0],[1.0,0.0,0.0,1.0]),
            ColorVertex::new([0.5,-0.5,0.0,1.0],[0.0,1.0,0.0,1.0]),
            ColorVertex::new([-0.5,-0.5,0.0,1.0],[0.0,0.0,1.0,1.0]),
        ], 
        None
    ));

    rm.add_scene_object("test triangle", "triangle", "default");

    let mut last_rendered = Instant::now();

    let _ = event_loop.run(move |root_event, ewt|{
        match root_event {
            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::Resized(size) => rm.gpu_state.set_resize(size.width, size.height),
                winit::event::WindowEvent::CloseRequested => ewt.exit(),
                winit::event::WindowEvent::Destroyed => ewt.exit(),
                winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    // pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
                },
                winit::event::WindowEvent::RedrawRequested => {
                    if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {
                        // println!("draw");
                        rm.gpu_state.update_resize();
                        rm.update_time();
                        rm.render();
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
