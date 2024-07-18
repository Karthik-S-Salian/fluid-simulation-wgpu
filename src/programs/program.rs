pub trait Program {
    fn dispatch(&self, compute_pass:  &mut wgpu::ComputePass);
}

