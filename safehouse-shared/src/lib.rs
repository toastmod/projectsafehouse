use vertex::Vertex;

pub mod vertex;

pub fn model_unpacker<V: Vertex>(model: &'static [u8]) {
    let mut header_bytes: [u8; 4] = [0u8; 4];
    header_bytes.copy_from_slice(&model[0..4]);
    let vertex_chunk: u32 = u32::from_be_bytes(header_bytes);
}