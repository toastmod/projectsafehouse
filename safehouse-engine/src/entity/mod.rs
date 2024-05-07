use std::rc::Rc;
use crate::{context::Context, gpu, model::ModelData, scene::{SceneObject, SceneObjectHandle}};
use gpu::wgpu;

pub trait Entity {
    fn on_instantiate(engine: &mut crate::Engine<'_>, handle: SceneObjectHandle) -> Self;
    fn load_model(state: &mut gpu::State) -> ModelData;
    fn model_name() -> &'static str;
    fn load_pipeline(state: &mut gpu::State) -> Option<Rc<wgpu::RenderPipeline>>;
    fn pipeline_name() -> &'static str;
}

// pub trait ActiveEntity<C: Context> {
//     fn on_spawn(&mut self, engine: &mut crate::Engine<'_,'_,C>);
//     fn update(&mut self, engine: &mut crate::Engine<'_,'_,C>);
//     fn on_despawn(self, engine: &mut crate::Engine<'_,'_,C>);
// }
