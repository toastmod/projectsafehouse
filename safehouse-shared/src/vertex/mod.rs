mod types;
pub use types::*;
pub trait Vertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static>;
}