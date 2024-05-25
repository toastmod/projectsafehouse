use std::rc::Rc;
use crate::{gpu, model::ModelData, scene::{SceneObject, SceneObjectHandle}};
use gpu::wgpu;

pub trait Entity {

    // Entity instance functions
    fn on_instantiate(rm: &mut crate::RenderManager<'_>, handle: SceneObjectHandle) -> Self;

    // Data Loading
    fn model_name() -> &'static str;
    fn load_model(state: &mut gpu::State) -> ModelData;
    fn pipeline_name() -> &'static str;
    fn load_pipeline(state: &mut gpu::State) -> Option<Rc<wgpu::RenderPipeline>>;
}