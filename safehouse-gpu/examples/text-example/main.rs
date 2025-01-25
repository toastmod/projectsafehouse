use std::time::{Duration, Instant};

use safehouse_gpu::{buffer::{Buffer, UniformPtr}, program, shaderprogram::Program, text, vertex::Vertex, winit::{dpi::LogicalSize, event_loop::EventLoop }, State, TextRenderState};
use winit_app_handler::{WinitApp, WinitState};

#[repr(C)]
#[derive(Debug,Clone,Copy,Default)]
struct ColorVertex {
    pos: [f32; 3],
    color: [f32;3]
}

impl ColorVertex {
    pub fn new(pos: [f32;3], color: [f32;3]) -> Self {
        Self {
            pos,
            color,
        }
    }
}

impl Vertex for ColorVertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static> {
        &wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<ColorVertex>() as u64, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<f32>() as u64 * 3u64,
                    shader_location: 1,
                }
            ] 
        }
    }
}

struct TextExample {
    state: State,
    text_rend: TextRenderState,
    inst: Instant,
    last_rendered: Instant
}

impl WinitApp for TextExample {
    type UserEvent = ();

    fn on_start(window: &std::sync::Arc<winit::window::Window>) -> Self {

        let mut state = safehouse_gpu::State::new(&window);

        let mut text_rend = safehouse_gpu::text::TextRenderState::new(&state, include_bytes!("./ttf/VeraMono.ttf"), "testing testing 123");

        let mut inst = Instant::now();

        let mut last_rendered = Instant::now();

        Self {
            state,
            text_rend,
            inst,
            last_rendered
        }
        
    }

    fn on_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(newsize) => {
                self.state.set_resize(newsize.width, newsize.height);
            },
            winit::event::WindowEvent::Destroyed => event_loop.exit(),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {

                self.state.update_resize();


                if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {

                    self.text_rend.prepare(&self.state);

                    let mut cmd = self.state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    let surfacetexture = self.state.surface.get_current_texture().unwrap();
                    {
                        let view = surfacetexture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let mut renderpass = cmd.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[
                                Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations{
                                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                    store: wgpu::StoreOp::Store,
                                } })
                            ],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                        self.text_rend.render(&mut renderpass);

                    }
                    self.state.queue.submit(std::iter::once(cmd.finish()));
                    surfacetexture.present();
                    self.text_rend.trim();
                    self.last_rendered = Instant::now();
                    // println!("draw") 
                
                }
                window.request_redraw();
            }
            _ => ()
        }
            
    }
    
    fn on_device_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::DeviceEvent) {
        ()
    }
}

fn main() {
    WinitState::<TextExample>::run(); 
}