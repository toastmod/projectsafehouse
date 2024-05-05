use std::rc::Rc;
use crate::{gpu, model::ModelData};
use gpu::wgpu;

pub trait Entity {
    fn instantiate<'w, 'c, Context>(engine: &mut crate::Engine<'w, 'c, Context>) -> Self;
    fn load_model(state: &mut gpu::State) -> Rc<ModelData>;
    fn load_pipeline(state: &mut gpu::State) -> Option<Rc<wgpu::RenderPipeline>>;
}

pub trait ActiveEntity {
    fn on_spawn(&mut self);
    fn update(&mut self);
    fn on_despawn(self);
}
