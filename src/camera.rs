use crate::gfx::Gfx;

use wgpu::util::DeviceExt;

pub struct CameraGpu {
    pub camera: Camera2D,
    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl CameraGpu {
    pub fn new(gfx:&Gfx) -> Self {
         let camera = Camera2D {
            center: [0.0, 0.0],
            pixels_per_unit: 64.0,
        };

        let mut uniform = CameraUniform::new();

        uniform.update(&camera, gfx.config.width as f32, gfx.config.height as f32);

        let buffer = gfx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let layout = gfx.device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            }
        );

        let bind_group = gfx.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            }
        );

        Self {
            camera,
            uniform,
            buffer,
            layout,
            bind_group,
        }
    }
    pub fn resize(&mut self, gfx: &Gfx){
        self.uniform.update(
            &self.camera,
            gfx.config.width as f32,
            gfx.config.height as f32
        );

        gfx.queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.uniform]),
        );
    }
}

pub struct Camera2D {
    pub center: [f32; 2],
    pub pixels_per_unit: f32,
}

impl Camera2D {
    pub fn view_proj(&self, width_px: f32, height_px: f32) -> [[f32; 4]; 4] {
        let sx = 2.0 * self.pixels_per_unit / width_px;
        let sy = 2.0 * self.pixels_per_unit / height_px;
        [
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-self.center[0] * sx, -self.center[1] * sy, 0.0, 1.0],
        ]
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {

        Self {
            view_proj: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn update(&mut self, camera: &Camera2D, width_px: f32, height_px: f32) {
        self.view_proj = camera.view_proj(width_px, height_px);
    }
}
