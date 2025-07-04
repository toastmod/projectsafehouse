use safehouse_render::{camera::{subject_zoom_pos, Camera}, entity::Entity, gpu::winit};

use crate::entity::{bunny::Bunny, ActiveEntity};

use super::{Scene, SceneEvent, SceneInit};

pub struct WalkingScene {
    bunny: Bunny
}

impl SceneInit for WalkingScene {
    fn init(engine: &mut crate::Engine) -> Self {
        engine.rm.load_entity::<Bunny>();

        let mut bunny = engine.rm.spawn_sceneobject_entity::<Bunny>("test bunny");
        let sub_zoom_pos = subject_zoom_pos(engine.camera.position, bunny.get_position(engine), f32::sin(engine.get_delta_time().as_secs_f32()));
        engine.camera.set_pos(sub_zoom_pos);
        
        Self {
            bunny
        }
    }

}

impl Scene for WalkingScene {

    fn update(&mut self, engine: &mut crate::Engine) -> SceneEvent {
        // engine.camera.upd8(true, engine.get_delta_time().as_nanos());
        engine.camera.update_vals(1.0*engine.get_delta_time().as_secs_f32(), &engine.controller);

        SceneEvent::Continue
    }
}