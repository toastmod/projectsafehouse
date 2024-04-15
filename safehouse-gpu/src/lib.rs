pub mod buffer;
pub mod shader;
pub mod bindings;
pub mod texture;
pub mod dataunit;
use std::{collections::HashMap, rc::Rc };

use wgpu::{Backends, InstanceFlags};
use winit::window::Window;

pub struct State {
    // GPU Context 
    pub window: Window,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // Model rendering
    pub shader_programs: HashMap<String, Rc<shader::Program>>,
    pub render_pipelines: HashMap<String, Rc<wgpu::RenderPipeline>>,
    pub texture_samplers: HashMap<String, Rc<wgpu::Sampler>>,
    // Data bindings
    pub global_bindgroup_layout: Option<Rc<wgpu::BindGroupLayout>>,
}

impl State {

    pub fn new(window: Window) -> Self {

        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { 
            backends: Backends::all(),
            ..Default::default() 
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
        .enumerate_adapters(Backends::all())
        .filter(|a|{
            a.is_surface_supported(&surface)
        })
        .next()
        .unwrap();

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )).unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities
                .formats
                .iter()
                .next()
                .unwrap()
                .clone(),
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        let mut shader_programs = HashMap::new();

        let mut render_pipelines = HashMap::new();

        State {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipelines,
            shader_programs,
            global_bindgroup_layout: None,
            texture_samplers: HashMap::new(),
        }
    }

    // TODO: log replacements when adding items 

    pub fn add_render_pipeline(&mut self, name: &str, desc: &wgpu::RenderPipelineDescriptor) -> Rc<wgpu::RenderPipeline> {
        let rp = Rc::new(self.device.create_render_pipeline(desc));
        self.render_pipelines.insert(String::from(name), rp);
        Rc::clone(self.render_pipelines.get(name).unwrap())
    }

    pub fn get_render_pipeline(&self, name: &str) -> Option<Rc<wgpu::RenderPipeline>> {
        Some(Rc::clone(
            self.render_pipelines
                .get(name)
                .expect(&format!("Fatal Error: Pipeline: '{}' not found.", name))
        ))
    }

    pub fn add_shader(&mut self, name: &str, program: shader::Program) {

        self.shader_programs
            .insert(name.to_string(), Rc::new(program));
    }

    pub fn get_shader(&self, name: &str) -> Rc<shader::Program> {
        Rc::clone(
            self.shader_programs
                .get(name)
                .expect(&format!("Fatal Error: Shader: '{}' not found.", name))
        )
    }


}