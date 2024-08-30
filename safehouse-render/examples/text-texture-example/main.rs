use std::{collections::VecDeque, rc::Rc, thread::park_timeout_ms, time::{Duration, Instant}};
use safehouse_gpu::{binding::Binder, buffer::VertexBuffer, program, texture::sampler::TextureSampler};
use safehouse_render::{entity::*, named_entity, texture::{DynamicTexture, TextureType}, vertex_type::TexVertex};
pub use safehouse_render as render;
use render::{RenderManager, gpu::{winit, wgpu, shaderprogram::Program}, model::ModelData, vertex_type::ColorVertex};
use winit::{dpi::{LogicalSize, Size}, event_loop::EventLoop, window::WindowBuilder};

struct TextPane {
    text_texture: DynamicTexture, 
    text_texture_sampler: Rc<TextureSampler> 
}

impl TextPane {

}

impl Entity for TextPane {
    const ENTITY_TYPE_NAME: &'static str = "TextPane";

    fn on_instantiate(rm: &mut safehouse_render::RenderManager<'_>, handle: safehouse_render::scene::SceneObjectHandle) -> Self {
        Self {
            text_texture: DynamicTexture::new_text(rm, wgpu::Color::TRANSPARENT, "This is some text, can you see it?"),
            text_texture_sampler: Rc::clone(rm.gpu_state.get_sampler("default")),
        } 
    }

    fn load_bindings<'a>() -> Vec<Binder<Self>> where Self: Sized {
        vec![
            Binder::new(0, crate::render::gpu::wgpu::ShaderStages::all(), &|x| { &x.text_texture } ),
            Binder::new(1, crate::render::gpu::wgpu::ShaderStages::all(), &|x| { x.text_texture_sampler.as_ref() } )
        ]
    }

    fn load_model(state: &safehouse_gpu::State) -> ModelData {
        ModelData {
            vertex_buffer: VertexBuffer::new(state, &[
                
                TexVertex { pos: [1.0,1.0,0.0,1.0], tex_coord: [1.0,0.0]},
                TexVertex { pos: [-1.0,1.0,0.0,1.0], tex_coord: [0.0,0.0]},
                TexVertex { pos: [-1.0,-1.0,0.0,1.0], tex_coord: [0.0,1.0]},

                TexVertex { pos: [-1.0,-1.0,0.0,1.0], tex_coord: [0.0,1.0]},
                TexVertex { pos: [1.0,-1.0,0.0,1.0], tex_coord: [1.0,1.0]},
                TexVertex { pos: [1.0,1.0,0.0,1.0], tex_coord: [1.0,0.0]},

            ]),
            textures: None,
            model_bindgroup: None,
            groups: Box::new([0..6]),
        }
    
    }

    fn load_pipeline(rm: &safehouse_render::RenderManager) -> Option<safehouse_render::entity::EntityPipeline> {
        Some(safehouse_render::entity::EntityPipeline{
            primitive: safehouse_gpu::wgpu::PrimitiveState { 
                topology: safehouse_gpu::wgpu::PrimitiveTopology::TriangleList, 
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
            depth_stencil: None,
        })
    }

    fn load_shader(rm: &safehouse_render::RenderManager, group_model: u32, group_entity: u32) -> Option<safehouse_gpu::shaderprogram::Program> {
        Some(
            program!(
                &rm.gpu_state,
                source: format!("

                @group({group_entity}) @binding(0)
                var dyntexture: texture_2d<f32>;

                @group({group_entity}) @binding(1)
                var dyntex_sampler: sampler;

                struct TextureVertexInput {{
                    @location(0) pos: vec4<f32>,
                    @location(1) tex_coord: vec2<f32>,
                }}

                struct TextureVertexOutput {{
                    @builtin(position) pos: vec4<f32>,
                    @location(0) tex_coord: vec2<f32>,
                }}

                @vertex
                fn vs_main(i: TextureVertexInput) -> TextureVertexOutput {{
                    var o: TextureVertexOutput;
                    o.pos = i.pos;
                    o.tex_coord = i.tex_coord;
                    return o;
                }}

                @fragment
                fn fs_main(iv: TextureVertexOutput) -> @location(0) vec4<f32> {{
                    return textureSample(dyntexture, dyntex_sampler, iv.tex_coord);
                }}
                
                ")
            )
        )
    }

}

named_entity!(TextPane);

fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop!");
    let wb = WindowBuilder::new();
    let window = wb
    .with_title("Text Texture Example")
    .with_inner_size(Size::new(LogicalSize::new(800,600)))
    .build(&event_loop)
    .expect("Could not create window!");

    let mut rm = RenderManager::new(&window); 

    rm.load_entity::<TextPane>();
    let mut pane = rm.spawn_sceneobject_entity::<TextPane>("TextPane");
    pane.text_texture.prepare(&mut rm);

    let mut dyn_texture_queue = vec![
        &pane.text_texture
    ];

    let mut last_rendered = Instant::now();

    let _ = event_loop.run(move |root_event, ewt|{
        match root_event {
            winit::event::Event::WindowEvent { window_id, event } => match event {
                winit::event::WindowEvent::Resized(size) => rm.gpu_state.set_resize(size.width, size.height),
                winit::event::WindowEvent::CloseRequested => ewt.exit(),
                winit::event::WindowEvent::Destroyed => ewt.exit(),
                winit::event::WindowEvent::CursorMoved { device_id, position } => {
                    // pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
                },
                winit::event::WindowEvent::RedrawRequested => {
                    if Instant::now().duration_since(last_rendered) >= Duration::from_millis(16) {
                        // println!("draw");
                        rm.gpu_state.update_resize();
                        rm.update_time();
                        rm.render(dyn_texture_queue.as_slice());
                        if !dyn_texture_queue.is_empty() {
                            dyn_texture_queue.clear();
                        }
                    }
                    rm.window.request_redraw();
                }

                _ => (),
            },
            // engine::gpu::winit::event::Event::LoopExiting => todo!(),
            _ => ()
        }
    });
}
