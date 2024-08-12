use safehouse_gpu::wgpu;

#[repr(C)]
#[derive(Debug,Clone,Copy,Default)]
pub struct AdvVertex {
    pub pos: [f32; 4],
    pub texcoord: [f32;3],
    pub normal: [f32;3],
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
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<f32>() as u64 * 4u64,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<f32>() as u64 * 7u64,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: std::mem::size_of::<f32>() as u64 * 10u64,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: std::mem::size_of::<f32>() as u64 * 11u64,
                    shader_location: 4,
                },
 
            ]  
        }
    }
}

#[repr(C)]
#[derive(Debug,Clone,Copy,Default)]
pub struct ColorVertex {
    pub pos: [f32; 4],
    pub color: [f32;4],
}

impl ColorVertex {
    pub fn new(pos: [f32; 4], color: [f32; 4]) -> Self {
        Self {
            pos,
            color
        }
    }
}

impl crate::gpu::vertex::Vertex for ColorVertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static> {
        &wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColorVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<f32>() as u64 * 4u64,
                    shader_location: 1,
                },
 
            ]  
        }
    }
}

#[repr(C)]
#[derive(Debug,Clone,Copy,Default)]
pub struct TexVertex {
    pub pos: [f32; 4],
    pub tex_coord: [f32;2],
}

impl TexVertex {
    pub fn new(pos: [f32; 4], tex_coord: [f32; 2]) -> Self {
        Self {
            pos,
            tex_coord
        }
    }
}

impl crate::gpu::vertex::Vertex for TexVertex {
    fn desc() -> &'static wgpu::VertexBufferLayout<'static> {
        &wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TexVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<f32>() as u64 * 4u64,
                    shader_location: 1,
                },
 
            ]  
        }
    }
}



