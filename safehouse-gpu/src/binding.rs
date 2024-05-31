use std::marker::PhantomData;

pub trait Bindable {
    fn get_binding_entry(&self, slot: u32) -> wgpu::BindGroupEntry; 
}

pub trait BindableType {
    fn get_layout_entry(slot: u32, visibility: wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry;
}

// A proxy through which a struct's contents can be mapped to a GPU bindgroup entry.
pub struct Binder<T> {
    binding: u32,
    visibility: wgpu::ShaderStages,
    member_binding: Box<dyn Fn(&T) -> &dyn Bindable>,
    binding_layout: &'static dyn Fn(u32, wgpu::ShaderStages) -> wgpu::BindGroupLayoutEntry,
    _marker: PhantomData<T>
}

impl<T> Binder<T> {
    pub fn new<B: Bindable + BindableType>(binding: u32, visibility: wgpu::ShaderStages, member_binding: &'static dyn Fn(&T) -> &B) -> Self {
        Self {
            binding,
            visibility,
            member_binding: Box::new(move |x| member_binding(x)),
            binding_layout: &B::get_layout_entry,
            _marker: PhantomData,
        }
    }

    pub fn get_layout_entry(&self) -> wgpu::BindGroupLayoutEntry {
        (self.binding_layout)(self.binding, self.visibility)
    }

    pub fn get_binding_entry<'a>(&'a self, object: &'a T) -> wgpu::BindGroupEntry {
        let a = (self.member_binding)(object);
        a.get_binding_entry(self.binding)
    }
}