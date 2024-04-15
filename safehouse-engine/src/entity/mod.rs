pub trait Entity {
    fn spawn(state: &mut safehouse_gpu::State);
}