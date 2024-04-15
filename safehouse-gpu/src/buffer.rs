
use wgpu::util::DeviceExt;

use super::{bindings::Binding, State};

pub trait Buffer {
    fn get_buffer(&self) -> &wgpu::Buffer;
}

pub struct VertexBuffer<T> {
    pub buffer: wgpu::Buffer,
    p: std::marker::PhantomData<T>,
}


impl<T> VertexBuffer<T> {
    pub fn new(display: &State, data: &[T]) -> Self {
        let buffer = display.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe { std::mem::transmute::<&[T], &[u8]>(data) },
            usage: wgpu::BufferUsages::VERTEX,
        });
        VertexBuffer {
            buffer,
            p: std::marker::PhantomData,
        }
    }
}

impl<T> Buffer for VertexBuffer<T> {

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
            contents: unsafe { std::mem::transmute::<&[T], &[u8]>(data) },
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

pub struct Uniform<T> {
    pub buffer: wgpu::Buffer,
    p: std::marker::PhantomData<T>,
}

impl<T> Uniform<T> {
    pub fn new(display: &State, data: &[T]) -> Self {
        let buffer = display.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: unsafe { std::mem::transmute::<&[T], &[u8]>(data) },
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        Uniform {
            buffer,
            p: std::marker::PhantomData,
        }
    }

    pub fn update(&self, display: &State, data: &[T]) {
        display.queue.write_buffer(&self.buffer, 0, unsafe { std::mem::transmute::<&[T], &[u8]>(data) });
    }
}

impl<T> Buffer for Uniform<T> {

    fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

}

impl<T> Binding for Uniform<T> {
    fn get_binding_resource(&self) -> wgpu::BindingResource {
        self.buffer.as_entire_binding()
    }
}

// TODO: if applicable, finish UniformPtr type.
//pub struct UniformPtr<'a, T> {
//    buffer: Uniform<T>,
//    data: &'a mut [T],
//}
//
//impl<'a, T: Sized> UniformPtr<'a, T> {
//    pub fn new(display: &State, data: &'a mut [T]) -> Self {
//
//        UniformPtr {
//            buffer: Uniform::new(display, data),
//            data
//        }
//    }
//}