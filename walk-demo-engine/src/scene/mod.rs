pub mod walking;

pub trait Scene {
    fn init(&mut self, engine: &mut crate::Engine);
    fn update(&mut self, engine: &mut crate::Engine);
}