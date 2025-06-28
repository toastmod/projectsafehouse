// Dynamic textures are textures that animate or otherwise change such that they need to be checked for rendering.

use std::rc::Rc;

use safehouse_gpu::binding::{Bindable, BindableType};

pub type DynamicTextureHandle = usize;

pub enum TextureType {
    Dynamic(usize),
    Static(Rc<crate::gpu::texture::Texture>)
} 

enum DynamicTextureState {
    Text(crate::gpu::TextRenderState),
    // Reflective
    // Animated //TODO: will use texture array and a uniform
}

pub struct DynamicTexture {
    pub texture: Rc<crate::gpu::texture::Texture>,
    dynamic_state: DynamicTextureState,
    clear_background: crate::gpu::wgpu::Color
}

impl DynamicTexture {

    pub fn new_text(rm: &crate::RenderManager, clear_background: crate::gpu::wgpu::Color, text: &str) -> Self {
        Self {
            texture: Rc::new(crate::gpu::texture::Texture::new_blank_dynamic(&rm.gpu_state, rm.gpu_state.config.width, rm.gpu_state.config.height)),
            dynamic_state: DynamicTextureState::Text(crate::gpu::TextRenderState::new(&rm.gpu_state, &[], text)),
            clear_background,
        }
    }

    pub fn prepare(&mut self, rm: &crate::RenderManager) {
        match &mut self.dynamic_state {
            DynamicTextureState::Text(textstate) => {
                textstate.prepare(&rm.gpu_state)
            },
        }
    }

    pub fn render_to_pass<'pass>(&'pass self, pass: &mut crate::gpu::wgpu::RenderPass<'pass>) {

        match &self.dynamic_state {
            DynamicTextureState::Text(textstate) => {
                textstate.render(pass);
            },
        }

    }

    pub fn render_self<'encoder>(&self, encoder: &'encoder mut crate::gpu::wgpu::CommandEncoder) {
        let mut pass = encoder.begin_render_pass(&crate::gpu::wgpu::RenderPassDescriptor{
            label: Some("TextPlane RenderPass"),
            color_attachments: &[
                Some(crate::gpu::wgpu::RenderPassColorAttachment {
                    view: &self.texture.view,
                    resolve_target: None,
                    ops: crate::gpu::wgpu::Operations {
                        load: crate::gpu::wgpu::LoadOp::Clear(self.clear_background),
                        store: crate::gpu::wgpu::StoreOp::Store
                    } 
                })
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.render_to_pass(&mut pass);

    }
}

impl Bindable for DynamicTexture {
    fn get_binding_entry(&self, slot: u32) -> safehouse_gpu::wgpu::BindGroupEntry {
        self.texture.get_binding_entry(slot)
    }
}

impl BindableType for DynamicTexture {
    fn get_layout_entry(slot: u32, visibility: safehouse_gpu::wgpu::ShaderStages) -> safehouse_gpu::wgpu::BindGroupLayoutEntry {
        safehouse_gpu::texture::Texture::get_layout_entry(slot, visibility)
    }
}