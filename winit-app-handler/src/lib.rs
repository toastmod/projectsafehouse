use std::marker::PhantomData;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes};

pub trait WinitApp {
    type UserEvent; 
    fn on_start(window: &Arc<Window>) -> Self;
    fn on_contstructed(&mut self, window: &Arc<Window>) {

    }
    fn on_event(&mut self, window: &Arc<Window>, event_loop: &ActiveEventLoop, event: WindowEvent);
    fn on_device_event(&mut self, window: &Arc<Window>, event_loop: &ActiveEventLoop, event: DeviceEvent);
}

pub enum WinitState<App: WinitApp> {
    Loading,
    Running {
        app: App,
        window: Arc<Window>
    }
}

impl<App: WinitApp> WinitState<App> {
    pub fn run() {
        let mut event_loop = winit::event_loop::EventLoop::new().expect("Could not create event loop!");
        event_loop.run_app(&mut Self::Loading);
    }
}

impl<App: WinitApp> ApplicationHandler for WinitState<App> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Loading = self {
            let mut window = Arc::new(event_loop.create_window(Default::default()).expect("Could not create window!")); 
            let mut app = App::on_start(&window); 
            *self = Self::Running {
                window,
                app
            };
        } else {
            panic!("Already running! Window context recreation not yet supported!")
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Self::Running { window, app } = self {
            app.on_event(window, event_loop, event);
        }
    }
    
    fn new_events(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, cause: winit::event::StartCause) {
        let _ = (event_loop, cause);
    }
    
    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {
        let _ = (event_loop, event);
    }
    
    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let _ = (event_loop, device_id, event);
    }
    
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
    
    fn suspended(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
    
    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
    
    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let _ = event_loop;
    }
}

