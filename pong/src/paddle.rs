use std::rc::Rc;

use crate::render;
use render::{BINDGROUP_GLOBAL, BINDGROUP_SCENEOBJECT, BINDGROUP_ENTITY, entity::Entity, glam, gpu::{self, buffer::UniformPtr, program}, model::ModelData, scene::SceneObjectHandle, vertex_type::{AdvVertex, ColorVertex}, RenderManager};
use gpu::{vertex::Vertex, wgpu,shaderprogram::Program};
use safehouse_render::gpu::buffer::Buffer;


#[derive(Debug)]
pub struct Paddle {
    scene_handle: SceneObjectHandle, 
    color: UniformPtr<[f32; 3]>,
}

impl Entity for Paddle {

    fn on_instantiate<'w>(rm: &mut RenderManager<'w>, handle: SceneObjectHandle) -> Self {
        let color = UniformPtr::new(&rm.gpu_state, [0.0, 1.0, 0.0]);

        // create bindgroup
        let bindgroup = rm.gpu_state.init_bindgroup_from_pipeline(Self::pipeline_name(), BINDGROUP_ENTITY, &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: color.get_buffer().as_entire_binding()
            }
        ]).expect("Could not create bindgroup!");
        rm.mut_scene_object(handle).unwrap().attach_entity_bindgroup(bindgroup); 

        Self {
            scene_handle: handle,
            color
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

        let shader = state.add_shader("paddle_shader", program!(
            state,
            source: format!("

            @group({BINDGROUP_GLOBAL}) @binding(0)
            var<uniform> time: f32;

            @group({BINDGROUP_SCENEOBJECT}) @binding(0)
            var<uniform> model_mat: mat4x4<f32>;

            @group({BINDGROUP_ENTITY}) @binding(0)
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
        ));

        Some(state.add_render_pipeline(Self::pipeline_name(), &wgpu::RenderPipelineDescriptor {
            label: Some(Self::pipeline_name()),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader.module,
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
                module: &shader.module,
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
        "paddle_model"
    }
    
    fn pipeline_name() -> &'static str {
        "paddle_pipe"
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