use std::rc::Rc;

use crate::render::{BINDGROUP_GLOBAL, BINDGROUP_SCENEOBJECT, entity::{Entity, EntityPipeline}, gpu::{self, binding::Binder, buffer::{Uniform, VertexBuffer}, dataunit::ImageFormat, program, shaderprogram::Program, texture::{sampler::TextureSampler, Texture}, wgpu::{self, PrimitiveState, ShaderStages}}, model::{ModelData, ModelResources}, named_entity, scene::SceneObjectHandle, texturetype::TextureType, vertex_type::TexVertex };

use super::ActiveEntity;

pub struct Bunny {
    pub handle: SceneObjectHandle
}

impl ActiveEntity for Bunny {
    fn get_sceneobject_handle(&self) -> SceneObjectHandle {
        self.handle
    }
}

impl Entity for Bunny {
    const ENTITY_TYPE_NAME: &'static str = "Bunny";

    fn on_instantiate(rm: &mut safehouse_render::RenderManager, handle: SceneObjectHandle) -> Self {
        Bunny { handle }
    }

    fn load_bindings<'a>() -> Vec<safehouse_render::gpu::binding::Binder<Self>> where Self: Sized {
        vec![]
    }

    fn load_model(state: &safehouse_render::gpu::State) -> safehouse_render::model::ModelData {
        let data = include_bytes!("../model/bunny.dat");

        struct BunnyModelRes {
            texture: Texture,
            sampler: Rc<TextureSampler>
        }

        impl ModelResources for BunnyModelRes {
            fn model_bindings() -> Vec<Binder<Self>> where Self: Sized {
                vec![
                    Binder::<BunnyModelRes>::new(0, wgpu::ShaderStages::all(), &|x| &x.texture),
                    Binder::<BunnyModelRes>::new(1, wgpu::ShaderStages::all(), &|x| x.sampler.as_ref()),
                ]
            }
        }

        ModelData::new::<Self,BunnyModelRes>(
            state,
            VertexBuffer::new_from_raw::<TexVertex>(state, data),
            vec![0..(data.len()/std::mem::size_of::<TexVertex>()) as u32],
            Some(BunnyModelRes {
                texture: Texture::load_encoded(state, include_bytes!("../../res/obj/bunny/buntex.1001.png"), gpu::dataunit::ImageFormat::Png),
                sampler: Rc::clone(&state.get_sampler("default"))
            })
        )

    }

    fn load_pipeline(rm: &safehouse_render::RenderManager) -> Option<safehouse_render::entity::EntityPipeline> {
        Some(EntityPipeline {
            primitive: PrimitiveState::default(),
            depth_stencil: None,
        })
    }

    fn load_shader(rm: &safehouse_render::RenderManager, group_model: u32, group_entity: u32) -> Option<safehouse_render::gpu::shaderprogram::Program> {
        Some(program!(
            &rm.gpu_state,
            source: format!("
            @group({BINDGROUP_GLOBAL}) @binding(0)
            var<uniform> pvm: mat4x4<f32>;
            @group({BINDGROUP_GLOBAL}) @binding(1)
            var<uniform> time: f32;

            @group({BINDGROUP_SCENEOBJECT}) @binding(0)
            var<uniform> obj_mat: mat4x4<f32>;

            @group({group_model}) @binding(0)
            var texture: texture_2d<f32>;
            @group({group_model}) @binding(1)
            var tex_sampler: sampler;


            struct TexVertexIn {{
                @location(0) pos: vec4<f32>,
                @location(1) tcoord: vec2<f32>,
            }}

            struct TexVertexOut {{
                @builtin(position) pos: vec4<f32>,
                @location(0) tcoord: vec2<f32>,
            }}

            @vertex
            fn vs_main(in: TexVertexIn) -> TexVertexOut {{
                var out: TexVertexOut;
                out.pos = pvm * in.pos;
                return out;
            }}

            @fragment
            fn fs_main(vo: TexVertexOut) -> @location(0) vec4<f32> {{
                return textureSample(texture, tex_sampler, vo.tcoord); 
            }}


            ")
        ))
    }

}

named_entity!(Bunny);
