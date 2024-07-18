struct Uniforms {
  grid_size:vec2f,
  dt:f32
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage> source_x: array<f32>;
@group(0) @binding(2) var<storage> source_y: array<f32>;
@group(0) @binding(3) var<storage,read_write> target_x: array<f32>;
@group(0) @binding(4) var<storage,read_write> target_y: array<f32>;

fn arrayIndex(cell: vec2u) -> u32 {
return (cell.y % u32(uniforms.grid_size.y)) * u32(uniforms.grid_size.y) + (cell.x % u32(uniforms.grid_size.x));  //to handle edge overflow cases
}

@compute
@workgroup_size(8,8)
fn main(@builtin(global_invocation_id) cell: vec3u) {
    let i = arrayIndex(cell.xy);
    target_x[i] += uniforms.dt*source_x[i];
    target_y[i] += uniforms.dt*source_y[i];
}