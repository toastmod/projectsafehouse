
use std::rc::Rc;

use safehouse_render::{entity::Entity, gpu::{binding::Binder, buffer::VertexBuffer, program, shaderprogram::Program, texture::sampler::TextureSampler, wgpu}, model::ModelData, named_entity, texture::DynamicTexture, vertex_type::TexVertex};
struct TextPane {
    text_texture: DynamicTexture, 
    text_texture_sampler: Rc<TextureSampler> 
}

impl TextPane {
    pub fn get_dyn_texture(&self) -> &DynamicTexture {
        &self.text_texture
    }
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

    fn load_model(state: &crate::gpu::State) -> ModelData {
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
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, 
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            },
            depth_stencil: None,
        })
    }

    fn load_shader(rm: &safehouse_render::RenderManager, group_model: u32, group_entity: u32) -> Option<crate::gpu::shaderprogram::Program> {
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