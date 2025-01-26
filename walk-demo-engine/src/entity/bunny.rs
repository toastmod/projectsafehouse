use safehouse_render::{entity::{Entity, EntityPipeline}, gpu::{binding::Binder, buffer::VertexBuffer, program, shaderprogram::Program, wgpu::{self, PrimitiveState, ShaderStages}}, model::ModelData, named_entity, scene::SceneObjectHandle, vertex_type::TexVertex};

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
        vec![
            // Binder::<Self>::new(0, ShaderStages::VERTEX_FRAGMENT, |x| )
        ]
    }

    fn load_model(state: &safehouse_render::gpu::State) -> safehouse_render::model::ModelData {
        let data = include_bytes!("../model/bunny.dat");
        ModelData {
            vertex_buffer: VertexBuffer::new_from_raw::<TexVertex>(state, data),
            textures: None,
            model_bindgroup: None,
            groups: Box::new([0..(data.len()/std::mem::size_of::<TexVertex>()) as u32]),
        }
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
                out.pos = in.pos;
                return out;
            }}

            @fragment
            fn fs_main(vo: TexVertexOut) -> @location(0) vec4<f32> {{
                return vec4<f32>(1.0,0.0,0.0,1.0); 
            }}


            ")
        ))
    }

}

named_entity!(Bunny);
