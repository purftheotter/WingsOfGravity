pub const MAX_INSTANCES:usize = 100_000;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable,)]
pub struct Instance {
    pub position: [f32; 2],
    pub rotation: f32,
    pub size: [f32; 2],
    pub uv_rect: [f32; 4],
    pub color: [f32; 4],
}

impl Instance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 20,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 36,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                }
            ],
        }
    }
}

pub struct QueuedInstance {
    pub layer: i32,
    pub texture: usize,
    pub instance: Instance,
}
