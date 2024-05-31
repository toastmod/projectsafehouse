use std::{any::Any, borrow::Borrow, num::NonZeroU64, rc::Rc};

use crate::{binding::{Bindable, BindableType}, State};
use slicebytes::cast_bytes;
use wgpu::util::DeviceExt;

pub trait Buffer {
    fn get_buffer(&self) -> &wgpu::Buffer;
}

pub struct VertexBuffer {
    pub buffer: wgpu::Buffer,
    pub desc: &'static wgpu::VertexBufferLayout<'static>,
}


impl VertexBuffer {
    pub fn new<V: crate::vertex::Vertex>(display: &State, data: &[V]) -> Rc<Self> {
        let desc = wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe { cast_bytes::<V>(data) },
            usage: wgpu::BufferUsages::VERTEX,
        };
        let buffer = display.device.create_buffer_init(&desc);
        Rc::new(VertexBuffer {
            buffer,
            desc: V::desc()
        })
    }
    pub fn new_from_raw<V: crate::vertex::Vertex>(display: &State, data: &[u8]) -> Rc<Self> {
        let desc = wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::VERTEX,
        };
        let buffer = display.device.create_buffer_init(&desc);
        Rc::new(VertexBuffer {
            buffer,
            desc: V::desc()
        })
    }
}

impl Buffer for VertexBuffer {

    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

}

pub struct IndexBuffer<T> {
    pub buffer: wgpu::Buffer,
    p: std::marker::PhantomData<T>,
}


impl<T> IndexBuffer<T> {
    pub fn new(display: &State, data: &[T]) -> Self {
        let buffer = display.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe { cast_bytes::<T>(data) },
            usage: wgpu::BufferUsages::INDEX,
        });
        IndexBuffer {
            buffer,
            p: std::marker::PhantomData,
        }
    }
}

impl<T> Buffer for IndexBuffer<T> {

    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

}

#[derive(Debug)]
pub struct Uniform<T> {
    pub buffer: wgpu::Buffer,
    p: std::marker::PhantomData<T>,
}

impl<T> Uniform<T> {
    pub fn new(display: &State, data: &[T]) -> Rc<Self> {
        let buffer = display.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe { cast_bytes::<T>(data) },
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        Rc::new(Uniform {
            buffer,
            p: std::marker::PhantomData,
        })
    }

    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    } 

    pub fn update(&self, display: &State, data: &[T]) {
        display.queue.write_buffer(&self.buffer, 0, unsafe {cast_bytes(data)} );
    }
}

impl<T> Buffer for Uniform<T> {
    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

impl<T> Bindable for Uniform<T> {
    fn get_binding_entry(&self, slot: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding: slot,
            resource: self.get_buffer().as_entire_binding()
        }
    }
}

impl<T> BindableType for Uniform<T> {
    fn get_layout_entry(slot: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: slot,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(NonZeroU64::new(std::mem::size_of::<T>() as u64).unwrap()) 
            },
            count: None
        }
    }
}

#[derive(Debug)]
pub struct UniformPtr<T> {
   buffer: Rc<Uniform<T>>,
   data: Box<[T]>,
}

impl<'a, T: Sized> UniformPtr<T> {
    pub fn new(display: &State, data: T) -> Self {
        let ptr = Box::new([data]);
        UniformPtr {
            buffer: Uniform::new(display, ptr.as_ref()),
            data: ptr
        }
    }

    pub fn update(&self, display: &State) {
        self.buffer.update(display, &self.data)
    }
}

impl<T> Buffer for UniformPtr<T> {

    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer.buffer
    }

}

impl<T> Bindable for UniformPtr<T> {
    fn get_binding_entry(&self, slot: u32) -> wgpu::BindGroupEntry {
        self.buffer.get_binding_entry(slot)
    }
}

impl<T> BindableType for UniformPtr<T> {
    fn get_layout_entry(slot: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry {
        Uniform::<T>::get_layout_entry(slot, visibility)
    }
}

impl<T: Sized> AsRef<T> for UniformPtr<T> {
    fn as_ref(&self) -> &T {
        &self.data[0]
    }
}

impl<T: Sized> AsMut<T> for UniformPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data[0]   
    }
}