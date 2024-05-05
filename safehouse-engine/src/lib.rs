use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use entity::{ActiveEntity, Entity};
use gpu::{buffer::{Buffer, UniformPtr}, program, vertex::Vertex};
use model::{obj, ModelData};

pub use safehouse_gpu as gpu;
use scene::SceneObject;
use shader::Shader;

use gpu::wgpu;

pub mod entity;
pub mod model;
pub mod scene;
pub mod vertex_type;
pub mod shader;

/// Bindings used by all shaders, governed by the engine.\
/// E.g: time, camera, debug flags, etc.
pub const GLOBAL: u32 = 0;

/// Bindings used by all shaders, governed by the engine.\
/// E.g: time, camera, debug flags, etc.
pub const CONTEXT: u32 = 1;

/// Bindings to model data.\
/// E.g: textures, skeleton animation variables, etc.
pub const MODEL: u32 = 2;

/// Bindings for shader specific options.
pub const SHADER: u32 = 3;

/// Bindings for GPU-side updates of an entity.
pub const ENTITY: u32 = 4;

pub struct Engine<'window, 'context, Context> {
    
    /// The GPU backend state.
    pub gpu_state: gpu::State<'window>,

    /// The global bindgroup to be used in all shaders.
    global_bindgroup: Rc<wgpu::BindGroup>,

    /// Cache for currently loaded model data.
    model_data_cache: HashMap<String, Rc<ModelData>>, 

    /// All active entities to be updated by the CPU.
    active_entities: Vec<Box<dyn ActiveEntity + 'context>>,

    /// All objects to render.
    scene_objects: Vec<SceneObject>,

    /// Cache for currently loaded shaders.
    shader_cache: HashMap<String, Rc<Shader>>,

    pub time: UniformPtr<f64>,

    /// The contextual data of the program. 
    context: &'context mut Context,

}

impl<'w, 'c, Context> Engine<'w, 'c, Context> {
    pub fn new(window: &'w gpu::winit::window::Window, context: &'c mut Context) -> Self {
        let mut gpu_state = gpu::State::new(window);

        let shader = gpu_state.add_shader("default", program!(
            &gpu_state,
            source: format!("
                @group({GLOBAL}) @binding(0)
                var<uniform> time: f32;

                struct ColorVertexInput {{
                    @location(0) pos: vec3<f32>,
                }}

                struct ColorVertexOutput {{
                    @builtin(position) pos: vec4<f32>,
                }}

                @vertex
                fn vs_main(i: ColorVertexInput) -> ColorVertexOutput {{
                    var o: ColorVertexOutput;
                    o.pos = vec4<f32>(i.pos.x, i.pos.y+sin(time), i.pos.z, 1.0);
                    return o;
                }}

                @fragment
                fn fs_main(iv: ColorVertexOutput) -> @location(0) vec4<f32> {{
                    return vec4<f32>(1.0,0.0,1.0,1.0);
                }}
            ")
        ));

        gpu_state.add_render_pipeline("default", &wgpu::RenderPipelineDescriptor { 
            label: None, 
            layout: None, 
            vertex: wgpu::VertexState { 
                module: &shader.module, 
                entry_point: "vs_main", 
                buffers: &[crate::vertex_type::ColorVertex::desc().clone()] 
            }, 
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, 
                polygon_mode: wgpu::PolygonMode::Fill, 
                ..Default::default()
            }, 

            depth_stencil: None, 
            multisample: wgpu::MultisampleState::default(), 
            fragment: Some(wgpu::FragmentState { 
                module: &shader.module, 
                entry_point: "fs_main", 
                targets: &[
                    Some(wgpu::ColorTargetState { format: gpu_state.config.format.clone(), blend: None, write_mask: wgpu::ColorWrites::all() })
                ] 
            }), 
            multiview: None 
        });

        let time = UniformPtr::new(&gpu_state, 0.0f64);

        let global_bindgroup = gpu_state.init_bindgroup_from_pipeline(None, crate::GLOBAL, &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: time.get_buffer().as_entire_binding(),
            }
        ]).expect("Could not create bindgroup!");

        Self {
            global_bindgroup,
            model_data_cache: HashMap::new(),
            active_entities: vec![],
            gpu_state,
            context,
            scene_objects: vec![],
            shader_cache: HashMap::new(),
            time,
        }
    }

    pub fn update(&mut self, context: &mut Context) {
        for entity in &mut self.active_entities {
            entity.update();
        }
    }

    pub fn render(&self) {

        let mut cmd = self.gpu_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let surfacetexture = self.gpu_state.surface.get_current_texture().unwrap();
        {
            let view = surfacetexture.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut renderpass = cmd.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations{
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    } })
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // TODO: Iterate over scene objects and set
            // renderpass.set_vertex_buffer(0, vb.buffer.slice(..));
            // renderpass.set_bind_group(0, &bindgroup, &[]);
            // renderpass.set_pipeline(pipeline.as_ref());
            renderpass.draw(0..3, 0..1);
        }
        self.gpu_state.queue.submit(std::iter::once(cmd.finish()));
        surfacetexture.present();

    }

    pub fn add_model(&mut self, model_name: &str, model_data: ModelData) -> Rc<ModelData> {
        let r = Rc::new(model_data);
        self.model_data_cache.insert(String::from(model_name), Rc::clone(&r));
        r
    }

    pub fn add_scene_object(&mut self, object_name: &str, using_model: &str) {
        self.scene_objects.push(SceneObject {
            name: String::from(object_name),
            model_data: self.get_model(using_model),
            pipeline_ref: todo!(),
            entity_bindgroup: todo!(),
            model_matrix: todo!(),
        });
    }

    pub fn get_model(&mut self, model_name: &str) -> Rc<ModelData> {
        // TODO: default model
        Rc::clone(self.model_data_cache.get(model_name).unwrap())
    }

    pub fn mut_context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn spawn_active_entity<E: Entity + ActiveEntity + 'c >(&mut self) {
        let mut new_entity: Box<dyn ActiveEntity> = Box::new(E::instantiate::<Context>(self));

        self.active_entities.push(new_entity);

    }

}