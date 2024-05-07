use std::{rc::Rc};
use crate::{engine, Pong};
use engine::{context::Context, entity::Entity, gpu::wgpu, model::ModelData, scene::SceneObjectHandle, vertex_type::ColorVertex};

#[derive(Debug,Default)]
pub struct Ball {
    scene_handle: SceneObjectHandle, 
    vx: f32,
    vy: f32,
}

impl Entity for Ball {

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
    
    fn on_instantiate<'w>(engine: &mut engine::Engine<'w>, handle: SceneObjectHandle) -> Self {
        let mut b = Self::default();
        b.scene_handle = handle;
        b
    }
    
    fn model_name() -> &'static str {
        "ball"
    }
    
    fn pipeline_name() -> &'static str {
        "default"
    }
    
}

impl Ball {
    pub fn update(&mut self, engine: &mut engine::Engine, ) {
        
    }
}