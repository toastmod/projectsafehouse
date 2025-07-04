use std::{f32::consts::PI, rc::Rc};

use crate::{ball::Ball, pong::{BackForwVecs, PongPhysics, SCREEN_HEIGHT, SCREEN_WIDTH}, render};
use render::{BINDGROUP_GLOBAL, BINDGROUP_SCENEOBJECT, entity::Entity, glam, gpu::{self, buffer::UniformPtr, program}, model::ModelData, scene::SceneObjectHandle, vertex_type::{AdvVertex, ColorVertex}, RenderManager};
use gpu::{vertex::Vertex, wgpu,shaderprogram::Program};
use safehouse_render::{entity::EntityPipeline, gpu::{binding::Binder, buffer::Buffer}, named_entity};

pub const PADDLE_LENGTH: f32 = 0.15;
pub const PADDLE_THICK: f32 = 0.01;

#[derive(Debug)]
pub struct Paddle {
    scene_handle: SceneObjectHandle, 
    color: UniformPtr<[f32; 3]>,
}

impl Entity for Paddle {

    const ENTITY_TYPE_NAME: &'static str = "Paddle";

    fn on_instantiate(rm: &mut RenderManager, handle: SceneObjectHandle) -> Self {
        let color = UniformPtr::new(&rm.gpu_state, [0.0, 1.0, 0.0]);

        Self {
            scene_handle: handle,
            color
        }
    }

    fn load_model(state: &gpu::State) -> ModelData {
        let model_bytes = include_bytes!("model/paddle.dat");
        ModelData::new::<Self,()> (
            state,
            gpu::buffer::VertexBuffer::new_from_raw::<ColorVertex>(&state, model_bytes),
            vec![
                0..(model_bytes.len()/std::mem::size_of::<ColorVertex>()) as u32
            ],
            None
        )
    }

    fn load_pipeline(rm: &RenderManager) -> Option<EntityPipeline> {

        Some(EntityPipeline{
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
        })

    }
    
    fn load_bindings<'a>() -> Vec<Binder<Self>> where Self: Sized {
        vec![
            Binder::new(0, wgpu::ShaderStages::all(), &|x|{&x.color})
        ]
    }
    
    fn load_shader(rm: &safehouse_render::RenderManager, group_model: u32, group_entity: u32) -> Option<gpu::shaderprogram::Program> {
        Some(program!(
            &rm.gpu_state,
            source: format!("

            @group({BINDGROUP_GLOBAL}) @binding(0)
            var<uniform> time: f32;

            @group({BINDGROUP_SCENEOBJECT}) @binding(0)
            var<uniform> model_mat: mat4x4<f32>;

            @group({group_entity}) @binding(0)
            var<uniform> paddle_color: vec3<f32>;

            struct VIn {{
                @location(0)
                pos: vec4<f32>,
                @location(1)
                col: vec4<f32>
            }}
            struct VOut {{
                @builtin(position)
                pos: vec4<f32>,
                @location(0)
                col: vec4<f32>
            }}

            @vertex
            fn vs_main(i: VIn) -> VOut {{
                var o: VOut;
                o.pos = model_mat * i.pos;
                o.col = vec4<f32>(paddle_color, i.col.w);
                return o;
            }}

            @fragment
            fn fs_main(i: VOut) -> @location(0) vec4<f32> {{
                return i.col;
            }}
            ")
        ))
    }
    
}

named_entity!(Paddle);

impl Paddle {
    /// In this specific implementation, we are assuming a static screen size with no resizing.
    /// Therefore our game logic will use window coordinates, which we will convert to screen coords.
    pub fn move_to(&mut self, rm: &mut RenderManager, x: f32, y: f32) {
        let transform = rm.mut_scene_object(self.scene_handle).unwrap().transform_mut();
        transform.w_axis.x = x; 
        transform.w_axis.y = -y; 
    }

    pub fn get_pos(&self, rm: &RenderManager) -> (f32,f32) {
        let scene_obj = rm.get_scene_object(self.scene_handle).unwrap(); 
        let transform = scene_obj.transform_ref();
        (transform.w_axis.x, -transform.w_axis.y)
    }

    /// Set the color for the paddle.
    pub fn set_color(&mut self, rm: &RenderManager, rgb: [f32; 3]) {
        *self.color.as_mut() = rgb;
        self.color.update(&rm.gpu_state);
    }

    /// Detect collision if Ball hits the Paddle
    pub fn collision(&self, rm: &RenderManager, ball: &Ball) -> Option<PongCollision> {
        let (b1, b1y) = ball.get_pos(rm);
        let (b2, b2y) = (ball.px, ball.py);
        let (x, y) = self.get_pos(rm);

        let xbfv = BackForwVecs::new(b1, b2, x);

        if xbfv.is_collision() {
            let slope = (b2y-b1y)/(b2-b1);
            let yint = SCREEN_HEIGHT + ((slope*x)+y); 
            let bounce_angle = f32::abs(yint+(y+(PADDLE_LENGTH/2.0)))/PADDLE_LENGTH;

            if bounce_angle >= 0.0 && bounce_angle <= 1.0 {
                Some(PongCollision { 
                    yint, 
                    xint: x, 
                    bounce_angle 
                })
            }else {
                None
            }

        }else{
            None
        }

    } 
}

pub struct PongCollision {
    pub yint: f32,
    pub xint: f32,
    pub bounce_angle: f32,
}