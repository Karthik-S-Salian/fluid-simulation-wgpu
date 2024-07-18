use bytemuck::{Pod, Zeroable};

use crate::config;
use crate::utils;

pub struct StorageBuffer2D {
    dim: (u64, u64),
    pub x: wgpu::Buffer,
    pub y: wgpu::Buffer,
}

impl StorageBuffer2D {
    pub fn new(
        device: &wgpu::Device,
        dim: (u64, u64),
        label: Option<&str>,
        initial_state_x: Option<Vec<i32>>,
        initial_state_y: Option<Vec<i32>>,
    ) -> StorageBuffer2D {
        let size = dim.0 * dim.1;
        let x = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: size * 4,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: initial_state_x.is_some(),
        });

        if let Some(state) = initial_state_x {
            x.slice(..)
                .get_mapped_range_mut()
                .copy_from_slice(bytemuck::cast_slice(&state));
            x.unmap();
        }

        let y = device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: size * 4,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: initial_state_y.is_some(),
        });

        if let Some(state) = initial_state_y {
            y.slice(..)
                .get_mapped_range_mut()
                .copy_from_slice(bytemuck::cast_slice(&state));
            y.unmap();
        }

        StorageBuffer2D { dim, x, y }
    }
}

pub struct StorageBuffers {
    source: StorageBuffer2D,
    velocity_prvs: StorageBuffer2D,
    velocity: StorageBuffer2D,
    density_prvs: StorageBuffer2D,
    density: StorageBuffer2D,
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: config::Config,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    display_pipeline: wgpu::RenderPipeline,
    display_bind_group: wgpu::BindGroup,
    storage_buffers: StorageBuffers,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Uniforms {
    grid_size: f32,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, config: config::Config) -> Renderer {
        device.on_uncaptured_error(Box::new(|error| {
            panic!("Aborting due to an error: {}", error);
        }));

        let shader_module = {
            let code = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/shaders/render.wgsl"
            ));

            utils::compile_shader_module(&device, code)
        };

        let (display_pipeline, display_layout) = create_display_pipeline(&device, &shader_module);

        let uniforms = Uniforms {
            grid_size: config.grid_size(),
        };
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: true,
        });
        uniform_buffer
            .slice(..)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::bytes_of(&uniforms));
        uniform_buffer.unmap();

        let display_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &display_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        let dim = (config.grid_size() as u64, config.grid_size() as u64);

        let storage_buffers: StorageBuffers = StorageBuffers {
            source: StorageBuffer2D::new(&device, dim, Some("source buffer"), None, None),
            velocity: StorageBuffer2D::new(&device, dim, Some("velocity buffer"), None, None),
            velocity_prvs: StorageBuffer2D::new(
                &device,
                dim,
                Some("velocity_prvs buffer"),
                None,
                None,
            ),
            density: StorageBuffer2D::new(&device, dim, Some("density buffer"), None, None),
            density_prvs: StorageBuffer2D::new(
                &device,
                dim,
                Some("density_prvs buffer"),
                None,
                None,
            ),
        };

        Renderer {
            device,
            queue,
            config,
            uniforms,
            uniform_buffer,
            display_pipeline,
            display_bind_group,
            storage_buffers,
        }
    }

    pub fn render_frame(&self, target: &wgpu::TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render frame"),
            });

        {
            // let compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            //     label: Some("compute pass"),
            //     timestamp_writes: None,
            // });
        }

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("display pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        render_pass.set_pipeline(&self.display_pipeline);

        render_pass.set_bind_group(0, &self.display_bind_group, &[]);

        render_pass.draw(
            0..6,
            0..(&self.config.grid_size() * &self.config.grid_size()) as u32,
        );

        drop(render_pass);

        let commmand_buffer = encoder.finish();
        self.queue.submit(Some(commmand_buffer));
    }
}

fn create_display_pipeline(
    device: &wgpu::Device,
    shader_module: &wgpu::ShaderModule,
) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout) {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("display"),
        layout: Some(
            &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
                ..Default::default()
            }),
        ),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
        vertex: wgpu::VertexState {
            module: shader_module,
            entry_point: "display_vs",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
            entry_point: "display_fs",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    (pipeline, bind_group_layout)
}

fn create_buffers() {}
