#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32;3],
    tex_coord: [f32;2]
}

pub const VERTICES: [Vertex;4] = [
    Vertex {
        position: [-1.0,1.0, 0.0],
        tex_coord: [-0.5,0.5],
    },
    Vertex {
        position: [-1.0,-1.0, 0.0],
        tex_coord: [-0.5,-0.5],
    },
    Vertex {
        position: [1.0,-1.0, 0.0],
        tex_coord: [0.5,-0.5],
    },
    Vertex {
        position: [1.0,1.0, 0.0],
        tex_coord: [0.5,0.5],
    },
];

pub const INDICES: [u16; 6] = [0,1,2,0,2,3];
