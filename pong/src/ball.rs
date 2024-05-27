use std::rc::Rc;
use crate::render;
use render::{entity::Entity, gpu::{self, wgpu}, model::ModelData, scene::SceneObjectHandle, vertex_type::ColorVertex, RenderManager};

#[derive(Debug,Default)]
pub struct Ball {
    scene_handle: SceneObjectHandle, 
    vx: f32,
    vy: f32,
}

impl Entity for Ball {

    fn load_model(rm: &mut RenderManager) -> Option<Rc<ModelData>> {
        Some(rm.add_model(
            &Self::model_name(), 
            gpu::buffer::VertexBuffer::new(&rm.gpu_state, &[
                ColorVertex { pos: [-0.01,0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [0.01,0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [0.01,-0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},

                ColorVertex { pos: [0.01,-0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [-0.01,-0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [-0.01,0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
            ]),
            None, 
            Box::new([0..6])
        ))
    }

    fn load_pipeline<'a>(rm: &'a RenderManager, shader_module: &'a wgpu::ShaderModule) -> render::entity::EntityPipeline<'a> {
        render::entity::EntityPipeline::Default 
    }

    fn on_instantiate<'w>(_: &mut RenderManager<'w>, handle: SceneObjectHandle) -> Self {
        let mut b = Self::default();
        b.vx = 0.0;
        b.vy = 0.0;
        b.scene_handle = handle;
        b
    }
    
    fn entity_type_name() -> &'static str {
        "ball"
    }
    
    fn load_entity_bindings<'inactive, 'active>() -> Vec<safehouse_render::entity::EntityLayoutEntry<'inactive, 'active, Self>> {
        vec![]
    }
    
    fn load_shader<'a>(rm: &'a safehouse_render::RenderManager) -> Option<gpu::shaderprogram::Program> {
        None
    }
    
    
}

impl Ball {
    pub fn update(&mut self, rm: &mut RenderManager) {
        let scene_obj = rm.mut_scene_object(self.scene_handle).unwrap(); 
        let transform = scene_obj.transform_mut();
        transform.w_axis.x += self.vx;
        transform.w_axis.y += self.vy;
    }

    pub fn get_pos(&self, rm: &RenderManager) -> (f32,f32) {
        let scene_obj = rm.get_scene_object(self.scene_handle).unwrap(); 
        let transform = scene_obj.transform_ref();
        rm.world_to_window_coord(transform.w_axis.x, transform.w_axis.y)
    }

    pub fn move_to(&mut self, rm: &mut RenderManager, x: f32, y: f32) {
        let (x, y) = rm.window_to_world_coord(x, y);
        let transform = rm.mut_scene_object(self.scene_handle).unwrap().transform_mut();
        transform.w_axis.x = x; 
        transform.w_axis.y = -y; 
    }
}