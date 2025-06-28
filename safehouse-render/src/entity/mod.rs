use std::{marker::PhantomData, num::NonZeroU32, rc::Rc};
use crate::{gpu, model::{ModelData, ModelDataRes}, resource::ManagerResource, scene::{SceneObject, SceneObjectHandle}};
use gpu::wgpu;
use safehouse_gpu::{binding::{Bindable, Binder}, shaderprogram::Program};
use constcat::*;

pub const BINDS_STR: &'static str = "_binds";
pub const MODEL_STR: &'static str = "_model";
pub const MODEL_BGL_STR: &'static str = "_model_bglayout";
pub const MODEL_BG_STR: &'static str = "_model_bindgroup";
pub const PIPE_STR: &'static str = "_pipe";
pub const SHADER_STR: &'static str = "_shader";

pub struct EntityPipeline {
            pub primitive: wgpu::PrimitiveState,
            pub depth_stencil: Option<wgpu::DepthStencilState>,
            // TODO: Multisample and Multiview support
            // pub multisample: wgpu::MultisampleState,
            // pub multiview: Option<NonZeroU32>
}

pub enum EntityShaderEntry {
    Separate { vertex: LoadTimeResource<Program>, fragment: LoadTimeResource<Program> },
    Combined(LoadTimeResource<Program>)
}

// TODO: implement resource load error type
pub enum LoadTimeResource<T: ManagerResource> {
    /// Attempt to load the default for this type, if there is one.
    UseDefault,

    /// Search by name for this resource.
    Search(&'static str),

    /// Provide new resource data to the manager.
    LoadNew(T),

    /// Provide an existing resource by it's owned reference.
    Reference(Rc<T>)
}

#[macro_export]
macro_rules! gen_const {
    ($t:ident, $postfix:ident) => {
        const {unsafe{&std::str::from_utf8_unchecked(&const {
            let mut s: [u8; $t::ENTITY_TYPE_NAME.as_bytes().len()+$postfix.as_bytes().len()+1] = [0u8; $t::ENTITY_TYPE_NAME.as_bytes().len()+$postfix.as_bytes().len()+1];

            let mut i = 0usize;

            while i < s.len() {
                if i < $t::ENTITY_TYPE_NAME.as_bytes().len() {
                    s[i] = $t::ENTITY_TYPE_NAME.as_bytes()[i];
                }else if i >= $t::ENTITY_TYPE_NAME.as_bytes().len() && i-$t::ENTITY_TYPE_NAME.as_bytes().len() < $postfix.as_bytes().len() {
                    s[i] = $postfix.as_bytes()[i-$t::ENTITY_TYPE_NAME.as_bytes().len()];
                }
                i+=1;
            }

            s 
        })}}
    };
}
#[macro_export]
macro_rules! named_entity {
    ($t:ident) => {
        use safehouse_render::entity::{NamedEntity, BINDS_STR, MODEL_STR, PIPE_STR, SHADER_STR, MODEL_BG_STR, MODEL_BGL_STR};
        use safehouse_render::gen_const;
        impl NamedEntity for $t {
            fn bindings_name() -> &'static str {
                gen_const!($t, BINDS_STR)
            }
        
            fn model_name() -> &'static str {
                gen_const!($t, MODEL_STR)
            }
        
            fn pipeline_name() -> &'static str {
                gen_const!($t, PIPE_STR)
            }
        
            fn shader_name() -> &'static str {
                gen_const!($t, SHADER_STR)
            }

            fn model_bindgroup_name() -> &'static str {
                gen_const!($t, MODEL_BG_STR)
            }

            fn model_bglayout_name() -> &'static str {
                gen_const!($t, MODEL_BGL_STR)
            }
        }
    };
}



pub trait Entity {

    const ENTITY_TYPE_NAME: &'static str; 

    // Entity instance functions

    /// Functionality for when an Entity instance is created.
    /// Note: The Entity bindgroup is created after this call.
    /// Therefore this call only initializes the Entity on the CPU-side, although buffers can be written to at this point anyway. 
    fn on_instantiate(rm: &mut crate::RenderManager, handle: SceneObjectHandle) -> Self;

    // Data Loading
    fn load_bindings<'a>() -> Vec<Binder<Self>> where Self: Sized;
    fn load_model(state: &gpu::State) -> ModelData;
    fn load_pipeline(rm: &crate::RenderManager) -> Option<EntityPipeline>;

    // TODO: return an array of specifically Vertex and Fragment references
    fn load_shader(rm: &crate::RenderManager, group_model: u32, group_entity: u32) -> Option<gpu::shaderprogram::Program>;

    // TODO: use ENTITY_TYPE_NAME to generate these strings, use an external const fn?
}

pub trait NamedEntity {
    fn bindings_name() -> &'static str;
    fn model_name() -> &'static str;
    fn pipeline_name() -> &'static str;
    fn shader_name() -> &'static str;
    fn model_bindgroup_name() -> &'static str;
    fn model_bglayout_name() -> &'static str;

}

impl NamedEntity for () {
    fn bindings_name() -> &'static str {
        "?_bind"
    }

    fn model_name() -> &'static str {
        "?_model"
    }

    fn pipeline_name() -> &'static str {
        "?_pipe"
    }

    fn shader_name() -> &'static str {
        "?_shader"
    }

    fn model_bindgroup_name() -> &'static str {
        "?_model_bg"
    }

    fn model_bglayout_name() -> &'static str {
        "?_model_bglayout"
    }
}

