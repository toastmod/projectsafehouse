use crate::render::{entity::Entity, model::ModelData, vertex_type::ColorVertex, RenderManager};
use crate::{ball::Ball, paddle::Paddle};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 800.0;

#[derive(Debug)]
pub struct Pong {
    player: Paddle,
    player_score: u8,
    cpu: Paddle,
    cpu_score: u8,
    ball: Ball,
}

impl Pong {
    /// Load resources to start the game
    pub fn load_game(rm: &mut RenderManager) -> Self {

        // Procedurally loading the entity data
        rm.load_entity::<Paddle>();
        rm.load_entity::<Ball>();

        // Spawn the scene objects and serve.
        Self {
            player: rm.spawn_sceneobject_entity::<Paddle>("Player"),
            cpu: rm.spawn_sceneobject_entity::<Paddle>("CPU"),
            ball: rm.spawn_sceneobject_entity::<Ball>("Ball"),
            player_score: 0,
            cpu_score: 0,
        }

    }

    /// On game start
    pub fn init(&mut self, rm: &mut RenderManager) {

        // Move player paddle to left center
        self.player.set_color(rm, [1.0,0.0,0.0]);
        self.player.move_to(rm, 0.0, SCREEN_HEIGHT/2.0);

        // Move CPU paddle to right center
        self.cpu.set_color(rm, [0.0,0.0,1.0]);
        self.cpu.move_to(rm, SCREEN_WIDTH, SCREEN_HEIGHT/2.0);

    }

    pub fn mouse_moved(&mut self, rm: &mut RenderManager, x: f32, y: f32) {
        self.player.move_to(rm, 0.0, y);
    }

    pub fn update(&mut self, rm: &mut RenderManager) {
        self.ball.update(rm);

    }
}