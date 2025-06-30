use image::GenericImageView;
use wgpu::Extent3d;

use crate::{binding::{Bindable, BindableType}, dataunit::*};
use std::rc::Rc;

pub struct Texture {
    texture: Rc<wgpu::Texture>,
    pub view: Rc<wgpu::TextureView>
}

impl Texture {

    pub fn load_encoded<'image>(
        display: &crate::State,
        data: &'image [u8],
        encoding_format: ImageFormat
    ) -> Texture {

        let image_loaded = image::load_from_memory_with_format(data, encoding_format).unwrap();
        let image_rgba = image_loaded.to_rgba8();
        let (width, height) = image_loaded.dimensions();
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        // Match surface format
        // TODO: Make this adjustable if some efficiency can be involved?
        let texture_format = display.config.format.clone();
            
        let texture = display.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size, 
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format.clone(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[texture_format.clone()],
        });
            
        display.queue.write_texture(
            wgpu::TexelCopyTextureInfoBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            }, 
            &image_rgba, 
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            }, 
            size.physical_size(texture_format.clone())
        );
            
        Texture {
            view: Rc::new(texture.create_view(&wgpu::TextureViewDescriptor {
                label: None,
                format: Some(texture_format.clone()), 
                dimension: Some(wgpu::TextureViewDimension::D2), 
                aspect: wgpu::TextureAspect::All, 
                base_mip_level: 0, 
                mip_level_count: None, 
                base_array_layer: 0, 
                array_layer_count: None,
                usage: None, 
            })),
            texture: Rc::new(texture),
        }
    }

    pub fn load_hardcoded(
        display: &crate::State,
        block: &DataUnit,
    ) -> Self {
    
        match block.1 {
            UnitFormat::IMAGE(imgfmt) => {
                Self::load_encoded(display, block.0, imgfmt)
            },
            _ => {panic!("Trying to load invalid data block type")}
        }
    
    }

    pub fn new_blank_dynamic(display: &crate::State, width: u32, height: u32) -> Self {
        let image_dimensions = wgpu::Extent3d{
            width,
            height,
            depth_or_array_layers: 1,
        };
            
        let texture = display.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: image_dimensions,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: display.config.format.clone(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
            
        Texture {
            // view: Rc::new(texture.create_view(&wgpu::TextureViewDescriptor {
            //     label: None,
            //     format: Some(display.config.format.clone()), 
            //     dimension: Some(wgpu::TextureViewDimension::D2), 
            //     aspect: wgpu::TextureAspect::All, 
            //     base_mip_level: 0, 
            //     mip_level_count: None, 
            //     base_array_layer: 0, 
            //     array_layer_count: None,
            //     usage: None, 
            // })),
            view: Rc::new(texture.create_view(&wgpu::TextureViewDescriptor::default())),
            texture: Rc::new(texture),
        }
    }

}

impl Bindable for Texture {
    fn get_binding_entry(&self, slot: u32) -> wgpu::BindGroupEntry {
        crate::wgpu::BindGroupEntry {
            binding: slot,
            resource: crate::wgpu::BindingResource::TextureView(&self.view),
        }
    }
}

impl BindableType for Texture {
    fn get_layout_entry(slot: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: slot, 
            visibility, 
            ty: wgpu::BindingType::Texture { 
                sample_type: wgpu::TextureSampleType::Float { filterable: false }, 
                view_dimension: wgpu::TextureViewDimension::D2, 
                multisampled: false
            }, 
            count: None 
        }
    }
}