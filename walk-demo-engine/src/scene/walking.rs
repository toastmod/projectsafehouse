use safehouse_render::{camera::{subject_zoom_pos, Camera}, entity::Entity, gpu::winit};

use crate::entity::bunny::Bunny;

use super::{Scene, SceneEvent, SceneInit};

pub struct WalkingScene {
    bunny: Bunny
}

impl SceneInit for WalkingScene {
    fn init(engine: &mut crate::Engine) -> Self {
        engine.rm.load_entity::<Bunny>();
        
        Self {
            bunny: engine.rm.spawn_sceneobject_entity::<Bunny>("test bunny")
        }
    }

}

impl Scene for WalkingScene {

    fn update(&mut self, engine: &mut crate::Engine) -> SceneEvent {
        engine.camera.upd8(false, engine.get_delta_time());

        // let sub_zoom_pos = subject_zoom_pos(engine.camera.position, engine.rm.get_scene_object(self.bunny.handle).unwrap.transform_ref(), 0.5);

        // engine.camera.set_dir(dir);
        SceneEvent::Continue
    }
}