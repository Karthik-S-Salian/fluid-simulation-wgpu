use bytemuck::{Pod, Zeroable};

use crate::config::Config;
pub struct AddSourceProgram {
    device: wgpu::Device,
    source: wgpu::Buffer,
    target: wgpu::Buffer,
    uniforms: Uniforms,
    config: Config,
    uniform_buffer: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
    compute_bind_group: wgpu::BindGroup,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Uniforms {
    grid_size: f32,
}

impl AddSourceProgram {
    pub fn new(
        device: wgpu::Device,
        source: wgpu::Buffer,
        target: wgpu::Buffer,
        code: &str,
        config: Config,
    ) -> AddSourceProgram {
        let shader_module = compile_shader_module(&device, code);
        let compute_pipeline = create_compute_pipeline(&device, &shader_module);

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
                        buffer: &source,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &target,
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

    fn dispatch<'a>(&'a self, compute_pass: &'a mut wgpu::ComputePass<'a>) {
        compute_pass.set_pipeline(&self.compute_pipeline);
        compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);
        let workgroup_size = self.config.workgroup_size();
        compute_pass.dispatch_workgroups(workgroup_size.0, workgroup_size.1, workgroup_size.2);
    }
}

fn compile_shader_module(device: &wgpu::Device, code: &str) -> wgpu::ShaderModule {
    use std::borrow::Cow;

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("render"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
    })
}

fn create_compute_pipeline(
    device: &wgpu::Device,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("add source"),
        layout: None,
        module: shader_module,
        entry_point: "main",
    })
}
