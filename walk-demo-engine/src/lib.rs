pub mod entity;
pub mod scene;

use std::{sync::Arc, time::{Duration, Instant}};

use entity::ActiveEntity;
use safehouse_render::{camera::Camera, controller::Controller, gpu::winit::event::DeviceEvent, scene::{ControllerHandle, SceneObject}};
pub use safehouse_render as render;
pub use safehouse_render::gpu as gpu;
use render::{gpu::winit::{self, window::{Window}}, texturetype::DynamicTexture, RenderManager};
use scene::{Scene, SceneEvent, SceneInit};


pub struct Engine<'engine> {
    rm: RenderManager,
    last_rendered: Instant,
    dyn_texture_queue: Vec<&'engine DynamicTexture>,
    camera: Camera,
    controller: Controller,
    pmousex: f64,
    pmousey: f64,
}

impl<'engine> Engine<'engine> {

    pub fn new(window: &Arc<Window>) -> Self {

        let rm = RenderManager::new(window);         
        rm.gpu_state.print_info();

        let controller = Controller::new(None); 
        let mut camera = Camera::new(window.inner_size().width as f32, window.inner_size().height as f32);
        camera.upd8(true, 0);

        let engine = Self {
            rm,
            last_rendered: Instant::now(),
            dyn_texture_queue: vec![],
            camera,
            controller,
            pmousex: 0.0,
            pmousey: 0.0
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

    pub fn update_controller(&mut self, event: DeviceEvent) {
        self.controller.device_input(event, (self.rm.gpu_state.config.width as f32, self.rm.gpu_state.config.height as f32), 10.0);
    }

    pub fn event_loop(&mut self, scene: &mut Box<dyn Scene>, window_event: winit::event::WindowEvent, ewt: &winit::event_loop::ActiveEventLoop) -> SceneEvent {
        let mut scene_event = SceneEvent::Continue;

        match window_event {
                winit::event::WindowEvent::Resized(size) => self.rm.gpu_state.set_resize(size.width, size.height),
                winit::event::WindowEvent::CloseRequested => ewt.exit(),
                winit::event::WindowEvent::Destroyed => ewt.exit(),
                winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    // let dx = position.x - self.pmousex;
                    // let dy = position.y - self.pmousey;
                    // self.controller.mouse_move((dx as f32,dy as f32));
                },
                winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    self.controller.keyboard_input(event.logical_key, event.state.is_pressed());
                },
                winit::event::WindowEvent::MouseInput { device_id, state, button } => {
                    self.controller.mouse_input(button, state.is_pressed());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    // Update
                    scene_event = scene.update(self);

                    // Render
                    if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {
                        // println!("draw");
                        self.rm.gpu_state.update_resize();
                        self.rm.render(&self.camera);
                        self.rm.update_time();
                        if !self.dyn_texture_queue.is_empty() {
                            self.dyn_texture_queue.clear();
                        }
                    }
                    self.rm.window.request_redraw();
                }

                _ => (),
        };

        scene_event
    }

}