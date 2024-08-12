use std::time::{Duration, Instant};

use safehouse_gpu::{buffer::{Buffer, UniformPtr}, program, shaderprogram::Program, text, vertex::Vertex, winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder}};

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

fn main() {
    let window_size = LogicalSize::new(800f64, 600f64);
    let event_loop = EventLoop::new().expect("Could not create window event loop.");

    let mut wb = WindowBuilder::new()
        .with_title("text example")
        .with_inner_size(window_size)
        .with_fullscreen(None);

    let window = wb.build(&event_loop).unwrap();

    let mut state = safehouse_gpu::State::new(&window);

    let mut text_rend = safehouse_gpu::text::TextRenderState::new(&state, include_bytes!("./ttf/VeraMono.ttf"), "testing testing 123");

    let mut inst = Instant::now();

    let mut last_rendered = Instant::now();

    let window_ref = &window;
    
    event_loop.run(move |loop_event, ewt| {
        match loop_event {
            winit::event::Event::WindowEvent { window_id, event } => {
                match event {
                    winit::event::WindowEvent::Resized(newsize) => {
                        state.set_resize(newsize.width, newsize.height);
                    },
                    winit::event::WindowEvent::Destroyed => ewt.exit(),
                    winit::event::WindowEvent::CloseRequested => ewt.exit(),
                    winit::event::WindowEvent::RedrawRequested => {

                        state.update_resize();


                        if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {

                            text_rend.prepare(&state);

                            let mut cmd = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                            let surfacetexture = state.surface.get_current_texture().unwrap();
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
                                text_rend.render(&mut renderpass);

                            }
                            state.queue.submit(std::iter::once(cmd.finish()));
                            surfacetexture.present();
                            text_rend.trim();
                            last_rendered = Instant::now();
                            // println!("draw") 
                        
                        }
                        window_ref.request_redraw();
                    }
                    _ => ()
                }
            },

            _ => ()
        }
    }).expect("Event loop error occured.");

}