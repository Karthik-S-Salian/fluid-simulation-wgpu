pub fn compile_shader_module(device: &wgpu::Device, code: &str) -> wgpu::ShaderModule {
    use std::borrow::Cow;

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("render"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(code)),
    })
}

pub fn create_auto_layout_compute_pipeline(
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

fn vec_items_mem_size<T>(vector:&Vec<T>)->usize{
    vector.len()*std::mem::size_of::<T>()
}

// pub fn create_2d_storage_buffer(
//     device: &wgpu::Device,
//     size: [u32; 2],
//     element_size: u32,
//     intial_values: Option<[Vec<i32>; 2]>,
// ) -> Vec<wgpu::Buffer> {
//     size.iter()
//         .enumerate()
//         .map(|(index, &len)| {
//             let buffer = device.create_buffer(&wgpu::BufferDescriptor {
//                 label: Some("storage buffer 1"),
//                 size: (len * element_size) as u64,
//                 usage: wgpu::BufferUsages::STORAGE,
//                 mapped_at_creation: intial_values.is_some(),
//             });

//             if let Some(intial_value) = &intial_values {
//                 buffer
//                     .slice(..)
//                     .get_mapped_range_mut()
//                     .copy_from_slice(bytemuck::cast_slice(&intial_value[index]));
//                 buffer.unmap();
//             }

//             buffer
//         })
//         .collect()
// }
