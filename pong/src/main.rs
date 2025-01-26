use safehouse_render::{camera::Camera, gpu::winit::window::Window};
pub use safehouse_render as render;
use winit_app_handler::{WinitApp, WinitState};
use std::{ops::Range, sync::Arc, time::{Duration, Instant}};
use ball::Ball;
use render::{entity::Entity, gpu::winit, model::ModelData, vertex_type::ColorVertex, RenderManager};
use winit::{dpi::{LogicalSize, Size}, event::KeyEvent, event_loop::EventLoop};

use paddle::Paddle;
use pong::Pong;

mod pong;
mod paddle;
mod ball;

pub fn map(x: f32, a: Range<f32>, b: Range<f32>) -> f32{
    let norm = (x-a.start)/(a.end-a.start);
    (x*(b.end-b.start)) + b.start
}

struct PongWindow {
    pong: Pong,
    window: Arc<Window>,
    rm: RenderManager,
    camera: Camera,
    last_rendered: Instant
}

impl WinitApp for PongWindow {
    type UserEvent = ();

    fn on_start(window: &std::sync::Arc<winit::window::Window>) -> Self {
        let mut rm = RenderManager::new(&window); 

        let mut camera = Camera::new(pong::SCREEN_WIDTH, pong::SCREEN_HEIGHT);

        camera.set_rot((-90.0f32.to_radians(), 0.0));
        camera.set_pos((0.0,0.0,-5.0));
        camera.upd8(true, 0);

        let mut pong = Pong::start(&mut rm);

        let mut last_rendered = Instant::now();
        Self {
            pong,
            window: Arc::clone(window),
            rm,
            camera,
            last_rendered
        }
    }

    fn on_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(size) => self.rm.gpu_state.set_resize(size.width, size.height),
            winit::event::WindowEvent::CloseRequested => {
                self.pong.stop(); 
                event_loop.exit();
            },
            winit::event::WindowEvent::Destroyed => event_loop.exit(),
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
                            if event.state.is_pressed() {
                                self.pong.state.reset(&mut self.rm);
                            }
                        },

                        _ => (),
                    },
                    winit::keyboard::PhysicalKey::Unidentified(_) => (),
                }
            },
            winit::event::WindowEvent::CursorMoved { device_id, position } => {
                let (x, y) = self.rm.window_to_world_coord(position.x as f32, position.y as f32);
                self.pong.mouse_moved(&mut self.rm, x, y);
            },
            winit::event::WindowEvent::RedrawRequested => {
                if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {
                    self.pong.update(&mut self.rm, self.last_rendered.elapsed());
                    self.rm.render(&self.camera);
                    self.last_rendered = Instant::now();
                }
                self.rm.window.request_redraw();
            }

            _ => (),
        }
    }

    fn on_device_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::DeviceEvent) {

    }
}

fn main() {
    // let event_loop = EventLoop::new().expect("Could not create event loop!");

    WinitState::<PongWindow>::run();

    // let window = wb
    // .with_title("Pong")
    // .with_inner_size(Size::new(LogicalSize::new(pong::SCREEN_WIDTH,pong::SCREEN_HEIGHT)))
    // .build(&event_loop)
    // .expect("Could not create window!");



    // let _ = event_loop.run(move |root_event, ewt|{
    //     match root_event {
    //         winit::event::Event::WindowEvent { window_id, event } => ,
    //         // engine::gpu::winit::event::Event::LoopExiting => todo!(),
    //         _ => ()
    //     }
    // });
}
