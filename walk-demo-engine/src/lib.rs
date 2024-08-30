pub mod entity;
pub mod scene;

use std::time::{Duration, Instant};

use safehouse_render::camera::Camera;
pub use safehouse_render as render;
pub use safehouse_render::gpu as gpu;
use render::{gpu::winit::{self, dpi::{LogicalSize, Size}, event_loop::EventLoop, window::{Window, WindowBuilder}}, texture::DynamicTexture, RenderManager};


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct Engine<'window, 'engine> {
    rm: RenderManager<'window>,
    last_rendered: Instant,
    dyn_texture_queue: Vec<&'engine DynamicTexture>,
    camera: Camera
}

impl<'window, 'engine> Engine<'window, 'engine> {

    pub fn create_window(title: &str, size: Size) -> (Window, EventLoop<()>) {
        let event_loop = EventLoop::new().expect("Could not create event loop!");
        let mut wb = WindowBuilder::new();
        let window = wb
        .with_title(title)
        .with_inner_size(Size::new(LogicalSize::new(800, 600)))
        .build(&event_loop)
        .expect("Could not create window!");

        (window, event_loop)
    }

    pub fn new<S: scene::Scene>(window: &'window Window, scene: &mut S) -> Self {

        let mut rm = RenderManager::new(&window); 

        let mut engine = Self {
            rm,
            last_rendered: Instant::now(),
            dyn_texture_queue: vec![],
            camera: Camera::new(window.inner_size().width as f32, window.inner_size().height as f32),
        };

        scene.init(&mut engine);
        engine

    }

    pub fn event_loop<T, S: scene::Scene>(&mut self, scene: &mut S, root_event: winit::event::Event<T>, ewt: &winit::event_loop::EventLoopWindowTarget<T>) {
        match root_event {
            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::Resized(size) => self.rm.gpu_state.set_resize(size.width, size.height),
                winit::event::WindowEvent::CloseRequested => ewt.exit(),
                winit::event::WindowEvent::Destroyed => ewt.exit(),
                winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    // pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
                },
                winit::event::WindowEvent::RedrawRequested => {
                    // Update
                    scene.update(self);

                    // Render
                    if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {
                        // println!("draw");
                        self.rm.gpu_state.update_resize();
                        self.rm.update_time();
                        self.rm.render(self.dyn_texture_queue.as_slice());
                        if !self.dyn_texture_queue.is_empty() {
                            self.dyn_texture_queue.clear();
                        }
                    }
                    self.rm.window.request_redraw();
                }

                _ => (),
            },
            // engine::gpu::winit::event::Event::LoopExiting => todo!(),
            _ => ()
        }
    }

}