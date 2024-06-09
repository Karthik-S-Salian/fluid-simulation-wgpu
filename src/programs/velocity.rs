pub struct Velocity{
    viscosity:f32,
    uv_buffer0:[wgpu::Buffer;2],
    uv_buffer1:[wgpu::Buffer;2],
}


impl Velocity{
    fn new()->Velocity{
        
    }
}


fn create_storage_buffers(initial_u:&Vec<i32>,initial_v:&Vec<i32>){
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("storage u"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::STORAGE,
    });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("storage v"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::STORAGE,
    });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("storage u prvs"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::STORAGE,
    });

    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("storage v prvs"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::STORAGE,
    });

    initialize_storage_buffer()
}

fn initialize_storage_buffer(initial_value: Option<&T>, mem_size: u64) -> wgpu::Buffer {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("storage v prvs"),
        size: mem_size,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: initial_value.is_some(),
    });

    buffer
        .slice(..)
        .get_mapped_range_mut()
        .copy_from_slice(bytemuck::bytes_of(&initial_value));
    buffer.unmap();
    buffer
}