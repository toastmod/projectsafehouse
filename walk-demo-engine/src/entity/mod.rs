pub mod textpane;
pub mod bunny;
use safehouse_render::{entity::{Entity, EntityPipeline}, glam::Vec4Swizzles, gpu::{self, buffer::VertexBuffer, program}, model::ModelData, scene::SceneObjectHandle, vertex_type::TexVertex};

pub trait ActiveEntity {
    fn get_sceneobject_handle(&self) -> SceneObjectHandle;
    fn get_position(&self, engine: &super::Engine) -> (f32,f32,f32) where Self: Sized {
        let pos = &engine.get_scene_object(self).unwrap().model_matrix.as_ref().w_axis;
        pos.xyz().into()
    }
}