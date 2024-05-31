use std::rc::Rc;

use crate::gpu;

// TODO: Decouple Vertex and Fragment shaders

pub struct FragmentShader {
    shader_program: Rc<gpu::shaderprogram::Program>
    // shader_bindgroup: Rc<wgpu::BindGroup>,
}