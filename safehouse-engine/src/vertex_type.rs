use safehouse_gpu::wgpu;

#[repr(C)]
#[derive(Debug,Clone,Copy,Default)]
pub struct AdvVertex {
    pub pos: [f32; 3],
    pub texcoord: [f32;3],
    pub group_id: u32,
    pub bone_id: u32,
}

impl crate::gpu::vertex::Vertex for AdvVertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static> {
        &wgpu::VertexBufferLayout {
            array_stride: 0,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<f32>() as u64 * 3u64,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: std::mem::size_of::<f32>() as u64 * 6u64,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: std::mem::size_of::<f32>() as u64 * 7u64,
                    shader_location: 3,
                },
 
            ]  
        }
    }
}

#[repr(C)]
#[derive(Debug,Clone,Copy,Default)]
pub struct ColorVertex {
    pub pos: [f32; 3],
    pub color: [f32;3],
}

impl crate::gpu::vertex::Vertex for ColorVertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static> {
        &wgpu::VertexBufferLayout {
            array_stride: 0,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<f32>() as u64 * 3u64,
                    shader_location: 1,
                },
 
            ]  
        }
    }
}


