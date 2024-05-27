use std::rc::Rc;

use crate::render;
use render::{BINDGROUP_GLOBAL, BINDGROUP_SCENEOBJECT, BINDGROUP_ENTITY, entity::Entity, glam, gpu::{self, buffer::UniformPtr, program}, model::ModelData, scene::SceneObjectHandle, vertex_type::{AdvVertex, ColorVertex}, RenderManager};
use gpu::{vertex::Vertex, wgpu,shaderprogram::Program};
use safehouse_render::{entity::{entity_layout_entry, EntityLayoutEntry, EntityPipeline}, gpu::buffer::{Bindable, Buffer}};


#[derive(Debug)]
pub struct Paddle {
    pub scene_handle: SceneObjectHandle, 
    pub color: UniformPtr<[f32; 3]>,
}

impl Entity for Paddle {

    fn on_instantiate<'w>(rm: &mut RenderManager<'w>, handle: SceneObjectHandle) -> Self {

        let color = UniformPtr::new(&rm.gpu_state, [0.0, 1.0, 0.0]);

        Self {
            scene_handle: handle,
            color,
        }
    }

    fn load_model(rm: &mut RenderManager) -> Option<Rc<ModelData>> {
        let model_bytes = include_bytes!("model/paddle.dat");
        Some(rm.add_model(
            &Self::model_name(),
            gpu::buffer::VertexBuffer::new_from_raw::<ColorVertex>(&rm.gpu_state, model_bytes),
            None,
            Box::new([
                0..(model_bytes.len()/std::mem::size_of::<ColorVertex>()) as u32
            ])
        ))
    }

    fn load_pipeline<'a>(rm: &'a RenderManager, module: &'a wgpu::ShaderModule) -> EntityPipeline<'a> {

        EntityPipeline::Custom {
            vertex: wgpu::VertexState {
                module,
                entry_point: "vs_main",
                buffers: &[ColorVertex::desc().clone()]
            },
            fragment: Some(wgpu::FragmentState{
                module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState{
                    format: rm.gpu_state.config.format.clone(),
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            depth_stencil: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
        }

    }
    
    fn entity_type_name() -> &'static str {
        "paddle"
    }
    
    fn load_entity_bindings<'inactive, 'active>() -> Vec<EntityLayoutEntry<'inactive, 'active, Self>> {
        vec![
            entity_layout_entry::<Paddle, UniformPtr<[f32; 3]>>(0, wgpu::ShaderStages::all(), &|x: &Paddle| x.color.get_binding_entry(0))
        ]
    }
    
    fn load_shader<'a>(rm: &'a safehouse_render::RenderManager) -> Option<Program> {
        Some(program!(
            &rm.gpu_state,
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
                let t = time;
                o.pos = model_mat * i.pos;
                o.col = i.col;
                return o;
            }}

            @fragment
            fn fs_main(i: VOut) -> @location(0) vec4<f32> {{
                return vec4<f32>(paddle_color, i.col.w);
            }}
            ")
        ))
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