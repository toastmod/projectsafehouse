use safehouse_render::entity::Entity;

use crate::entity::bunny::Bunny;

use super::Scene;

pub struct WalkingScene {

}

impl Scene for WalkingScene {
    fn init(&mut self, engine: &mut crate::Engine) {
        engine.rm.load_entity::<Bunny>();
        engine.rm.spawn_sceneobject_entity::<Bunny>("test bunny");
    }

    fn update(&mut self, engine: &mut crate::Engine) {
        // engine.camera.upd8(force_upd8_matrix, delta_time)
    }
}