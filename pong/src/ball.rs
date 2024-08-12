use std::{f32::consts::PI, rc::Rc, time::Duration};
use crate::{map, pong::{PongPhysics, PongState}, render};
use render::{entity::Entity, gpu::{self, wgpu}, model::ModelData, scene::SceneObjectHandle, vertex_type::ColorVertex, RenderManager};
use safehouse_render::entity::EntityPipeline;

#[derive(Debug,Default)]
pub struct Ball {
    pub scene_handle: SceneObjectHandle, 
    pub speed: f32,
    pub vx: f32,
    pub vy: f32,
    pub px: f32,
    pub py: f32,
}

impl Entity for Ball {

    const ENTITY_TYPE_NAME: &'static str = "ball";

    fn on_instantiate<'w>(rm: &mut RenderManager<'w>, handle: SceneObjectHandle) -> Self {
        let mut b = Self::default();
        b.speed = 1.4; // 0.5 units/second
        // Initial distribution of speed stat
        b.vx = -0.9*b.speed;
        b.vy = -0.1*b.speed;
        let (px, py) = b.get_pos(rm);
        b.px = px;
        b.py = py;
        b.scene_handle = handle;
        b
    }

    fn model_name() -> &'static str {
        "ball_model"
    }
    fn load_model(state: &gpu::State) -> ModelData {
        
        ModelData {
            vertex_buffer: gpu::buffer::VertexBuffer::new(&state, &[
                ColorVertex { pos: [-0.01,0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [0.01,0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [0.01,-0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},

                ColorVertex { pos: [0.01,-0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [-0.01,-0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
                ColorVertex { pos: [-0.01,0.01,0.0,1.0], color: [1.0,1.0,1.0,1.0]},
            ]),
            textures: None,
            model_bindgroup: None,
            groups: Box::new([0..6])
        }
        
    }

    fn pipeline_name() -> &'static str {
        "default"
    }
    fn load_pipeline(_: &RenderManager) -> Option<EntityPipeline> {
        None
    }
    
    fn bindings_name() -> &'static str {
        "ball_bindings"
    }
    fn load_bindings<'a>() -> Vec<gpu::binding::Binder<Self>> where Self: Sized {
        vec![]
    }
    
    fn shader_name() -> &'static str {
        "default" 
    }
    
    fn load_shader(rm: &safehouse_render::RenderManager, group_model: u32, group_entity: u32) -> Option<gpu::shaderprogram::Program> {
        None
    }
    
    
}

impl Ball {

    pub fn move_next(&mut self, rm: &mut RenderManager, delta_time: Duration) {
        let scene_obj = rm.get_scene_object(self.scene_handle).unwrap(); 
        let transform = scene_obj.transform_ref();
        let (x, y) = self.get_next_pos(rm, delta_time);
        self.move_to(rm, x, y)
    }

    pub fn get_next_pos(&self, rm: &RenderManager, delta_time: Duration) -> (f32, f32) {
        let (x, y) = self.get_pos(rm);
        let scene_obj = rm.get_scene_object(self.scene_handle).unwrap(); 
        let transform = scene_obj.transform_ref();
        (transform.w_axis.x + self.vx * delta_time.as_secs_f32(), (-transform.w_axis.y) + self.vy * delta_time.as_secs_f32())
    }

    pub fn get_pos(&self, rm: &RenderManager) -> (f32,f32) {
        let scene_obj = rm.get_scene_object(self.scene_handle).unwrap(); 
        let transform = scene_obj.transform_ref();
        (transform.w_axis.x, -transform.w_axis.y)
    }

    pub fn move_to(&mut self, rm: &mut RenderManager, x: f32, y: f32) {
        let transform = rm.mut_scene_object(self.scene_handle).unwrap().transform_mut();
        transform.w_axis.x = x; 
        transform.w_axis.y = -y; 
    }

    pub fn bounce(&mut self, bounce_angle: f32) {
        let bounce_angle = map(bounce_angle, 0.0..1.0, 0.2..0.8);
        self.vx = -self.vx.signum() * f32::cos(bounce_angle * (PI/2.0)) * self.speed;
        self.vy = self.vy.signum() * f32::sin(bounce_angle * (PI/2.0)) * self.speed;

    } 

    pub fn bounce_x(&mut self) {
        self.vx *= -1.0;
    } 

    pub fn bounce_y(&mut self) {
        self.vy *= -1.0;
    } 
}