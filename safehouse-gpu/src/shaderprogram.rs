use std::rc::Rc;

#[macro_export]
macro_rules! program {
    ($state:expr, source: $src:expr) => {
        Program::new(
            $state, 
            wgpu::ShaderSource::Wgsl(
                $src
                .into() 
            )

        )
    };
}

pub struct Program {
    pub module: Rc<wgpu::ShaderModule>,
}

impl Program {
    pub fn new(display: &crate::State, module: wgpu::ShaderSource<'_>) -> Self {
        let module = display.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: module,
        });

        Self {
            module: Rc::new(module),
        }
    }

}