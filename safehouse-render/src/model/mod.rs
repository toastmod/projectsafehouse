pub mod d2;
use crate::{entity::NamedEntity, gpu, texturetype::TextureType};
use std::{ops::Range, rc::Rc};

use gpu::wgpu;
use safehouse_gpu::{binding::{Bindable, BindableType, Binder}, buffer::VertexBuffer, texture::Texture, wgpu::ShaderStages, State};

pub trait ModelDataRes {}

pub trait ModelResources {
    fn model_bindings() -> Vec<Binder<Self>> where Self: Sized;
}

pub struct ModelBindings {
    pub(crate) res: Rc<dyn ModelResources>,
    pub(crate) bg_layout: wgpu::BindGroupLayout, 
    pub(crate) bindgroup: wgpu::BindGroup, 
}

impl ModelResources for () {
    fn model_bindings() -> Vec<Binder<Self>> where Self: Sized {
        vec![]
    }
}

pub struct ModelData {
    pub vertex_buffer: Rc<VertexBuffer>,
    pub groups: Box<[Range<u32>]>,
    pub(crate) binding: Option<ModelBindings>
}

impl ModelData {
    pub fn new<E: NamedEntity, B: ModelResources + 'static>(state: &State, vertex_buffer: Rc<VertexBuffer>, groups: Vec<Range<u32>>, resources: Option<B>) -> Self {

        let binding = if let Some(mres) = resources {
            let binders = B::model_bindings();
            let layout_entries: Vec<wgpu::BindGroupLayoutEntry> = binders.iter().map(|x| x.get_layout_entry()).collect();

            let bg_layout = state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(E::model_bglayout_name()),
                entries: &layout_entries 
            });

            let bg_entries: Vec<wgpu::BindGroupEntry> = binders.iter().map(|x| x.get_binding_entry(&mres)).collect();
            let bindgroup = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(E::model_bindgroup_name()),
                layout: &bg_layout,
                entries: &bg_entries 
            });

            let res = Rc::new(mres) as Rc<dyn ModelResources>;
            Some(ModelBindings {
                res, 
                bg_layout,
                bindgroup
            })
        } else {
            None
        };


        Self {
            vertex_buffer,
            groups: groups.into_boxed_slice(),
            binding
        }
    } 
}

// impl ModelBindings {
//     pub fn create(mut bindables: Vec<Rc<dyn Bindable>>) -> Self {

//         let binders: Vec<Binder<Vec<Rc<dyn Bindable>>>> = bindables.iter_mut().enumerate().map(|(i, b)|{
//             Binder::<Vec<Rc<dyn Bindable>>>::new(
//                 i as u32, 
//                 ShaderStages::all(), 
//                 &|x| &x[i].get_binding_entry(i)
//             )
//         }).collect();

//         Self {
//             model_bindgroup: todo!(),
//             model_bglayout: todo!(),
//         }
//     }
// }

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