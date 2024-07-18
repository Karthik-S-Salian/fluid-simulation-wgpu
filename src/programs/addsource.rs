use bytemuck::{Pod, Zeroable};

use crate::{config::Config, render::StorageBuffer2D, utils::{compile_shader_module, create_auto_layout_compute_pipeline}};

pub struct AddSourceProgram<'a> {
    device: &'a wgpu::Device,
    source: &'a StorageBuffer2D,
    target: &'a StorageBuffer2D,
    uniforms: Uniforms,
    config: &'a Config,
    uniform_buffer: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
    compute_bind_group: wgpu::BindGroup
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Uniforms {
    grid_size: f32,
}

impl<'a> AddSourceProgram<'a> {
    pub fn new(
        device: &'a wgpu::Device,
        source: &'a StorageBuffer2D,
        target: &'a StorageBuffer2D,
        config: &'a Config,
    ) -> AddSourceProgram<'a> {
        let code = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/shaders/addsource.wgsl"
        ));

        let shader_module = compile_shader_module(&device, code);
        let compute_pipeline = create_auto_layout_compute_pipeline(&device, &shader_module);

        let uniforms = Uniforms {
            grid_size: config.grid_size(),
        };

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("add source uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: true,
        });

        uniform_buffer
            .slice(..)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::bytes_of(&uniforms));
        uniform_buffer.unmap();

        let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &compute_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &source.x,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &source.y,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &target.x,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &target.y,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        AddSourceProgram {
            device,
            source,
            target,
            uniforms,
            config,
            uniform_buffer,
            compute_pipeline,
            compute_bind_group,
        }
    }

    fn dispatch<'b>(&'b self, compute_pass: &'b mut wgpu::ComputePass<'b>) {
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
        let workgroup_size = self.config.workgroup_size();
        compute_pass.dispatch_workgroups(workgroup_size.0, workgroup_size.1, workgroup_size.2);
    }
}
