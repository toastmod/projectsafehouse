pub use super::bindgroups::{
    BINDGROUP_GLOBAL,
    BINDGROUP_SCENEOBJECT,
};
use std::collections::VecDeque;
use std::sync::Arc;
use std::{collections::HashMap, hash::Hash, num::NonZeroU64, rc::Rc, time::Instant};

use crate::texture::{DynamicTexture, DynamicTextureHandle};
// use crate::bindgroups::BINDGROUP_SHADER;
use crate::{camera::Camera, resource::ManagerResource};
use crate::controller::Controller;
use crate::entity::{Entity, NamedEntity};
use gpu::{buffer::{Buffer, UniformPtr}, program, shaderprogram::Program, vertex::Vertex};
use safehouse_gpu::buffer::Uniform;
use crate::model::ModelData;

pub use safehouse_gpu as gpu;
pub use glam; 
use crate::scene::{SceneObject, SceneObjectHandle, ControllerHandle};

use gpu::wgpu;
use tagmap::TagMap;

pub struct RenderManager {

    pub window: Arc<gpu::winit::window::Window>,
    
    /// The GPU backend state.
    pub gpu_state: gpu::State,

    /// The default fallback rendering pipeline.
    pub default_pipeline: Rc<wgpu::RenderPipeline>,

    // TODO: The default fallback model data.
    // pub default_pipeline: Rc<ModelData>,

    /// The global bindgroup to be used in all shaders.
    global_bindgroup: Rc<wgpu::BindGroup>,

    /// The global bindgroup layout to be used in all shaders.
    global_bglayout: Rc<wgpu::BindGroupLayout>,

    /// The SceneObject bindgroup layout to be used in all shaders.
    sceneobj_bglayout: Rc<wgpu::BindGroupLayout>,

    /// Cache for currently loaded model data.
    model_data_cache: HashMap<String, Rc<ModelData>>, 

    // All active entity bindgroup layouts
    pub(crate) entity_bglayout_cache: HashMap<String, Rc<wgpu::BindGroupLayout>>,

    /// All active entities to be updated by the CPU.
    // active_entities: Vec<Box<dyn ActiveEntity<C> + 'context>>,

    /// All objects to render.
    scene_objects: TagMap<SceneObject>,

    /// All objects to render.
    scene_queue: Vec<usize>,

    /// Cache for currently loaded shaders.
    /// TODO: Implement Shader functionalities
    // shader_cache: HashMap<String, Rc<Shader>>,

    pub time: UniformPtr<f32>,

    pub start_instant: Instant,

    pub last_render_instant: Instant,

    pub global_pvm: Rc<Uniform<glam::Mat4>>,

    pub dynamic_textures: TagMap<DynamicTexture>,

    dyntexture_queue: VecDeque<DynamicTextureHandle>

}

impl RenderManager {
    pub fn new(window: &Arc<gpu::winit::window::Window>) -> Self {
        let mut gpu_state = gpu::State::new(window);

        // let global_pvm = UniformPtr::new(&gpu_state, glam::Mat4::IDENTITY);
        let global_pvm = Uniform::new(&gpu_state, &[glam::Mat4::IDENTITY]);

        let global_bglayout = Rc::new(gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: Some(NonZeroU64::new(std::mem::size_of::<glam::Mat4>() as u64).unwrap())
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: Some(NonZeroU64::new(std::mem::size_of::<f32>() as u64).unwrap())
                    },
                    count: None,
                }
            ],
        }));

        let sceneobj_bglayout = Rc::new(gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: Some(NonZeroU64::new(std::mem::size_of::<glam::Mat4>() as u64).unwrap())
                    },
                    count: None,
                }
            ],
        }));

        let default_pipelayout = gpu_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("default_pipelayout"), 
            bind_group_layouts: &[
                global_bglayout.as_ref(),
                sceneobj_bglayout.as_ref()
            ], 
            push_constant_ranges: &[] 
        });

        let shader = gpu_state.add_shader("default", program!(
            &gpu_state,
            source: format!("
                @group({BINDGROUP_GLOBAL}) @binding(0)
                var<uniform> pvm: mat4x4<f32>;
                @group({BINDGROUP_GLOBAL}) @binding(1)
                var<uniform> time: f32;

                @group({BINDGROUP_SCENEOBJECT}) @binding(0)
                var<uniform> model_mat: mat4x4<f32>;

                struct ColorVertexInput {{
                    @location(0) pos: vec4<f32>,
                    @location(1) color: vec4<f32>,
                }}

                struct ColorVertexOutput {{
                    @builtin(position) pos: vec4<f32>,
                    @location(0) color: vec4<f32>,
                }}

                @vertex
                fn vs_main(i: ColorVertexInput) -> ColorVertexOutput {{
                    var o: ColorVertexOutput;
                    o.color = i.color;
                    var t = time;
                    o.pos = pvm * vec4<f32>(i.pos.x, i.pos.y+(sin(time)*0.1), i.pos.z, i.pos.w);
                    return o;
                }}

                @fragment
                fn fs_main(iv: ColorVertexOutput) -> @location(0) vec4<f32> {{
                    return vec4<f32>(iv.color.x,iv.color.y,iv.color.z,iv.color.w);
                }}
            ")
        ));

        let default_pipeline = gpu_state.add_render_pipeline("default", &wgpu::RenderPipelineDescriptor { 
            label: None, 
            layout: Some(&default_pipelayout), 
            vertex: wgpu::VertexState { 
                module: &shader.module, 
                entry_point: Some("vs_main"), 
                buffers: &[crate::vertex_type::ColorVertex::desc().clone()],
                compilation_options: Default::default(), 
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
                entry_point: Some("fs_main"), 
                targets: &[
                    Some(wgpu::ColorTargetState { format: gpu_state.config.format.clone(), blend: None, write_mask: wgpu::ColorWrites::ALL })
                ],
                compilation_options: Default::default(), 
            }), 
            multiview: None,
            cache: None
        });

        gpu_state.add_sampler("default", &wgpu::SamplerDescriptor { 
            label: Some("default"), 
            ..Default::default()
        });

        let time = UniformPtr::new(&gpu_state, 0.0f32);

        let global_bindgroup = Rc::new(gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("global_bindgroup"),
            layout: &global_bglayout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: global_pvm.get_buffer().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: time.get_buffer().as_entire_binding(),
                },
            ],
        }));

        let start_instant = Instant::now();


        // let mut controllers = TagMap::new();

        Self {
            global_bindgroup,
            global_bglayout,
            sceneobj_bglayout,
            model_data_cache: HashMap::new(),
            // active_entities: vec![],
            gpu_state,
            // context,
            scene_objects: TagMap::new(),
            scene_queue: vec![],
            // shader_cache: HashMap::new(),
            time,
            default_pipeline,
            window: Arc::clone(window),
            start_instant,
            last_render_instant: Instant::now(),
            entity_bglayout_cache: HashMap::new(),
            dynamic_textures: TagMap::new(),
            dyntexture_queue: VecDeque::new(),
            global_pvm,
        }
    }

    /// Update the amount of time elapsed since the renderer started.
    pub fn update_time(&mut self) {
        *self.time.as_mut() = self.start_instant.elapsed().as_secs_f32();
    }

    /// Calculate and update the global PVM matrix using a `Camera` (PV) and a model (M)
    pub fn update_pvm(&self, camera: &Camera, model: &glam::Mat4) {
        self.global_pvm.update(&self.gpu_state, &[
            camera.calc_pvm(model)
        ]);
    }

    pub fn queue_dyn_texture(&mut self, handle: DynamicTextureHandle) {
        self.dyntexture_queue.push_back(handle);
    }

    pub fn add_dyn_texture(&mut self, dt: DynamicTexture) -> DynamicTextureHandle {
        self.dynamic_textures.add(dt)
    }

    pub fn get_dyn_texture(&mut self, handle: DynamicTextureHandle) -> Option<&mut DynamicTexture> {
        self.dynamic_textures[handle].as_mut()
    }

    pub fn render_dyn_textures(&mut self) {
        while !self.dyntexture_queue.is_empty() {
            let dyntex = if let Some(dt) = self.dyntexture_queue.pop_back() {
                if let Some(dtt) = self.dynamic_textures[dt].as_mut() {
                    dtt
                }else {
                    println!("A dynamic texture was queued but it's index did not contain data.");
                    continue;
                }
            } else {
                continue;
            };
        }
    }

    pub fn render<'pass>(&mut self, camera: &Camera) {

        let mut cmd = self.gpu_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        let surfacetexture = self.gpu_state.surface.get_current_texture().unwrap();
        {

            while !self.dyntexture_queue.is_empty() {
                let dyntex = if let Some(dt) = self.dyntexture_queue.pop_back() {
                    if let Some(dtt) = self.dynamic_textures[dt].as_mut() {
                        dtt.render_self(&mut cmd);
                    }else {
                        println!("A dynamic texture was queued but it's index did not contain data.");
                        continue;
                    }
                } else {
                    continue;
                };
            }

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
            renderpass.set_bind_group(BINDGROUP_GLOBAL, self.global_bindgroup.as_ref(), &[]);

            // update globals
            self.time.update(&self.gpu_state);
            // self.camera.force_camera_update_flag

            // Render each SceneObject
            for objhandle in &self.scene_queue {

                // Get object reference
                let obj = self.get_scene_object(*objhandle).unwrap();

                // Update model matrix only at render time (now)
                obj.update_matrix(self);
                
                // Set the SceneObject bindgroup for this object
                renderpass.set_bind_group(BINDGROUP_SCENEOBJECT, obj.sceneobject_bindgroup.as_ref(), &[]);

                let mut curbg_id = BINDGROUP_SCENEOBJECT+1;
                
                // Set model BG if there is one
                if let Some(mbg) = obj.model_data.model_bindgroup.as_ref() {
                    renderpass.set_bind_group(curbg_id, mbg.0.as_ref(), &[]);
                    curbg_id +=1;

                }

                // Entity BG should only be active if it's model is, otherwise it wouldn't make sense to use the shader.
                if let Some(ebg) = obj.entity_bindgroup.as_ref() {
                    renderpass.set_bind_group(curbg_id, ebg.as_ref(), &[]);
                }

                // TODO: impl shader bindgroup

                // Set the model's vertex buffer
                renderpass.set_vertex_buffer(0, obj.model_data.vertex_buffer.buffer.slice(..));
                
                // Set the entity's pipeline type
                renderpass.set_pipeline(obj.pipeline_ref.as_ref().unwrap_or(&self.default_pipeline));

                // Render each group of vertices
                for group in obj.model_data.groups.iter().cloned() {
                    self.update_pvm(camera, obj.model_matrix.as_ref());
                    renderpass.draw(group, 0..1);
                }
                
            }

        }
        self.gpu_state.queue.submit(std::iter::once(cmd.finish()));
        surfacetexture.present();

    }

    // pub fn add_controller(&mut self) -> ControllerHandle {
    //     self.controllers.add(Controller::new(None))
    // }

    // pub fn build_bindgroup(&mut self) {
        
    // }

    pub fn add_model(&mut self, model_name: &str, model_data: ModelData) -> Rc<ModelData> {
        let r = Rc::new(model_data);
        self.model_data_cache.insert(String::from(model_name), Rc::clone(&r));
        r
    }

    pub fn load_entity<E: Entity + NamedEntity>(&mut self) {

        println!("Loading Entity: {}", E::ENTITY_TYPE_NAME);

        let bindings = E::load_bindings();

        let entity_bglayout = if bindings.len() > 0 {
            Some(match self.get_entity_layout(E::bindings_name()) {
                Some(layout) => Rc::clone(layout),
                None => {
                    let entries: Vec<wgpu::BindGroupLayoutEntry> = E::load_bindings().iter().map(|b| b.get_layout_entry()).collect(); 
                    self.add_entity_layout(E::bindings_name(), &entries)
                },
            })

        } else {
            None
        };

        let model = E::load_model(&mut self.gpu_state);
        
        
        let mut total_layout = vec![
            self.global_bglayout.as_ref(),
            self.sceneobj_bglayout.as_ref(),
        ];

        let mut group_entity = 0u32;
        let mut group_model = 0u32;
        // Note: Shader bg is currently ignored
        // let mut group_shader = BINDGROUP_SHADER;

        if let Some((_,layout)) = &model.model_bindgroup {
            total_layout.push(&layout);
            group_model = (total_layout.len()-1) as u32;
        }

        if let Some(layout) = &entity_bglayout {
            total_layout.push(&layout);
            group_entity = (total_layout.len()-1) as u32;
        }

        let shader_load = E::load_shader(&self, group_model, group_entity);
        match shader_load {
            Some(prog) => {
                self.gpu_state.add_shader(E::shader_name(), prog);
            },
            None => (),
        };

        match E::load_pipeline(&self) {
            Some(pipeargs) => {

                let shader = self.gpu_state.get_shader(E::shader_name());

                let pipe_layout = Rc::new(self.gpu_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &total_layout,
                    push_constant_ranges: &[] 
                }));   
                
                // Create pipeline
                // NOTE: pseudo of self.gpu_state.add_render_pipeline
                let pipe = Rc::new(self.gpu_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(E::pipeline_name()),
                    layout: Some(&pipe_layout),
                    vertex: wgpu::VertexState {
                        module: &shader.module, 
                        entry_point: Some("vs_main"), 
                        buffers: &[model.vertex_buffer.desc.clone()],
                        compilation_options: Default::default(), 
                    },
                    primitive: pipeargs.primitive,
                    depth_stencil: pipeargs.depth_stencil,
                    fragment: Some(wgpu::FragmentState {
                        module: &shader.module,
                        entry_point: Some("fs_main"),
                        // TODO: Support for multiple color targets
                        targets: &[Some(wgpu::ColorTargetState {
                            format: self.gpu_state.config.format.clone(),
                            blend: None,
                            write_mask: wgpu::ColorWrites::all() 
                        })],
                        compilation_options: Default::default(), 
                    }),
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None, 
                    cache: None
                }));
        
                self.gpu_state.render_pipelines.insert(String::from(E::pipeline_name()), Rc::clone(&pipe));
            },
            None => {
                // Use default if not specified
                self.gpu_state.render_pipelines.insert(String::from(E::pipeline_name()), Rc::clone(&self.default_pipeline));
            },
        };

        self.add_model(E::model_name(), model);



    }

    pub fn add_scene_object(&mut self, object_name: &str, using_model: &str, using_pipeline: &str) -> SceneObjectHandle {

        let model_matrix = UniformPtr::new(&self.gpu_state, glam::Mat4::IDENTITY); 

        let sceneobject_bindgroup = Rc::new(self.gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(object_name),
            layout: &self.sceneobj_bglayout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_matrix.get_buffer().as_entire_binding(),
                }
            ],
        }));

        //TODO: resolve consistency of loading &Rc vs Rc for load functions 
        // Should search() return a cloned ref, while the functions return &Rc?

        let sceneobj_handle = self.scene_objects.add(SceneObject {
            name: String::from(object_name),
            model_data: ModelData::fetch(using_model, self),
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

    pub fn get_model(&self, model_name: &str) -> Option<&Rc<ModelData>> {
        // TODO: default pipeline 
        // TODO: Implement a proper error convention
        self.model_data_cache.get(model_name)
    }

    pub fn get_entity_layout(&self, layout_name: &str) -> Option<&Rc<wgpu::BindGroupLayout>> {
        self.entity_bglayout_cache.get(layout_name)
    }

    pub fn add_entity_layout(&mut self, layout_name: &str, entries: &[wgpu::BindGroupLayoutEntry]) -> Rc<wgpu::BindGroupLayout> {
        let layout = Rc::new(self.gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(layout_name),
            entries: &entries
        }));
        self.entity_bglayout_cache.insert(String::from(layout_name), Rc::clone(&layout));
        layout
    }

    // pub fn mut_context(&mut self) -> &mut C {
    //     &mut self.context
    // }

    // pub fn get_context(&self) -> &C {
    //     &self.context
    // }

    /// Spawn an entity as a static SceneObject.\ 
    /// Note: entities should only contain references to the context.
    pub fn spawn_sceneobject_entity<E: Entity + NamedEntity>(&mut self, name: &str) -> E {
        
        // Create a SceneObject to accompany entity
        let sceneobject_handle = self.add_scene_object(name, E::model_name(), E::pipeline_name());

        // Instantiate the entity.
        let e = E::on_instantiate(self, sceneobject_handle);

        // Load the bindings
        let bindings = E::load_bindings();
        if bindings.len() == 0 {
            return e;
        }
        
        // Check if a bindgroup layout was created yet
        let (layout, bg_entries) = match self.entity_bglayout_cache.get(E::bindings_name()) {
            
            // Layout is already in memory, use it.
            Some(l) => {
                let layout = Rc::clone(&l);

                let bg_entries: Vec<wgpu::BindGroupEntry> = bindings.iter().map(|x| {
                    x.get_binding_entry(&e)
                }).collect();

                (layout, bg_entries)

            },

            // Make the new layout
            None => panic!("Entity BindGroupLayout not loaded!"),

        };

        let bg = Rc::new(self.gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(E::bindings_name()),
            layout: &layout,
            entries: &bg_entries 
        }));

        self.mut_scene_object(sceneobject_handle).unwrap().entity_bindgroup = Some(bg);

        e

    }

    /// Window width
    pub fn w_width(&self) -> f32 {
        self.gpu_state.config.width as f32
    }

    /// Window height
    pub fn w_height(&self) -> f32 {
        self.gpu_state.config.height as f32
    }

    pub fn world_to_window_coord(&self, x: f32, y: f32) -> (f32,f32) {
        (
            ((x+1.0)/2.0)*self.w_width(),
            ((y+1.0)/2.0)*self.w_height()
        )
    }

    pub fn window_to_world_coord(&self, x: f32, y: f32) -> (f32,f32) {
        (
            ((x/self.w_width())*2.0)-1.0,
            ((y/self.w_height())*2.0)-1.0
        )
    }

}