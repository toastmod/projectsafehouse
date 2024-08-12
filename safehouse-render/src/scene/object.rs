use std::rc::Rc;
use safehouse_gpu::buffer::{UniformPtr};

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

    // pub fn init_entity_bindgroup<'a>(&mut self, rm: &mut crate::RenderManager, bindings: &'a[EntityBinding<'a>]) {

    //     // TODO: check if layout already exists, in the current form layouts are duplicate
    //     let (layout_entries, bg_entries): (Vec<wgpu::BindGroupLayoutEntry>, Vec<wgpu::BindGroupEntry>) = bindings.iter().enumerate().map(|(i,x)| {
    //         (x.object_layout_entry, x.object_binding.get_binding_entry(x.binding))
    //     }).unzip();

    //     let layout = Rc::new(rm.gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    //         label: None,
    //         entries: &layout_entries,
    //     }));

    //     rm.entity_bglayout_cache.insert(String::from("PLACEHOLDER_ENTITY_NAME_entitylayout"), Rc::clone(&layout));

    //     let mut bgname = self.name.clone();
    //     bgname.push_str("_bg");
        
    //     let bg = Rc::new(rm.gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor{
    //         label: Some(&bgname),
    //         layout: &layout,
    //         entries: &bg_entries,
    //     }));

    //     self.entity_bindgroup = Some(bg);
    // }

    pub fn transform_mut(&mut self) -> &mut glam::Mat4 {
        self.model_matrix.as_mut()
    }

    pub fn transform_ref(&self) -> &glam::Mat4 {
        self.model_matrix.as_ref()
    }

    pub(crate) fn update_matrix(&self, rm: &crate::RenderManager) {
        self.model_matrix.update(&rm.gpu_state);
    }
}