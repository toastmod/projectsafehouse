use std::{collections::VecDeque, rc::Rc, thread::park_timeout_ms, time::{Duration, Instant}};
use safehouse_gpu::{binding::Binder, buffer::VertexBuffer, program, texture::sampler::TextureSampler};
use safehouse_render::{camera::Camera, entity::*, named_entity, texture::{DynamicTexture, DynamicTextureHandle, TextureType}, vertex_type::TexVertex};
pub use safehouse_render as render;
use render::{RenderManager, gpu::{winit, wgpu, shaderprogram::Program}, model::ModelData, vertex_type::ColorVertex};
use winit::{dpi::{LogicalSize, Size}, event_loop::EventLoop };
use winit_app_handler::{WinitApp, WinitState};

struct TextPane {
    dyn_texture_handle: DynamicTextureHandle,
    dyn_texture_ref: Rc<safehouse_gpu::texture::Texture>,
    text_texture_sampler: Rc<TextureSampler> 
}

impl TextPane {

}

impl Entity for TextPane {
    const ENTITY_TYPE_NAME: &'static str = "TextPane";

    fn on_instantiate(rm: &mut safehouse_render::RenderManager, handle: safehouse_render::scene::SceneObjectHandle) -> Self {
        // Create a new DynmaicTexture
        let mut text_texture = DynamicTexture::new_text(rm, wgpu::Color::TRANSPARENT, "This is some text, can you see it?");
        text_texture.prepare(rm);
        let dyn_texture_handle = rm.add_dyn_texture(text_texture);
        let dyn_texture_ref = Rc::clone(&rm.get_dyn_texture(dyn_texture_handle).unwrap().texture);
        rm.queue_dyn_texture(dyn_texture_handle);
        Self {
            dyn_texture_handle,
            dyn_texture_ref,
            text_texture_sampler: Rc::clone(rm.gpu_state.get_sampler("default")),
        } 
    }

    fn load_bindings<'a>() -> Vec<Binder<Self>> where Self: Sized {
        vec![
            Binder::new(0, crate::render::gpu::wgpu::ShaderStages::all(), &|x| { x.dyn_texture_ref.as_ref() } ),
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

struct TextTextureExample {
    rm: RenderManager,
    camera: Camera,
    pane: TextPane,
    last_rendered: Instant,
}

impl WinitApp for TextTextureExample {
    type UserEvent = ();

    fn on_start(window: &std::sync::Arc<winit::window::Window>) -> Self {

        let mut rm = RenderManager::new(&window); 

        let mut camera = Camera::new(800f32, 600f32);

        rm.load_entity::<TextPane>();
        let mut pane = rm.spawn_sceneobject_entity::<TextPane>("TextPane");

        let mut last_rendered = Instant::now();

        Self {
            rm,
            camera,
            pane,
            last_rendered,
        }
    }

    fn on_contstructed(&mut self, window: &std::sync::Arc<winit::window::Window>) {
        
    }

    fn on_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::Resized(size) => self.rm.gpu_state.set_resize(size.width, size.height),
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Destroyed => event_loop.exit(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => {
                // pong.mouse_moved(&mut engine, position.x as f32, position.y as f32);
            },
            winit::event::WindowEvent::RedrawRequested => {
                if Instant::now().duration_since(self.last_rendered) >= Duration::from_millis(16) {
                    // println!("draw");
                    self.rm.gpu_state.update_resize();
                    self.rm.update_time();
                    self.rm.render(&self.camera);

                }
                window.request_redraw();
            }

            _ => (),
        }
    }

    fn on_device_event(&mut self, window: &std::sync::Arc<winit::window::Window>, event_loop: &winit::event_loop::ActiveEventLoop, event: winit::event::DeviceEvent) {
        todo!()
    }
}

fn main() {
    WinitState::<TextTextureExample>::run();
}
