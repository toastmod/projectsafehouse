use std::rc::Rc;
use crate::{gpu, model::ModelData, scene::{SceneObject, SceneObjectHandle}};
use gpu::wgpu;
use safehouse_gpu::buffer::Bindable;

/// A descriptor for an entity's custom pipeline.
pub enum EntityPipeline<'a> {
    Default,
    Custom {
        vertex: wgpu::VertexState<'a>,
        fragment: Option<wgpu::FragmentState<'a>>, 
        primitive: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
    }
} 

/// A layout entry specifically to describe the bindings of an entity.\
/// `'i` indicates the lifetime of instantiation.
// pub struct EntityLayoutEntry<'inactive, 'active, E> {
//     pub(crate) slot: u32,
//     pub(crate) f: &'inactive dyn Fn(&'active E) -> wgpu::BindGroupEntry<'active>,
//     pub(crate) layout_entry: wgpu::BindGroupLayoutEntry,
// }

// impl<'inactive, 'active, E> EntityLayoutEntry<'inactive, 'active, E> {
//     pub fn new<B: Bindable>(slot: u32, visibility: wgpu::ShaderStages, f: &'inactive dyn Fn(&'active E) -> wgpu::BindGroupEntry<'active>) -> Self {
//         Self {
//             layout_entry: B::get_layout_entry(slot, visibility),
//             slot,
//             f,
//         }
//     }
//     // pub (crate) fn to_bgentry(&self, new_entity: &'i E) -> wgpu::BindGroupEntry<'active> {
//     //     (self.f)(new_entity)
//     // }
// }



pub type EntityLayoutEntry<'inactive, 'active, E> = (u32, &'inactive dyn Fn(&'active E) -> wgpu::BindGroupEntry<'active>, wgpu::BindGroupLayoutEntry); 

pub fn entity_layout_entry<'inactive, 'active, E: Entity, B: Bindable>(slot: u32, visibility: wgpu::ShaderStages, f: &'inactive dyn Fn(&'active E) -> wgpu::BindGroupEntry<'active>) -> EntityLayoutEntry<'inactive, 'active, E> {
    (
        slot,
        f,
        B::get_layout_entry(slot, visibility),
    )
}

pub trait Entity {
    
    // Entity instance functions
    fn on_instantiate(rm: &mut crate::RenderManager<'_>, handle: SceneObjectHandle) -> Self;

    // Data Loading
    fn entity_type_name() -> &'static str;
    fn model_name() -> String {
        let mut s = String::new();
        s.push_str("model_");
        s.push_str(Self::entity_type_name());
        s
    }
    fn pipeline_name() -> String {
        let mut s = String::new();
        s.push_str("pipe_");
        s.push_str(Self::entity_type_name());
        s
    }

    // TODO: Vertex format and Vertex shader should be determined here
    fn load_model(rm: &mut crate::RenderManager) -> Option<Rc<ModelData>>;

    // TODO: only depth stencil and render options should go here
    fn load_pipeline<'a>(rm: &'a crate::RenderManager, shader_module: &'a wgpu::ShaderModule) -> EntityPipeline<'a>;

    // TODO: Fragment shader should be determined here 
    fn load_shader<'a>(rm: &'a crate::RenderManager) -> Option<gpu::shaderprogram::Program>;
    fn load_entity_bindings<'inactive, 'active>() -> Vec<EntityLayoutEntry<'inactive, 'active, Self>>;
}
