#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    position: [f32; 3],
    colour: [f32; 3],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        colour: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        colour: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        colour: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        colour: [1.0, 1.0, 1.0],
    },
];

pub const INDICES: &[u16] = &[1, 0, 2, 2, 0, 3];

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // colour
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
