use crate::binding::{Bindable, BindableType};

pub struct TextureSampler {
    sampler: wgpu::Sampler,
}

impl TextureSampler {
    pub fn new(state: &crate::State, desc: &wgpu::SamplerDescriptor) -> Self {
        Self {
            sampler: state.device.create_sampler(desc)
        }
    }
}

impl Bindable for TextureSampler {
    fn get_binding_entry<'a>(&'a self, slot: u32) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry { binding: slot, resource: wgpu::BindingResource::Sampler(&self.sampler) }
    }
}

impl BindableType for TextureSampler {
    fn get_layout_entry(slot: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry { binding: slot, visibility: wgpu::ShaderStages::all(), ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering), count: None }
    }
}