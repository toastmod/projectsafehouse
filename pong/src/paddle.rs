use std::rc::Rc;

use engine::{entity::Entity, gpu::wgpu, model::ModelData, scene::SceneObjectHandle, vertex_type::ColorVertex, Engine};

use crate::engine;
use engine::glam;

#[derive(Debug,Default)]
pub struct Paddle {
    scene_handle: SceneObjectHandle, 
    xwidth: f32, 
}

impl Entity for Paddle {

    fn load_model(state: &mut engine::gpu::State) -> ModelData {
        
        ModelData {
            vertex_buffer: engine::gpu::buffer::VertexBuffer::new(&state, &[
                ColorVertex { pos: [0.0,0.5,0.0], color: [1.0,0.0,0.0]},
                ColorVertex { pos: [0.5,-0.5,0.0], color: [0.0,1.0,0.0]},
                ColorVertex { pos: [-0.5,-0.5,0.0], color: [0.0,0.0,1.0]},
            ]),
            textures: None,
            model_bindgroup: None,
            groups: Box::new([0..3])
        }
        
    }

    fn load_pipeline(state: &mut safehouse_engine::gpu::State) -> Option<Rc<wgpu::RenderPipeline>> {
        None
    }
    
    fn instantiate<'w>(engine: &mut Engine<'w>, handle: SceneObjectHandle) -> Self {
        Self {
            scene_handle: handle,
            ..Default::default()
        }
    }
    
    fn model_name() -> &'static str {
        "paddle"
    }
    
    fn pipeline_name() -> &'static str {
        "default"
    }
    
}

impl Paddle {
    pub fn move_to(&mut self, engine: &mut Engine, x: f32, y: f32) {
        let screen_width = engine.gpu_state.config.width;
        let screen_height= engine.gpu_state.config.height;
        let mut transform = engine.mut_scene_object(self.scene_handle).unwrap().transform_mut();
        *transform = glam::Mat4::from_translation(glam::vec3(x/screen_width as f32, y/screen_height as f32, 0.0));
    }
}