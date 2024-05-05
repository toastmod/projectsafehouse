pub mod buffer;
pub mod shaderprogram;
pub mod texture;
pub mod dataunit;
pub mod vertex;
use std::{collections::HashMap, rc::Rc };
use wgpu::Backends;
pub use wgpu;
pub use winit;

pub struct State<'window> {
    // GPU Context 
    pub surface: wgpu::Surface<'window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // Model rendering
    pub shader_programs: HashMap<String, Rc<shaderprogram::Program>>,
    pub render_pipelines: HashMap<String, Rc<wgpu::RenderPipeline>>,
    pub texture_samplers: HashMap<String, Rc<wgpu::Sampler>>,
    // Data bindings
    pub bindgroups: Vec<Rc<wgpu::BindGroupLayout>>,
    pub resized: bool,
}

impl<'window> State<'window> {

    pub fn new(window: &'window winit::window::Window) -> Self {

        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { 
            backends: Backends::all(),
            ..Default::default() 
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
        .enumerate_adapters(Backends::all())
        .into_iter()
        .filter(|a|{
            a.is_surface_supported(&surface)
        })
        .next()
        .unwrap();

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                ..Default::default()
            },
            None,
        )).unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
    
        surface.configure(&device, &config);

        let mut shader_programs = HashMap::new();

        let mut render_pipelines = HashMap::new();

        State {
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines,
            shader_programs,
            texture_samplers: HashMap::new(),
            bindgroups: vec![],
            resized: false, 
        }
    }

    pub fn set_resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.resized = true;
    }

    pub fn update_resize(&self) {
        if self.resized {
            self.surface.configure(&self.device, &self.config);
        }
    }

    // TODO: log replacements when adding items 

    pub fn add_render_pipeline(&mut self, pipeline_name: &str, desc: &wgpu::RenderPipelineDescriptor) -> Rc<wgpu::RenderPipeline> {
        let rp = Rc::new(self.device.create_render_pipeline(desc));
        self.render_pipelines.insert(String::from(pipeline_name), Rc::clone(&rp));
        rp
    }

    pub fn get_render_pipeline(&self, pipeline_name: &str) -> Option<Rc<wgpu::RenderPipeline>> {
        Some(Rc::clone(
            self.render_pipelines
                .get(pipeline_name)
                .expect(&format!("Fatal Error: Pipeline: '{}' not found.", pipeline_name))
        ))
    }

    pub fn add_shader(&mut self, shader_name: &str, program: shaderprogram::Program) -> Rc<shaderprogram::Program>{

        let shader_ref = Rc::new(program);

        self.shader_programs
            .insert(shader_name.to_string(), Rc::clone(&shader_ref));

        shader_ref
    }

    pub fn get_shader(&self, shader_name: &str) -> Rc<shaderprogram::Program> {
        Rc::clone(
            self.shader_programs
                .get(shader_name)
                .expect(&format!("Fatal Error: Shader: '{}' not found.", shader_name))
        )
    }

    pub fn init_bindgroup_from_pipeline(&mut self, pipeline_name: Option<&str>, bindgroup_index: u32, entries: &[wgpu::BindGroupEntry]) -> Option<Rc<wgpu::BindGroup>> {
        let pipeline_ref = self.get_render_pipeline(pipeline_name.unwrap_or("default"))?;
        Some(Rc::new(self.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: None,
            layout: &pipeline_ref.get_bind_group_layout(bindgroup_index),
            entries
        })))
    }


}