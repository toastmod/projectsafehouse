use std::{rc::Rc};

pub struct Program {
    pub module: Rc<wgpu::ShaderModule>,
    pub shader_layout: Rc<wgpu::BindGroupLayout>
}

impl Program {
    pub fn new(display: &crate::State, module: wgpu::ShaderSource<'_>, shader_layout_desc: &wgpu::BindGroupLayoutDescriptor) -> Self {
        let module = display.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: module,
        });

        Self {
            module: Rc::new(module),
            shader_layout: Rc::new(display.device.create_bind_group_layout(shader_layout_desc))
        }
    }

    /// Collects bindgroups 0 (global) and 1 (shader)
    pub fn collect_bindgroup_layouts<'a>(&'a self, display: &'a crate::State) -> Vec<&wgpu::BindGroupLayout>{
        let mut bind_group_layouts = match &display.global_bindgroup_layout {
            Some(global_layout) => vec![global_layout.as_ref(),self.shader_layout.as_ref()],
            None => vec![]
            
        };
        // let mut shader_bind_group_layouts: Vec<&wgpu::BindGroupLayout> = self.layouts.iter().map(|x|x.as_ref()).collect();
        // bind_group_layouts.append(&mut shader_bind_group_layouts);
        bind_group_layouts
    }
}