use safehouse_engine::{entity::Entity, model::ModelData, vertex_type::ColorVertex, Engine};

use crate::{ball::Ball, paddle::Paddle};

#[derive(Debug)]
pub struct Pong {
    player: Paddle,
    cpu: Paddle,
    ball: Ball,
}

impl Pong {
    /// Load resources to start the game
    pub fn load_game(engine: &mut Engine) -> Self {
        let ball_model = Ball::load_model(&mut engine.gpu_state);

        engine.add_model(
            "ball",
            ball_model 
        );

        let paddle_model = ModelData::create_model::<ColorVertex>(
                &mut engine.gpu_state,
            None,
            &[
                ColorVertex { pos: [-1.0,1.0,0.0], color: [1.0,0.0,0.0] },
                ColorVertex { pos: [1.0,1.0,0.0], color: [0.0,1.0,0.0] },
                ColorVertex { pos: [1.0,-1.0,0.0], color: [0.0,0.0,1.0] },

                ColorVertex { pos: [1.0,-1.0,0.0], color: [0.0,0.0,1.0] },
                ColorVertex { pos: [-1.0,-1.0,0.0], color: [0.0,1.0,0.0] },
                ColorVertex { pos: [-1.0,1.0,0.0], color: [1.0,0.0,0.0] }
            ],
            None 
        );

        engine.add_model(
            "paddle",
            paddle_model
        );

        Self {
            player: engine.spawn_sceneobject_entity::<Paddle>("Player"),
            cpu: engine.spawn_sceneobject_entity::<Paddle>("CPU"),
            ball: Ball::default(),
        }

    }

    pub fn mouse_moved(&mut self, engine: &mut Engine, x: f32, y: f32) {
        self.player.move_to(engine, x, y)
    }
}