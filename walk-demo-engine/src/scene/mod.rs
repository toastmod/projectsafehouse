use safehouse_render::{gpu::winit, scene::SceneObjectHandle};

pub mod walking;

pub enum SceneEvent {

    /// Continue the scene normally
    Continue,

    /// Exit the current scene, effectively ending the process.
    Exit,

    /// Mutate the current scene, allowing moving existing handles and data to a new structure.
    MutScene(fn(&mut Box<dyn Scene>, &mut crate::Engine)),

}

pub trait SceneInit {
    fn init(engine: &mut crate::Engine) -> Self;
}

pub trait Scene {
    fn update(&mut self, engine: &mut crate::Engine) -> SceneEvent;
}
