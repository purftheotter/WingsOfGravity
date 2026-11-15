use crate::{
    camera::CameraGpu, gfx::*, instance::*, texture::{self, GpuTexture, TextureId}, vertex::*,
};

use wgpu::util::DeviceExt;

pub struct SpriteRenderer {
    pipeline: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    texture_layout: wgpu::BindGroupLayout,
    textures: Vec<GpuTexture>,
    runs: Vec<(usize, std::ops::Range<u32>)>,
    quad_vertices: wgpu::Buffer,
    quad_indices: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    instances: Vec<Instance>,
    queue: Vec<QueuedInstance>,
}

impl SpriteRenderer {
    pub fn new(gfx: &Gfx, camera: &CameraGpu) -> Self {
        let texture_layout = 
            gfx.device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor{
                    entries: &[
                        wgpu::BindGroupLayoutEntry{
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension:
                                    wgpu::TextureViewDimension::D2,
                                sample_type:
                                    wgpu::TextureSampleType::Float {
                                        filterable: true },
                            },
                            count: None
                        },
                        wgpu::BindGroupLayoutEntry{
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(
                                wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                }
            );

        let sampler = gfx.device.create_sampler(
            &wgpu::wgt::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::MipmapFilterMode::Nearest,
                ..Default::default()
            }
        );

        let queue = vec![];

        let instances = vec![];

        let instance_buffer = gfx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (MAX_INSTANCES * std::mem::size_of::<Instance>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let quad_vertices = gfx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let quad_indices = gfx.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = INDICES.len() as u32;

        let shader = gfx.device.create_shader_module(
            wgpu::include_wgsl!("shader.wgsl")
        );

        let pipeline_layout =
            gfx.device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        Some(&texture_layout),
                        Some(&camera.layout),
                    ],
                    immediate_size: 0,
                }
            );

        let pipeline = gfx.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Some(Vertex::desc()), Some(Instance::desc())],
                    compilation_options: 
                        wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gfx.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview_mask: None,
                cache: None,
            }
        );

        Self {
            pipeline,
            sampler,
            texture_layout,
            textures: Vec::new(),
            runs: Vec::new(),
            quad_vertices,
            quad_indices,
            num_indices,
            instance_buffer, 
            instances,
            queue,
        }
    }

    pub fn load_texture(
        &mut self,
        gfx: &Gfx,
        bytes: &[u8],
        label: &str,
    ) -> anyhow::Result<TextureId> {
        let texture = texture::Texture::from_bytes(&gfx.device, &gfx.queue, bytes, label)?;
        let bind_group = gfx.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.texture_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
                label: Some(label),
            }
        );

        let texture_size = texture.texture.size();
        let size = [texture_size.width as f32, texture_size.height as f32];

        self.textures.push(GpuTexture { texture, bind_group, size});
        Ok(TextureId { index: self.textures.len() - 1 })
    }

    pub fn texture_size(&self, id: TextureId) -> [f32; 2] {
        self.textures[id.index].size
    }

    pub fn queue_instance(&mut self, q: QueuedInstance) {
        self.queue.push(q);
    }

    pub fn prepare(&mut self, gfx: &Gfx) {
        self.queue.sort_by_key(|q| (q.layer, q.texture));
        self.queue.truncate(MAX_INSTANCES);

        self.instances.clear();
        self.runs.clear();

        for (i, q) in self.queue.iter().enumerate() {
            self.instances.push(q.instance);

            let end = i as u32 + 1;
            let extend = matches!(self.runs.last(), Some((tex, _)) if *tex == q.texture);
            if extend {
                self.runs.last_mut().unwrap().1.end = end;
            }else {
                self.runs.push((q.texture, i as u32..end));
            }
        }
        self.queue.clear();

        if !self.instances.is_empty() {
            gfx.queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.instances),
            );
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, camera: &CameraGpu) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(1, &camera.bind_group, &[]);
        pass.set_vertex_buffer(0, self.quad_vertices.slice(..));
        pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        pass.set_index_buffer(self.quad_indices.slice(..), wgpu::IndexFormat::Uint16);

        for (tex, range) in &self.runs {
            pass.set_bind_group(0, &self.textures[*tex].bind_group, &[]);
            pass.draw_indexed(0..self.num_indices, 0, range.clone());
        }
    }
}
