pub use super::bindgroups::{
    BINDGROUP_GLOBAL,
    BINDGROUP_SCENEOBJECT,
    BINDGROUP_MODEL,
    BINDGROUP_ENTITY
};
use std::iter::{Enumerate, Map, Zip};
use std::ops::Range;
use std::{collections::HashMap, rc::Rc, time::Instant};

use crate::{camera::Camera, model};
use crate::controller::Controller;
use crate::entity::{self, Entity};
use gpu::{buffer::{Buffer, UniformPtr}, program, shaderprogram::Program, vertex::Vertex};
use safehouse_gpu::buffer::{Bindable, Uniform};
use safehouse_gpu::winit::event::MouseScrollDelta;
use crate::model::ModelData;

pub use safehouse_gpu as gpu;
pub use glam; 
use crate::scene::{SceneObject, SceneObjectHandle, ControllerHandle};
use crate::shader::Shader;

use gpu::wgpu;
use tagmap::TagMap;

pub struct RenderManager<'window> {

    pub window: &'window gpu::winit::window::Window,
    
    /// The GPU backend state.
    pub gpu_state: gpu::State<'window>,

    /// The default fallback rendering pipeline.
    pub default_pipeline: Rc<wgpu::RenderPipeline>,

    pub default_shader: Rc<Program>,

    // TODO: The default fallback model data.
    // pub default_pipeline: Rc<ModelData>,

    /// The global bindgroup to be used in all shaders.
    global_bindgroup: Rc<wgpu::BindGroup>,
    global_bglayout: wgpu::BindGroupLayout,

    // The SceneObject Bindgroup layout
    sceneobject_bglayout: wgpu::BindGroupLayout,

    /// Cache for currently loaded model data.
    model_data_cache: HashMap<String, Rc<ModelData>>, 

    // Cache for currently loaded entity bindgroups.
    entity_bglayout_cache: HashMap<String, Rc<wgpu::BindGroupLayout>>, 

    /// All active entities to be updated by the CPU.
    // active_entities: Vec<Box<dyn ActiveEntity<C> + 'context>>,

    /// All objects to render.
    scene_objects: TagMap<SceneObject>,

    /// All objects to render.
    scene_queue: Vec<usize>,

    /// Cache for currently loaded shaders.
    /// TODO: Implement Shader functionalities
    shader_cache: HashMap<String, Rc<Shader>>,

    pub time: UniformPtr<f32>,

    pub start_instant: Instant,

    pub last_render_instant: Instant,

    pub camera: Camera,

    pub controllers: TagMap<Controller>

}

impl<'w> RenderManager<'w> {
    pub fn new(window: &'w gpu::winit::window::Window) -> Self {
        let mut gpu_state = gpu::State::new(window);

        let shader = gpu_state.add_shader("default", program!(
            &gpu_state,
            source: format!("
                @group({BINDGROUP_GLOBAL}) @binding(0)
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
                    o.pos = model_mat * vec4<f32>(i.pos.x, i.pos.y+(sin(time)*0.1), i.pos.z, i.pos.w);
                    return o;
                }}

                @fragment
                fn fs_main(iv: ColorVertexOutput) -> @location(0) vec4<f32> {{
                    return vec4<f32>(iv.color.x,iv.color.y,iv.color.z,iv.color.w);
                }}
            ")
        ));

        let default_pipeline = gpu_state.add_render_pipeline("default", wgpu::RenderPipelineDescriptor { 
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


        // Global Bindings
        let time = UniformPtr::new(&gpu_state, 0.0f32);

        // SceneObject Binding Inits
        let model_mat = Uniform::new(&gpu_state, &glam::Mat4::IDENTITY.to_cols_array());


        let global_bglayout = gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bindgroup_global_bglayout"),
            entries: &[
                UniformPtr::<f32>::get_layout_entry(0, wgpu::ShaderStages::all()),
            ] 
        });

        let sceneobject_bglayout = gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bindgroup_sceneobject_bglayout"),
            entries: &[
                Uniform::<f32>::get_layout_entry(0, wgpu::ShaderStages::all()),
            ] 
        });

        // Create the global bindgroup data
        // TODO: does this need to be RC'd? does the layout as well?
        let global_bindgroup = Rc::new(gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bindgroup_global"),
            layout: &global_bglayout,
            entries: &[
                time.get_binding_entry(0)
            ] 
        }));

        let start_instant = Instant::now();

        let camera = Camera::new(gpu_state.config.width as f32, gpu_state.config.height as f32);

        let mut controllers = TagMap::new();

        Self {
            global_bindgroup,
            global_bglayout,
            sceneobject_bglayout,
            model_data_cache: HashMap::new(),
            entity_bglayout_cache: HashMap::new(),
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
            camera,
            controllers,
            last_render_instant: Instant::now(),
            default_shader: shader,
        }

    }

    /// Update the amount of time elapsed since the renderer started.
    pub fn update_time(&mut self) {
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
            renderpass.set_bind_group(BINDGROUP_GLOBAL, &self.global_bindgroup, &[]);

            // update globals
            self.time.update(&self.gpu_state);

            // Render each SceneObject
            for objhandle in &self.scene_queue {

                // Get object reference
                let obj = self.get_scene_object(*objhandle).unwrap();

                // Update model matrix only at render time (now)
                obj.update_matrix(&self.gpu_state);
                
                // Set the SceneObject bindgroup for this object
                renderpass.set_bind_group(BINDGROUP_SCENEOBJECT, &obj.sceneobject_bindgroup, &[]);
                
                // Set model BG if there is one
                if let Some(mbg) = obj.model_data.model_bindgroup.as_ref() {
                    renderpass.set_bind_group(BINDGROUP_MODEL, &mbg.0, &[]);

                }

                // Entity BG should only be active if it's model is, otherwise it wouldn't make sense to use the shader.
                if let Some(ebg) = obj.entity_bindgroup.as_ref() {
                    renderpass.set_bind_group(BINDGROUP_ENTITY, ebg, &[]);
                }

                // Set the model's vertex buffer
                renderpass.set_vertex_buffer(0, obj.model_data.vertex_buffer.buffer.slice(..));
                
                // Set the entity's pipeline type
                renderpass.set_pipeline(obj.pipeline_ref.as_ref().unwrap_or(&self.default_pipeline));

                // Render each group of vertices
                for group in obj.model_data.groups.iter().cloned() {
                    renderpass.draw(group, 0..1);
                }
                
            }

        }
        self.gpu_state.queue.submit(std::iter::once(cmd.finish()));
        surfacetexture.present();

    }

    pub fn add_controller(&mut self) -> ControllerHandle {
        self.controllers.add(Controller::new(None))
    }

    pub fn add_model(
        &mut self,
        model_name: &str,
        vertex_buffer: Rc<gpu::buffer::VertexBuffer>,
        textures: Option<Vec<gpu::texture::Texture>>,
        groups: Box<[Range<u32>]>
        // TODO: animation data
    ) -> Rc<ModelData> {
        let mut layout_label = String::new();
        layout_label.push_str(model_name);
        layout_label.push_str("_bglayout"); 

        // TODO: match on other types of resources
        let model_bindgroup = if let Some(layout_entries_iter) = match &textures {
            Some(txt) => Some(
                txt
                .iter().enumerate()
                .map(|(slot, x)| {
                    gpu::texture::Texture::get_layout_entry(slot as u32, wgpu::ShaderStages::all())
                }).zip(
                    txt
                    .iter().enumerate()
                    .map(|(slot, x)| {
                        x.get_binding_entry(slot as u32)
                    })
                )
            ),
            None => None,
        } {
            let (layout_entries, binding_entries): (Vec<wgpu::BindGroupLayoutEntry>, Vec<wgpu::BindGroupEntry>) = layout_entries_iter.unzip();
            let model_layout = Rc::new(self.gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&layout_label), entries: &layout_entries
            }));
            
            let mut bg_label = String::new();
            bg_label.push_str(model_name);
            bg_label.push_str("_bg"); 

            let model_bg = Rc::new(self.gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&bg_label),
                layout: &model_layout,
                entries: &binding_entries
            }));
    
            Some((model_bg, model_layout))
        }else{
            None
        };
        
        let r = Rc::new(ModelData {
            vertex_buffer,
            textures,
            model_bindgroup,
            groups
        });

        self.model_data_cache.insert(String::from(model_name), Rc::clone(&r));

        r

    }

    fn generate_bglayout_name<E: Entity>() -> String {
        let mut bglayout_label = String::new();
        bglayout_label.push_str(&E::entity_type_name());
        bglayout_label.push_str("_bglayout"); 
        bglayout_label
    }

    fn generate_pipeline_layout_name<E: Entity>() -> String {
        let mut p_layout_label = String::new();
        p_layout_label.push_str(&E::entity_type_name());
        p_layout_label.push_str("_pipelayout"); 
        p_layout_label
    }

    fn generate_shader_name<E: Entity>() -> String {
        let mut shader_label = String::new();
        shader_label.push_str(&E::entity_type_name());
        shader_label.push_str("_shader"); 
        shader_label
    }

    fn generate_bindgroup_name(object_name: &str) -> String {
        let mut bg_label = String::new();
        bg_label.push_str(object_name);
        bg_label.push_str("_bg"); 
        bg_label
    }

    pub fn load_entity<E: Entity + Sized>(&mut self) {

        // Model loading
        let model = E::load_model(self);

        // Get static layouts
        let mut all_layouts = vec![
            &self.global_bglayout,
            &self.sceneobject_bglayout,
        ];

        // Get model bindgroup layout
        let model_layout_ref = model.as_ref().unwrap().model_bindgroup.as_ref().unwrap().1.as_ref();
        // TODO: for now it is assumed there is always a model bindgroup
        all_layouts.push(model_layout_ref);
        
        // Load entity bindgroup layout
        let bglayout_label = Self::generate_bglayout_name::<E>();

        let entity_layout: Vec<wgpu::BindGroupLayoutEntry> = E::load_entity_bindings().iter().map(|x| x.2).collect();
        let entity_bglayout = Rc::new(self.gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&bglayout_label),
            entries: &entity_layout
        }));

        all_layouts.push(&entity_bglayout);

        let mut p_layout_label = Self::generate_pipeline_layout_name::<E>(); 

        let pipe_layout = self.gpu_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
                label: Some(&p_layout_label),
                bind_group_layouts: &all_layouts,
                push_constant_ranges: &[],
        });

        // Shader loading
        let prg = E::load_shader(self);
        let shader_module = match prg {
            Some(p) => {
                let shader_name = Self::generate_shader_name::<E>();
                self.gpu_state.add_shader(&shader_name, p);
                self.gpu_state.get_shader(&shader_name)

            },
            None => Rc::clone(&self.default_shader),
        };
        
        // Pipeline loading
        let pipeline_name = &E::pipeline_name();
        if let Some (rp_desc) = match E::load_pipeline(self, &shader_module.as_ref().module) {
            entity::EntityPipeline::Default => None,
            entity::EntityPipeline::Custom { vertex, fragment, primitive, depth_stencil } => {
        
                Some(wgpu::RenderPipelineDescriptor {
                    label: Some(&pipeline_name),
                    layout: Some(&pipe_layout),
                    vertex,
                    primitive,
                    depth_stencil,
                    fragment,
                    // TODO: Multisampling
                    multisample: wgpu::MultisampleState::default(), 
                    multiview: None 
                })

                
            },
        } {
            // TODO: Due to mutability issues, pseudo-ing self.gpu_state.add_render_pipeline here
            // In the future, should probably fix this to work with the actual function
            let rp = Rc::new(self.gpu_state.device.create_render_pipeline(&rp_desc));
            self.gpu_state.render_pipelines.insert(String::from(pipeline_name), Rc::clone(&rp));
            self.entity_bglayout_cache.insert(bglayout_label, entity_bglayout);

        }


    }

    pub fn add_scene_object(&mut self, object_name: &str, using_model: &str, using_pipeline: &str) -> SceneObjectHandle {

        let model_matrix = UniformPtr::new(&self.gpu_state, glam::Mat4::IDENTITY); 

        let sceneobject_bindgroup = self.gpu_state.init_bindgroup_from_pipeline(using_pipeline, "bindgroup_sceneobject", BINDGROUP_SCENEOBJECT, &[
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

    /// Spawn an entity as a static SceneObject.\ 
    /// Note: entities should only contain references to the context.
    pub fn spawn_sceneobject_entity<E: Entity + Sized>(&mut self, name: &str) -> E {
        
        // Create a SceneObject to accompany entity
        let sceneobject_handle = self.add_scene_object(name, &E::model_name(), &E::pipeline_name());
        
        // Instantiate the entity.
        let new_entity = E::on_instantiate(self, sceneobject_handle);

        // Create an entity bindgroup if required by entity.
        let binds = E::load_entity_bindings();
        if binds.len() >= 1 {
            let entity_entries: Vec<wgpu::BindGroupEntry> = binds.iter().map(|x| (x.1)(&new_entity)).collect();
            let entity_bindgroup = self.gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&Self::generate_bindgroup_name(E::entity_type_name())),
                layout: self.entity_bglayout_cache.get(&Self::generate_bglayout_name::<E>()).expect("Could not fetch bglayout"),
                entries: &entity_entries
            });

            self.mut_scene_object(sceneobject_handle).unwrap().attach_entity_bindgroup(Rc::new(entity_bindgroup));
        }

        new_entity
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
            -((y+1.0)/2.0)*self.w_height()
        )
    }

    pub fn window_to_world_coord(&self, x: f32, y: f32) -> (f32,f32) {
        (
            ((x/self.w_width())*2.0)-1.0,
            ((y/self.w_height())*2.0)-1.0
        )
    }

}