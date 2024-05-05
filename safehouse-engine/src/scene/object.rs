use std::rc::Rc;
use crate::model::ModelData;
use crate::gpu::wgpu;
pub struct SceneObject {
    pub name: String,
    pub model_data: Rc<ModelData>,
    pub pipeline_ref: Option<Rc<wgpu::RenderPipeline>>,
    pub entity_bindgroup: Option<Rc<wgpu::BindGroup>>,
    pub model_matrix: ()
}