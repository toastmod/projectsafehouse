pub trait Vertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static>;
}