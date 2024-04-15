use std::sync::Arc;

/// Anything that can produce a wgpu::BindingResource
/// Namely, a buffer or a texture (not the wgpu object itself, but rather the wrappers written in this crate).
pub trait Binding {
    fn get_binding_resource(&self) -> wgpu::BindingResource;
}

pub struct BindingGroup {
    pub bindings: Vec<Box<dyn Binding>>,
    pub layout: Arc<wgpu::BindGroupLayout>,
    pub bind_group: wgpu::BindGroup
}

impl BindingGroup {
    pub fn new(display: &crate::State, layout: Arc<wgpu::BindGroupLayout>, bindings: Vec<Box<dyn Binding>>) -> Self {
        let bind_group = display.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &bindings.iter().enumerate().map(|x|{
                wgpu::BindGroupEntry {
                    binding: x.0 as u32,
                    resource: x.1.get_binding_resource()
                }

            }).collect::<Vec<wgpu::BindGroupEntry>>(),
            label: None,
        });

        Self {
            bindings,
            layout: layout,
            bind_group,
        }
    }
}