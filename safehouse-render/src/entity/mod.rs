use std::{marker::PhantomData, num::NonZeroU32, rc::Rc};
use crate::{gpu, model::{ModelData, ModelDataRes}, resource::ManagerResource, scene::{SceneObject, SceneObjectHandle}};
use gpu::wgpu;
use safehouse_gpu::{binding::{Bindable, Binder}, shaderprogram::Program};

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
    Search(&'static str),
    LoadNew(T),
    Reference(Rc<T>)
}

pub trait Entity {

    const ENTITY_TYPE_NAME: &'static str;

    // Entity instance functions

    /// Functionality for when an Entity instance is created.
    /// Note: The Entity bindgroup is created after this call.
    /// Therefore this call only initializes the Entity on the CPU-side, although buffers can be written to at this point anyway. 
    fn on_instantiate(rm: &mut crate::RenderManager<'_>, handle: SceneObjectHandle) -> Self;

    // Data Loading
    fn load_bindings<'a>() -> Vec<Binder<Self>> where Self: Sized;
    fn load_model(state: &gpu::State) -> ModelData;
    fn load_pipeline(rm: &crate::RenderManager) -> Option<EntityPipeline>;

    // TODO: return an array of specifically Vertex and Fragment references
    fn load_shader(rm: &crate::RenderManager, group_model: u32, group_entity: u32) -> Option<gpu::shaderprogram::Program>;
    fn bindings_name() -> &'static str;
    fn model_name() -> &'static str;
    fn pipeline_name() -> &'static str;
    fn shader_name() -> &'static str;
}
