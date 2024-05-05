use std::{rc::Rc};
use crate::{engine, Pong};
use engine::entity::ActiveEntity;
use safehouse_engine::{entity::Entity, model::ModelData};

#[derive(Debug,Default)]
pub struct Ball {
    x: f32,
    vx: f32,
    y: f32,
    vy: f32,
}

impl Entity for Ball {

    fn load_model(state: &mut engine::gpu::State) -> Rc<safehouse_engine::model::ModelData> {
        Rc::new(ModelData {
            vertex_buffer: engine::gpu::buffer::VertexBuffer::new(&state, &[

            ]),
            textures: None,
            model_bindgroup: state.init_bindgroup_from_pipeline(None, engine::MODEL, &[]),
        })
    }

    fn load_pipeline(state: &mut safehouse_engine::gpu::State) -> Option<Rc<wgpu::RenderPipeline>> {
        None
    }
    
    fn instantiate<'w, 'c, Context>(engine: &mut engine::Engine<'w, 'c, Context>) -> Self {
        todo!()
    }
}

impl ActiveEntity for Ball {
    fn on_spawn(&mut self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn on_despawn(self) {
        todo!()
    }
}