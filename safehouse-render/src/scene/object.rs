use std::rc::Rc;
use safehouse_gpu::buffer::UniformPtr;

use crate::model::ModelData;
use crate::gpu::wgpu;
pub struct SceneObject {
    pub name: String,
    pub model_data: Rc<ModelData>,
    pub pipeline_ref: Option<Rc<wgpu::RenderPipeline>>,
    pub sceneobject_bindgroup: Rc<wgpu::BindGroup>,
    pub entity_bindgroup: Option<Rc<wgpu::BindGroup>>,
    pub model_matrix: UniformPtr<glam::Mat4>,
    pub(crate) model_matrix_changed: bool,  
}

impl SceneObject {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn attach_entity_bindgroup(&mut self, entity_bindgroup: Rc<wgpu::BindGroup>) {
        self.entity_bindgroup = Some(entity_bindgroup);
    }

    pub fn transform_mut(&mut self) -> &mut glam::Mat4 {
        self.model_matrix.as_mut()
    }

    pub fn transform_ref(&self) -> &glam::Mat4 {
        self.model_matrix.as_ref()
    }

    pub(crate) fn update_matrix(&self, state: &safehouse_gpu::State) {
        self.model_matrix.update(state);
    }
}