use std::time::{Duration, Instant};

use safehouse_gpu::{buffer::{Buffer, UniformPtr}, program, vertex::Vertex, winit::{dpi::LogicalSize, event_loop::EventLoop, window::{Window, WindowBuilder}}};
use wgpu::rwh::HasDisplayHandle;

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
        .with_title("hello triangle")
        .with_inner_size(window_size)
        .with_fullscreen(None);

    let window = wb.build(&event_loop).unwrap();

    let mut state = safehouse_gpu::State::new(&window);

    let debug = safehouse_debug::SafehouseDebugger::new(&window);

    let vb = safehouse_gpu::buffer::VertexBuffer::new(&state, &[
        ColorVertex::new([0.0,0.5,0.0],[1.0,0.0,0.0]),
        ColorVertex::new([0.5,-0.5,0.0],[0.0,1.0,0.0]),
        ColorVertex::new([-0.5,-0.5,0.0],[0.0,0.0,1.0]),
    ]);

    let prog = state.add_shader("color_shader", program!(
        &state,
        source: "

        struct ColorVertexInput {
            @location(0) pos: vec3<f32>,
            @location(1) color: vec3<f32>,
        }

        struct ColorVertexOutput {
            @builtin(position) pos: vec4<f32>,
            @location(0) color: vec4<f32>,
        }

        @binding(0) @group(0)
        var<uniform> wobble: f32;

        @vertex
        fn vs_main(i: ColorVertexInput) -> ColorVertexOutput {
            var o: ColorVertexOutput;
            o.pos = vec4<f32>(i.pos.x, i.pos.y+wobble, i.pos.z, 1.0);
            o.color = vec4<f32>(i.color, 1.0);
            return o;
        }

        @fragment
        fn fs_main(iv: ColorVertexOutput) -> @location(0) vec4<f32> {
            return iv.color;
        }
        "
    ));


    let pipeline = state.add_render_pipeline("triangle_pipeline", &wgpu::RenderPipelineDescriptor { 
        label: None, 
        layout: None, 
        vertex: wgpu::VertexState {
            module: &prog.module,
            entry_point: "vs_main",
            buffers: &[ColorVertex::desc().clone()],
        }, 
        primitive: wgpu::PrimitiveState { 
            topology: wgpu::PrimitiveTopology::TriangleList, 
            strip_index_format: None, 
            front_face: wgpu::FrontFace::Cw, 
            polygon_mode: wgpu::PolygonMode::Fill, 
            cull_mode: Some(wgpu::Face::Back), 
            unclipped_depth: false, 
            conservative: false 
        }, 
        depth_stencil: None, 
        multisample: wgpu::MultisampleState {
            ..Default::default()
        }, 
        fragment: Some(
            wgpu::FragmentState {
                module: &prog.module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { format: state.config.format, blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })],
            }
        ), 
        multiview: None 
    });

    let mut wobble = safehouse_gpu::buffer::UniformPtr::new(&state, 0f32);

    let bindgroup = state.init_bindgroup_from_pipeline(Some("triangle_pipeline"), 0, &[
        wgpu::BindGroupEntry {
            binding: 0,
            resource: wobble.get_buffer().as_entire_binding(),
        }
    ]).expect("Could not create bindgroup!");

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

                        if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {

                            *wobble.as_mut() = (inst.elapsed().as_secs_f32()).sin()*0.1;
                            wobble.update(&state);

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
                                renderpass.set_vertex_buffer(0, vb.buffer.slice(..));
                                renderpass.set_bind_group(0, &bindgroup, &[]);
                                renderpass.set_pipeline(pipeline.as_ref());
                                renderpass.draw(0..3, 0..1);
                            }
                            state.queue.submit(std::iter::once(cmd.finish()));
                            surfacetexture.present();
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