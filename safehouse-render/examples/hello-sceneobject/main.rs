use std::time::{Duration, Instant};
use safehouse_gpu::{buffer::VertexBuffer, winit::event::KeyEvent};
use safehouse_render::{camera::Camera, controller::Controller};
pub use safehouse_render as render;
use render::{RenderManager, gpu::winit, model::ModelData, vertex_type::ColorVertex};
use winit::{dpi::{LogicalSize, Size}, event_loop::EventLoop };
use winit_app_handler::{WinitApp, WinitState};

struct HelloSceneObject {
    rm: RenderManager,
    camera: Camera,
    controller: Controller,
    last_rendered: Instant, 
}

impl WinitApp for HelloSceneObject {
    type UserEvent = ();

    fn on_start(window: &std::sync::Arc<winit::window::Window>) -> Self {

        let mut rm = RenderManager::new(&window); 

        let mut controller = Controller::new(None);

        let mut camera = Camera::new(800f32, 600f32);
        camera.set_rot((90.0f32.to_radians(),0.0f32.to_radians()));
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

        Self {
            rm,
            camera,
            controller,
            last_rendered 
        }
    }

    fn on_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(size) => self.rm.gpu_state.set_resize(size.width, size.height),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Destroyed => event_loop.exit(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => {
                // pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
            },
            winit::event::WindowEvent::RedrawRequested => {
                if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {
                    // println!("draw");
                    self.rm.gpu_state.update_resize();
                    self.rm.update_time();
                    self.camera.update_vals(1.0*self.last_rendered.elapsed().as_secs_f32(), &self.controller);
                    self.rm.render(&self.camera);
                    self.last_rendered = Instant::now();
                }
                self.rm.window.request_redraw();
            },

            winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                self.controller.keyboard_input(event.logical_key, event.state.is_pressed());
            },

            winit::event::WindowEvent::MouseInput { device_id, state, button } => {
                self.controller.mouse_input(button, state.is_pressed());
            },

            _ => (),
        };
    }
    
    fn on_device_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::DeviceEvent) {
        self.controller.device_input(event, (800.0,600.0), 10.0);
    }
}

fn main() {
    WinitState::<HelloSceneObject>::run();
}
