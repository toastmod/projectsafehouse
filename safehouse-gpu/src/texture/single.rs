use crate::{buffer::Bindable, dataunit::*};
// use glium::glutin::surface::WindowSurface;
// use glium::{Display, Program};
use std::rc::Rc;

// pub fn load_texture<'a>(display: &'a Display<WindowSurface>, path: &str) -> glium::texture::Texture2d {
//     //loads texture sampler2d data
//     let image = image::open(path).unwrap().to_rgba8();
//     let image_dimensions = image.dimensions();
//     let image =
//         glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

//     //spit out texture obj
//     let texture = glium::texture::Texture2d::new(display, image).unwrap();
//     texture
// }

pub struct Texture {
    texture: Rc<wgpu::Texture>,
    // sampler: Rc<wgpu::Sampler>,
    pub view: Rc<wgpu::TextureView>
}

impl Bindable for Texture {
    fn get_layout_entry(slot: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: slot,
            visibility,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false 
            },
            count: None 
        }
    }

    fn get_binding_entry(&self, slot: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry { binding: slot, resource: wgpu::BindingResource::TextureView(&self.view) }
    }
}

impl Texture {
    pub fn load_hardcoded(
        display: &crate::State,
        block: &DataUnit,
    ) -> Self {
    
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
            
                Texture {
                    view: Rc::new(texture.create_view(&wgpu::TextureViewDescriptor {
                        label: None,
                        format: Some(display.config.format.clone()), 
                        dimension: Some(wgpu::TextureViewDimension::D2), 
                        aspect: wgpu::TextureAspect::All, 
                        base_mip_level: 0, 
                        mip_level_count: None, 
                        base_array_layer: 0, 
                        array_layer_count: None 
                    })),
                    texture: Rc::new(texture),
                }
            },
            _ => {panic!("Trying to load invalid data block type")}
        }
    
    }

}