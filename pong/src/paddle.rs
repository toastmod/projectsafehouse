use std::rc::Rc;

use crate::render;
use render::{BINDGROUP_GLOBAL, BINDGROUP_SCENEOBJECT, entity::Entity, glam, gpu::{self, buffer::UniformPtr, program}, model::ModelData, scene::SceneObjectHandle, vertex_type::{AdvVertex, ColorVertex}, RenderManager};
use gpu::{vertex::Vertex, wgpu,shaderprogram::Program};


#[derive(Debug)]
pub struct Paddle {
    scene_handle: SceneObjectHandle, 
    color: UniformPtr<[f32; 3]>,
}

impl Entity for Paddle {

    fn on_instantiate<'w>(rm: &mut RenderManager<'w>, handle: SceneObjectHandle) -> Self {
        Self {
            scene_handle: handle,
            color: UniformPtr::new(&rm.gpu_state, [0.0, 1.0, 0.0])
        }
    }

    fn load_model(state: &mut gpu::State) -> ModelData {
        let model_bytes = include_bytes!("model/paddle.dat");
        ModelData {
            vertex_buffer: gpu::buffer::VertexBuffer::new_from_raw::<ColorVertex>(&state, model_bytes),
            textures: None,
            model_bindgroup: None,
            groups: Box::new([
                0..(model_bytes.len()/std::mem::size_of::<ColorVertex>()) as u32
            ])
        }
    }

    fn load_pipeline(state: &mut gpu::State) -> Option<Rc<wgpu::RenderPipeline>> {

        let shader = &state.get_shader("default").module;

        let s = state.add_shader("shader_paddle", program!(
            state,
            source: format!("

            @binding({BINDGROUP_GLOBAL}) @group(0)
            var<uniform> time: f32;

            @binding({BINDGROUP_SCENEOBJECT}) @group(0)
            var<uniform> model_mat: mat4x4<f32>;

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
                o.pos = i.pos;
                o.col = i.col;
                return o;
            }}

            @fragment
            fn fs_main(i: VOut) -> @location(0) vec4<f32> {{
                return i.col;
            }}
            ")
        ));

        Some(state.add_render_pipeline("paddle", &wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: "vs_main",
                buffers: &[ColorVertex::desc().clone()]
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState{
                module: shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState{
                    format: state.config.format.clone(),
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None
        }))
    }
    
    fn model_name() -> &'static str {
        "model_paddle"
    }
    
    fn pipeline_name() -> &'static str {
        "default"
    }
    
}

impl Paddle {
    /// In this specific implementation, we are assuming a static screen size with no resizing.
    /// Therefore our game logic will use window coordinates, which we will convert to screen coords.
    pub fn move_to(&mut self, rm: &mut RenderManager, x: f32, y: f32) {
        let (x, y) = rm.window_to_world_coord(x, y);
        let transform = rm.mut_scene_object(self.scene_handle).unwrap().transform_mut();
        transform.w_axis.x = x; 
        transform.w_axis.y = -y; 
    }

    /// Set the color for the paddle.
    pub fn set_color(&mut self, rm: &mut RenderManager, rgb: [f32; 3]) {
        *self.color.as_mut() = rgb;
        self.color.update(&rm.gpu_state);
    }
}