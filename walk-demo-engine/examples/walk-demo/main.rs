use safehouse_render::gpu::winit::dpi::LogicalSize;
use walk_demo_engine::{scene::{self, walking::WalkingScene, Scene}, Engine};
use winit_app_handler::{WinitApp, WinitState};

struct WalkDemo<'a> {
    engine: Engine<'a>,
    scene: Box<dyn Scene> 
}

impl<'a> WinitApp for WalkDemo<'a> {
    type UserEvent = ();

    fn on_start(window: &std::sync::Arc<safehouse_render::gpu::winit::window::Window>) -> Self {
        let mut engine = Engine::new(window); 
        Self {
            scene: engine.load_scene::<WalkingScene>(),
            engine,
        }
    }

    fn on_event(&mut self, window: &std::sync::Arc<safehouse_render::gpu::winit::window::Window>, event_loop: &safehouse_render::gpu::winit::event_loop::ActiveEventLoop, event: safehouse_render::gpu::winit::event::WindowEvent) {
        self.engine.event_loop(&mut self.scene, event, event_loop);
    }

    fn on_device_event(&mut self, window: &std::sync::Arc<safehouse_render::gpu::winit::window::Window>, event_loop: &safehouse_render::gpu::winit::event_loop::ActiveEventLoop, event: safehouse_render::gpu::winit::event::DeviceEvent) {
        self.engine.update_controller(event);
    }
}

fn main() {
    WinitState::<WalkDemo>::run();
}