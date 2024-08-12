pub mod buffer;
pub mod shaderprogram;
pub mod texture;
pub mod dataunit;
pub mod vertex;
pub mod binding;
use std::{collections::HashMap, rc::Rc };
use texture::sampler::TextureSampler;
use wgpu::Backends;
pub use wgpu;
pub use winit;

#[cfg(feature="text")]
pub mod text;

#[cfg(feature="text")]
pub use text::*;

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
    pub texture_samplers: HashMap<String, Rc<TextureSampler>>,
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
                required_limits: wgpu::Limits {
                    ..Default::default()
                },
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

    pub fn update_resize(&mut self) {
        if self.resized {
            println!("Resizing!");
            self.surface.configure(&self.device, &self.config);
            self.resized = false;
        }
    }

    // TODO: log replacements when adding items 

    pub fn add_render_pipeline(&mut self, pipeline_name: &str, desc: &wgpu::RenderPipelineDescriptor) -> Rc<wgpu::RenderPipeline> {
        let rp = Rc::new(self.device.create_render_pipeline(desc));
        self.render_pipelines.insert(String::from(pipeline_name), Rc::clone(&rp));
        rp
    }

    pub fn get_render_pipeline(&self, pipeline_name: &str) -> Option<Rc<wgpu::RenderPipeline>> {
        // TODO: why is this a redundant `Some`? fix this
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

    pub fn get_shader<'a>(&'a self, shader_name: &str) -> &'a Rc<shaderprogram::Program> {
        self.shader_programs
            .get(shader_name)
            .expect(&format!("Fatal Error: Shader: '{}' not found.", shader_name))
        
    }

    pub fn add_sampler(&mut self, sampler_name: &str, sampler: &wgpu::SamplerDescriptor) -> Rc<texture::sampler::TextureSampler> {
        let sampler_rc = Rc::new(TextureSampler::new(&self, sampler));
        self.texture_samplers.insert(String::from(sampler_name), Rc::clone(&sampler_rc));
        sampler_rc
    }

    pub fn get_sampler<'a>(&'a self, sampler_name: &str) -> &'a Rc<TextureSampler> {
        self.texture_samplers
        .get(sampler_name)
        .expect(&format!("Fatal Error: Sampler: '{}' not found.", sampler_name))
    }

    pub fn init_bindgroup_from_pipeline(&self, pipeline_name: &str, bindgroup_index: u32, entries: &[wgpu::BindGroupEntry]) -> Option<(Rc<wgpu::BindGroup>, Rc<wgpu::BindGroupLayout>)> {
        let pipeline_ref = self.get_render_pipeline(pipeline_name)?;
        let bglayout = Rc::new(pipeline_ref.get_bind_group_layout(bindgroup_index));
        Some((Rc::new(self.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: None,
            layout: &bglayout,
            entries
        })), bglayout))
    }


}