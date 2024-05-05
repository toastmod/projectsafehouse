use crate::gpu;
use std::rc::Rc;

pub mod obj;

use gpu::wgpu;
use safehouse_gpu::buffer::VertexBuffer;

pub struct ModelData {
    pub vertex_buffer: Rc<VertexBuffer>,
    pub textures: Option<Vec<gpu::texture::Texture>>,
    pub model_bindgroup: Rc<wgpu::BindGroup>,
}

impl ModelData {


    pub fn create_model<V: crate::gpu::vertex::Vertex>(state: &mut crate::gpu::State, model_name: &str, using_pipeline_name: Option<&'static str>, vertices: &[V], with_textures: Option<Vec<gpu::texture::Texture>>) -> Rc<ModelData> {
        let bg_entries_heap: Vec<wgpu::BindGroupEntry>;
        let bg_entries = match with_textures.as_ref() {
            Some(textures) => {
                bg_entries_heap = textures.iter().enumerate().map(|(i, texture)|{
                    wgpu::BindGroupEntry {
                        binding: i as u32,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    }
                }).collect();
                bg_entries_heap.as_slice()
            },
            None => &[],
        };

        let bg = state.init_bindgroup_from_pipeline(
            using_pipeline_name, 
            crate::MODEL, 
            &bg_entries 
        ).expect("Could not create bind group!");

        Rc::new(ModelData{
            vertex_buffer: gpu::buffer::VertexBuffer::new(&state, vertices),
            textures: with_textures,
            model_bindgroup: bg,
        }) 

    }


}