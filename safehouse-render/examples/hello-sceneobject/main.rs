use std::time::{Duration, Instant};
use safehouse_gpu::{buffer::VertexBuffer, winit::event::KeyEvent};
use safehouse_render::{camera::Camera, controller::Controller};
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

    let mut controller = Controller::new(None);

    let mut camera = Camera::new(800f32, 600f32);
    camera.set_rot((-90.0f32.to_radians(),0.0));
    camera.set_pos((0.0,0.0,-5.0));
    camera.upd8(true, 0);

    rm.add_model("triangle", ModelData {
        vertex_buffer: VertexBuffer::new(&rm.gpu_state, &[
            ColorVertex::new([0.0,0.5,0.0,1.0],[1.0,0.0,0.0,1.0]),
            ColorVertex::new([0.5,-0.5,0.0,1.0],[0.0,1.0,0.0,1.0]),
            ColorVertex::new([-0.5,-0.5,0.0,1.0],[0.0,0.0,1.0,1.0]),
        ]),
        textures: None,
        model_bindgroup: None,
        groups: Box::new([0..3]),
    });

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
                        camera.update_vals(1.0*last_rendered.elapsed().as_secs_f32(), &controller);
                        rm.render(&[], &camera);
                        last_rendered = Instant::now();
                    }
                    rm.window.request_redraw();
                },

                winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    controller.keyboard_input(event.logical_key, event.state.is_pressed());
                },

                winit::event::WindowEvent::MouseInput { device_id, state, button } => {
                    controller.mouse_input(button, state.is_pressed());
                },

                _ => (),
            },

            winit::event::Event::DeviceEvent { device_id, event } => {
                controller.device_input(event, (800.0,600.0), 10.0);
            }
            // engine::gpu::winit::event::Event::LoopExiting => todo!(),
            _ => ()
        }
    });
}
