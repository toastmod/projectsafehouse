pub mod d2;
use crate::{gpu, texture::TextureType};
use std::{ops::Range, rc::Rc};

use gpu::wgpu;
use safehouse_gpu::buffer::VertexBuffer;

pub trait ModelDataRes {}

pub struct ModelData {
    pub vertex_buffer: Rc<VertexBuffer>,
    pub textures: Option<Vec<TextureType>>,
    pub model_bindgroup: Option<(Rc<wgpu::BindGroup>, Rc<wgpu::BindGroupLayout>)>,
    pub groups: Box<[Range<u32>]>
}

impl ModelDataRes for ModelData {}

impl ModelData {

    //// / Creates a basic model with a single group of vertices.
    // pub fn create_model<V: crate::gpu::vertex::Vertex>(state: &crate::gpu::State, using_pipeline_name: Option<&'static str>, vertices: &[V], with_textures: Option<Vec<gpu::texture::Texture>>) -> ModelData {
    //     let bg_entries_heap: Vec<wgpu::BindGroupEntry>;
    //     let bg = match with_textures.as_ref() {
    //         Some(textures) => {
    //             bg_entries_heap = textures.iter().enumerate().map(|(i, texture)|{
    //                 wgpu::BindGroupEntry {
    //                     binding: i as u32,
    //                     resource: wgpu::BindingResource::TextureView(&texture.view),
    //                 }
    //             }).collect();
                
    //             Some(state.init_bindgroup_from_pipeline(
    //                 using_pipeline_name.unwrap_or("default"), 
    //                 crate::BINDGROUP_MODEL, 
    //                 bg_entries_heap.as_slice() 
    //             ).expect("Could not create bind group!"))
    //         },
    //         None => None,
    //     };

    //     ModelData {
    //         vertex_buffer: gpu::buffer::VertexBuffer::new(&state, vertices),
    //         textures: with_textures,
    //         model_bindgroup: (bg),
    //         groups: Box::new([0u32..(vertices.len() as u32)])
    //     }

    // }


}