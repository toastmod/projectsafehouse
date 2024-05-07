use std::{collections::HashMap, marker::PhantomData, num::NonZeroU64, rc::Rc, time::Instant};

use context::Context;
use entity::Entity;
use gpu::{buffer::{Buffer, UniformPtr}, program, vertex::Vertex};
use model::{obj, ModelData};

pub use safehouse_gpu as gpu;
pub use glam; 
use scene::{SceneObject, SceneObjectHandle};
use shader::Shader;

use gpu::wgpu;
use tagmap::TagMap;

pub mod entity;
pub mod model;
pub mod scene;
pub mod vertex_type;
pub mod shader;
pub mod context;
pub mod camera;
pub mod controller;
pub mod utils;

/// Bindings used by all shaders, governed by the engine.\
/// E.g: time, camera, debug flags, etc.
pub const GLOBAL: u32 = 0;

// /// Bindings used by all shaders, governed by the engine.\
// /// E.g: time, camera, debug flags, etc.
// pub const CONTEXT: u32 = 1;


/// Bindings for shader specific options.
pub const SCENEOBJECT: u32 = 1;

/// Bindings to model data.\
/// E.g: textures, skeleton animation variables, etc.
pub const MODEL: u32 = 2;

/// Bindings for GPU-side updates of an entity.
pub const ENTITY: u32 = 3;

pub struct Engine<'window> {

    pub window: &'window gpu::winit::window::Window,
    
    /// The GPU backend state.
    pub gpu_state: gpu::State<'window>,

    /// The default fallback rendering pipeline.
    pub default_pipeline: Rc<wgpu::RenderPipeline>,

    // TODO: The default fallback model data.
    // pub default_pipeline: Rc<ModelData>,

    /// The global bindgroup to be used in all shaders.
    global_bindgroup: Rc<wgpu::BindGroup>,

    /// Cache for currently loaded model data.
    model_data_cache: HashMap<String, Rc<ModelData>>, 

    /// All active entities to be updated by the CPU.
    // active_entities: Vec<Box<dyn ActiveEntity<C> + 'context>>,

    /// All objects to render.
    scene_objects: TagMap<SceneObject>,

    /// All objects to render.
    scene_queue: Vec<usize>,

    /// Cache for currently loaded shaders.
    shader_cache: HashMap<String, Rc<Shader>>,

    pub time: UniformPtr<f32>,

    pub start_instant: Instant,

    // TODO: The contextual data of the program. 
    // context: &'context mut C,

}

impl<'w> Engine<'w> {
    pub fn new(window: &'w gpu::winit::window::Window) -> Self {
        let mut gpu_state = gpu::State::new(window);

        let shader = gpu_state.add_shader("default", program!(
            &gpu_state,
            source: format!("
                @group({GLOBAL}) @binding(0)
                var<uniform> time: f32;

                @group({SCENEOBJECT}) @binding(0)
                var<uniform> model_mat: mat4x4<f32>;

                struct ColorVertexInput {{
                    @location(0) pos: vec3<f32>,
                    @location(1) color: vec3<f32>,
                }}

                struct ColorVertexOutput {{
                    @builtin(position) pos: vec4<f32>,
                    @location(0) color: vec3<f32>,
                }}

                @vertex
                fn vs_main(i: ColorVertexInput) -> ColorVertexOutput {{
                    var o: ColorVertexOutput;
                    o.color = i.color;
                    var t = time;
                    o.pos = vec4<f32>(i.pos.x, i.pos.y+(sin(time)*0.1), i.pos.z, 1.0) * model_mat;
                    return o;
                }}

                @fragment
                fn fs_main(iv: ColorVertexOutput) -> @location(0) vec4<f32> {{
                    return vec4<f32>(iv.color.x,iv.color.y,iv.color.z,1.0);
                }}
            ")
        ));

        let default_pipeline = gpu_state.add_render_pipeline("default", &wgpu::RenderPipelineDescriptor { 
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
                front_face: wgpu::FrontFace::Cw,
                ..Default::default()
            }, 

            depth_stencil: None, 
            multisample: wgpu::MultisampleState::default(), 
            fragment: Some(wgpu::FragmentState { 
                module: &shader.module, 
                entry_point: "fs_main", 
                targets: &[
                    Some(wgpu::ColorTargetState { format: gpu_state.config.format.clone(), blend: Some(wgpu::BlendState::ALPHA_BLENDING), write_mask: wgpu::ColorWrites::ALL })
                ] 
            }), 
            multiview: None 
        });

        let time = UniformPtr::new(&gpu_state, 0.0f32);

        let global_bindgroup = gpu_state.init_bindgroup_from_pipeline("default", crate::GLOBAL, &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: time.get_buffer().as_entire_binding(),
            }
        ]).expect("Could not create bindgroup!");

        let start_instant = Instant::now();

        Self {
            global_bindgroup,
            model_data_cache: HashMap::new(),
            // active_entities: vec![],
            gpu_state,
            // context,
            scene_objects: TagMap::new(),
            scene_queue: vec![],
            shader_cache: HashMap::new(),
            time,
            default_pipeline,
            window,
            start_instant,
        }

    }

    pub fn update_delta_time(&mut self) {
        *self.time.as_mut() = self.start_instant.elapsed().as_secs_f32();
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    } })
                ],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set global bindgroup
            renderpass.set_bind_group(crate::GLOBAL, &self.global_bindgroup, &[]);

            // update globals
            self.time.update(&self.gpu_state);

            // Render each SceneObject
            for objhandle in &self.scene_queue {

                let obj = self.get_scene_object(*objhandle).unwrap();

                obj.update_matrix(&self.gpu_state);

                renderpass.set_vertex_buffer(0, obj.model_data.vertex_buffer.buffer.slice(..));

                if let Some(mbg) = obj.model_data.model_bindgroup.as_ref() {
                    renderpass.set_bind_group(crate::MODEL, mbg, &[]);
                }

                if let Some(ebg) = obj.entity_bindgroup.as_ref() {
                    renderpass.set_bind_group(crate::ENTITY, ebg, &[]);
                }

                renderpass.set_bind_group(crate::SCENEOBJECT, &obj.sceneobject_bindgroup, &[]);

                renderpass.set_pipeline(obj.pipeline_ref.as_ref().unwrap_or(&self.default_pipeline));

                for group in obj.model_data.groups.iter().cloned() {
                    renderpass.draw(group, 0..1);
                }
                
            }

        }
        self.gpu_state.queue.submit(std::iter::once(cmd.finish()));
        surfacetexture.present();

    }

    pub fn add_model(&mut self, model_name: &str, model_data: ModelData) -> Rc<ModelData> {
        let r = Rc::new(model_data);
        self.model_data_cache.insert(String::from(model_name), Rc::clone(&r));
        r
    }

    pub fn add_scene_object(&mut self, object_name: &str, using_model: &str, using_pipeline: &str) -> SceneObjectHandle {

        let model_matrix = UniformPtr::new(&self.gpu_state, glam::Mat4::IDENTITY); 

        // let bg_layout = self.gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::VERTEX,
        //             ty: wgpu::BindingType::Buffer { 
        //                 ty: wgpu::BufferBindingType::Uniform, 
        //                 has_dynamic_offset: false, 
        //                 min_binding_size: Some(NonZeroU64::new(std::mem::size_of::<glam::Mat4>() as u64).unwrap())
        //             },
        //             count: None,
        //         }
        //     ],
        // });

        let sceneobject_bindgroup = self.gpu_state.init_bindgroup_from_pipeline(using_pipeline, crate::SCENEOBJECT, &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: model_matrix.get_buffer().as_entire_binding(),
            }
        ]).unwrap();

        let sceneobj_handle = self.scene_objects.add(SceneObject {
            name: String::from(object_name),
            model_data: self.get_model(using_model),
            pipeline_ref: self.get_pipeline(using_pipeline),
            entity_bindgroup: None,
            model_matrix,
            sceneobject_bindgroup,
            model_matrix_changed: false,
        });

        self.scene_queue.push(sceneobj_handle.clone());

        sceneobj_handle

    }

    pub fn get_scene_object(&self, handle: SceneObjectHandle) -> Option<&SceneObject> {
        self.scene_objects[handle].as_ref()
    }

    pub fn mut_scene_object(&mut self, handle: SceneObjectHandle) -> Option<&mut SceneObject> {
        self.scene_objects[handle].as_mut()
    }

    pub fn get_pipeline(&self, pipeline_name: &str) -> Option<Rc<wgpu::RenderPipeline>> {
        // TODO: default model
        self.gpu_state.get_render_pipeline(pipeline_name)
    }

    pub fn get_model(&self, model_name: &str) -> Rc<ModelData> {
        // TODO: default pipeline 
        Rc::clone(self.model_data_cache.get(model_name).unwrap())
    }

    // pub fn mut_context(&mut self) -> &mut C {
    //     &mut self.context
    // }

    // pub fn get_context(&self) -> &C {
    //     &self.context
    // }

    /// Spawn an entity as a static SceneObject.\ 
    /// Note: entities should only contain references to the context.
    pub fn spawn_sceneobject_entity<E: Entity>(&mut self, name: &str) -> E {
        
        // Create a SceneObject to accompany entity
        let sceneobject_handle = self.add_scene_object(name, E::model_name(), E::pipeline_name());

        // Instantiate the entity.
        E::on_instantiate(self, sceneobject_handle)
    }

    // TODO: Spawn an active entity.\ 
    // TODO: Note: entities should only contain references to the context.
    // pub fn spawn_active_entity<E: Entity<C> + ActiveEntity<C> + 'c >(&mut self, name: &str) {

    //     // Create the static SceneObject Entity
    //     let sceneobject_entity: Box<dyn ActiveEntity<C>> = self.spawn_sceneobject_entity(name) as Box<E>;

    //     // Add the Entity into active entities list to take part in the CPU update cycle. 
    //     self.active_entities.push(sceneobject_entity);

    // }

}