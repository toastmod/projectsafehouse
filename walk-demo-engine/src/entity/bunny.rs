use safehouse_render::{entity::{Entity, EntityPipeline}, gpu::{wgpu, shaderprogram::Program, binding::Binder, buffer::VertexBuffer, program, wgpu::{PrimitiveState, ShaderStages}}, model::ModelData, named_entity, vertex_type::TexVertex};

pub struct Bunny {

}

impl Entity for Bunny {
    const ENTITY_TYPE_NAME: &'static str = "Bunny";

    fn on_instantiate(rm: &mut safehouse_render::RenderManager<'_>, handle: safehouse_render::scene::SceneObjectHandle) -> Self {
        Bunny {}
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

            @vertex
            fn vs_main() {{

            }}

            ")
        ))
    }

}

named_entity!(Bunny);
