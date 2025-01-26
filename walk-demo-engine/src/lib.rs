pub mod entity;
pub mod scene;

use std::{sync::Arc, time::{Duration, Instant}};

use entity::ActiveEntity;
use safehouse_render::{camera::Camera, scene::SceneObject};
pub use safehouse_render as render;
pub use safehouse_render::gpu as gpu;
use render::{gpu::winit::{self, dpi::{LogicalSize, Size}, event_loop::EventLoop, window::{Window}}, texture::DynamicTexture, RenderManager};
use scene::{Scene, SceneEvent, SceneInit};


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

pub struct Engine<'engine> {
    rm: RenderManager,
    last_rendered: Instant,
    dyn_texture_queue: Vec<&'engine DynamicTexture>,
    camera: Camera,
}

impl<'engine> Engine<'engine> {

    // pub fn create_window(title: &str, size: Size) -> (Window, EventLoop<()>) {
    //     let event_loop = EventLoop::new().expect("Could not create event loop!");
    //     let wb = WindowBuilder::new();
    //     let window = wb
    //     .with_title(title)
    //     .with_inner_size(Size::new(LogicalSize::new(800, 600)))
    //     .build(&event_loop)
    //     .expect("Could not create window!");

    //     (window, event_loop)
    // }

    pub fn new(window: &Arc<Window>) -> Self {

        let rm = RenderManager::new(window); 

        let mut engine = Self {
            rm,
            last_rendered: Instant::now(),
            dyn_texture_queue: vec![],
            camera: Camera::new(window.inner_size().width as f32, window.inner_size().height as f32),
        };

        engine
        
    }
    
    pub fn load_scene<S: SceneInit + Scene>(&mut self) -> Box<S>{
        Box::new(S::init(self))
    }

    /// Gets the delta time since last rendered 
    pub fn get_delta_time(&self) -> Duration {
        self.last_rendered.elapsed()
    }

    pub fn get_scene_object(&self, entity: &dyn ActiveEntity) -> Option<&SceneObject> {
        self.rm.get_scene_object(entity.get_sceneobject_handle())
    }

    pub fn event_loop(&mut self, scene: &mut Box<dyn Scene>, root_event: winit::event::Event<()>, ewt: &winit::event_loop::ActiveEventLoop) -> SceneEvent {
        let mut scene_event = SceneEvent::Continue;
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
                    scene_event = scene.update(self);

                    // Render
                    if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {
                        // println!("draw");
                        self.rm.gpu_state.update_resize();
                        self.rm.update_time();
                        self.rm.render(&self.camera);
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
        };
        scene_event
    }

}