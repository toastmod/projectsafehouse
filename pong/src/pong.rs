use std::time::Duration;

use safehouse_render::entity::NamedEntity;
use safehouse_render::scene::{SceneObject, SceneObjectHandle};
use tagmap::TagMap;
use winit_app_handler::WinitApp;

use crate::paddle::{self, PongCollision, PADDLE_LENGTH, PADDLE_THICK};
use crate::render::{entity::Entity, model::ModelData, vertex_type::ColorVertex, RenderManager};
use crate::{ball::Ball, paddle::Paddle};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 800.0;
pub const PLAYER_X: f32 = (-1.0) + (PADDLE_THICK*3.0);
pub const CPU_X: f32 = 1.0 - (PADDLE_THICK*3.0);

pub struct Pong {
   pub state: PongState,
   pub cpu_speed: f32,
} 

impl Pong {
    pub fn start(rm: &mut RenderManager) -> Self {
        let mut state = PongState::load_game(rm);
        state.init(rm);

        Self {
            state,
            cpu_speed: 1.3,
        }
    }

    pub fn update(&mut self, rm: &mut RenderManager, delta_time: Duration) {
        let ballpos = self.state.ball.get_pos(rm);
        let playerpos = self.state.player.get_pos(rm); 
        let cpupos = self.state.cpu.get_pos(rm); 
        let ballnextpos = self.state.ball.get_next_pos(rm, delta_time);

        let slope = (ballnextpos.1 - ballpos.1)/(ballnextpos.0 - ballpos.0);

        if ballnextpos.0 <= PLAYER_X {
            let player_collision_y = (slope * (ballnextpos.0 - PLAYER_X)) + ballpos.1;

            let paddle_upper = playerpos.1 + (PADDLE_LENGTH/2.0);
            let paddle_lower = playerpos.1 - (PADDLE_LENGTH/2.0);

            let grad = (player_collision_y - paddle_lower) / (PADDLE_LENGTH);
            
            if grad >= 0.0 && grad <= 1.0 {                
                self.state.ball.bounce(grad);
            } else {
                self.state.reset(rm);
            }

        } else if ballnextpos.0 >= CPU_X {
            let cpu_collision_y = (slope * (ballnextpos.0 - CPU_X)) + ballpos.1;

            let paddle_upper = cpupos.1 + (PADDLE_LENGTH/2.0);
            let paddle_lower = cpupos.1 - (PADDLE_LENGTH/2.0);

            let grad = (cpu_collision_y - paddle_lower) / (PADDLE_LENGTH);
            
            if grad >= 0.0 && grad <= 1.0 {                
                self.state.ball.bounce(grad);
            } else {
                self.state.reset(rm);
            }

        } 
        
        // Court floor and ceiling
        if ballnextpos.1 >= 1.0 || ballnextpos.1 <= -1.0 {
            self.state.ball.bounce_y();
        }

        // CPU Logic
        if (cpupos.1 - ballpos.1).is_sign_negative() {
            self.state.cpu.move_to(rm, cpupos.0, cpupos.1 + (self.cpu_speed * delta_time.as_secs_f32()));
        } else {
            self.state.cpu.move_to(rm, cpupos.0, cpupos.1 - (self.cpu_speed * delta_time.as_secs_f32()));
        }

        self.state.ball.move_next(rm, delta_time);


    } 

    pub fn stop(&self) {
        if self.state.player_score > self.state.cpu_score {
            println!("Player wins with {} points!", self.state.player_score);
        } else if self.state.player_score > self.state.cpu_score {
            println!("CPU wins with {} points!", self.state.cpu_score);
        } else{
            println!("It was a tie at {} - {} points!", self.state.player_score, self.state.cpu_score);
        }
    }

    pub fn mouse_moved(&mut self, rm: &mut RenderManager, x: f32, y: f32) {
        self.state.player.move_to(rm, PLAYER_X, y);
    }
}


#[derive(Debug)]
pub struct PongState {
    pub player: Paddle,
    pub player_score: u8,
    pub cpu: Paddle,
    pub cpu_score: u8,
    pub ball: Ball,
}

impl PongState {
    /// Load resources to start the game
    pub fn load_game(rm: &mut RenderManager) -> Self {

        // Loading entity data
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

        self.cpu.move_to(rm, CPU_X, 0.0);
        self.player.move_to(rm, PLAYER_X, 0.0);

        // Set player and CPU colors
        self.player.set_color(rm, [1.0,0.0,0.0]);
        self.cpu.set_color(rm, [0.0,0.0,1.0]);

        self.reset(rm);

    }

    /// Called everytime a goal is scored
    pub fn reset(&mut self, rm: &mut RenderManager) {
        println!("Game reset!");

        // Move ball to center
        self.ball.move_to(rm, 0.0, 0.0);

    }

}

pub struct BackForwVecs {
    back: f32,
    forw: f32
}

impl BackForwVecs {
    pub fn new(b1: f32, b2: f32, x: f32) -> BackForwVecs {
        let back = b1-x;
        let forw = b2-x;
        BackForwVecs {
            back,
            forw
        }
    }

    /// Refract the Forward vector by an amount
    pub fn refract(&mut self, r_angle: f32) {
        self.forw *= r_angle;
    }

    pub fn is_collision(&self) -> bool {
        // For a valid collision, the two vectors should be opposite directions.
        // A valid collision is also when the forward vector is exactly zero.
        (self.back.is_sign_negative() == self.forw.is_sign_positive()) || self.forw == 0.0
    }

    pub fn is_zero(&self) -> bool {
        self.back == 0.0 && self.forw == 0.0
    }

    pub fn b_direction(&self) -> f32 {
        self.forw.signum()
    }

    pub fn get_normal(&self) -> f32 {
        self.forw + self.back
    }
}

pub struct PongPhysics;

impl PongPhysics {

    pub fn update(rm: &mut RenderManager, game: &mut PongState, delta_time: Duration) {

        let (bx2, by2) = game.ball.get_pos(rm);
        let (bx1, by1) = (game.ball.px, game.ball.py);

        if let Some(collision) = game.player.collision(rm, &game.ball) {

            println!("Collision!");
            game.ball.bounce(collision.bounce_angle);
        }

        let (ball_x, ball_y) = game.ball.get_pos(rm);

        // CPU Scored!
        if ball_x < 0.0 {
            game.cpu_score += 1;
            game.reset(rm);
            println!("Score!");
        }

        // Player Scored!
        if ball_x > SCREEN_WIDTH {
            game.player_score += 1;
            game.reset(rm);
            println!("Score!");
        } 
    }



    /// Detects a collision between a time delta line `b` (as in ball) and two boundaraies of a box `x1` and `x2`.\
    /// Returns the amount to move back or fourth long that line to correct `b`.
    pub fn linebox_collision(b1: f32, b2: f32, x1: f32, x2: f32) -> Option<f32> {
        let x1bf = BackForwVecs::new(b1, b2, x1);
        let x2bf = BackForwVecs::new(b1, b2, x2);
        let x1norm = x1bf.get_normal();
        let x2norm = x2bf.get_normal();

        // If one or both are a collision
        if x1bf.is_collision() || x2bf.is_collision() {
            let b_delta = b2-b1; // <- signum of this = direction of ball
            // Whichever vector matches the direction of the ball, set the ball to that boundary
            if x1bf.back.signum() == b_delta.signum() {
                return Some(b2 - x1);
            } else if x2bf.back.signum() == b_delta.signum() {
                return Some(b2 - x2);
            } else {
                // Both vector pairs must be dead center, this should never happen.
                return None;
            }
        }

        // If both are a collision (frame overshot, ball delta goees over each end of the box)
        // if x1bf.is_collision() && x2bf.is_collision() {
        //     // Set ball to opposite side to it's direction
        //     // Whichever vector matches the direction of the ball, set the ball to that boundary
        //     let b_delta = b2-b1; // <- signum of this = direction of ball
        //     if x1norm.signum() == b_delta.signum() {
        //         return Some(x1 - b2);
        //     } else if x2norm.signum() == b_delta.signum() {
        //         return Some(x2 - b2);
        //     } else {
        //         // Both vector pairs must be dead center, this should never happen.
        //         return None;
        //     }

        // }

        // If neither are a collision (ball delta is either inside or outside the box)
        if !x1bf.is_collision() && !x2bf.is_collision() {

            // If either are inside, the back and forward deltas (combined as normals) will face toward from eachother
            if x1norm.is_sign_positive() == x2norm.is_sign_negative() {
                let b_delta = b2-b1; // <- signum of this = direction of ball
                // Whichever vector matches the direction of the ball, set the ball to that boundary
                if x1norm.signum() == b_delta.signum() {
                    return Some(b2 - x1);
                } else if x2norm.signum() == b_delta.signum() {
                    return Some(b2 - x2);
                } else {
                    // Both vector pairs must be dead center, this should never happen.
                    return None;
                }
            } 
            // If either are outside, the back and forward deltas (combined as normals) will face the same direction 
            else {
                // At this point, there is definetly no collision. Do nothing.
                return None;
            } 
        } else {
            return None;
        }

    }

}