use crate::dataunit::*;
use std::rc::Rc;

pub struct TextureArray {
    pub texture: Rc<wgpu::Texture>,
    // sampler: Rc<wgpu::Sampler>,
    // view: Rc<wgpu::TextureView>
}

impl TextureArray {
    pub fn load_hardcoded(
        display: &crate::State,
        block: &DataUnit,
    ) -> Self {

        todo!();
    
        match block.1 {
            UnitFormat::IMAGE(imgfmt) => {
                let image_loaded = image::load_from_memory_with_format(block.0, imgfmt).unwrap();
                let image_rgba = image_loaded.to_rgba8();
                let image_dimensions = wgpu::Extent3d{
                    width: image_rgba.width(),
                    height: image_rgba.height(),
                    depth_or_array_layers: 1,
                };
            
                let texture = display.device.create_texture(&wgpu::TextureDescriptor {
                    label: None,
                    size: image_dimensions,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });
            
                display.queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    }, 
                    &image_rgba, 
                    wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * image_dimensions.width),
                        rows_per_image: Some(image_dimensions.height),
                    }, 
                    image_dimensions
                );
            
                TextureArray {
                    texture: Rc::new(texture)
                }
            },
            _ => {panic!("Trying to load invalid data block type")}
        }
    
    }

    pub fn create_view(&self, state: &crate::State) -> Rc<wgpu::TextureView> {
        Rc::new(self.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(state.config.format.clone()), 
            dimension: Some(wgpu::TextureViewDimension::D2Array), 
            aspect: wgpu::TextureAspect::All, 
            base_mip_level: 0, 
            mip_level_count: None, 
            base_array_layer: 0, 
            array_layer_count: None 
        }))
    }

}