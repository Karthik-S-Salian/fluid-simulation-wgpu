#[derive(Debug)]
pub struct Config {
    grid_size:f32,
    workgroup_size:(u32,u32,u32)

}

impl Default for Config {
    fn default() -> Self {
        Self {
            grid_size:4.0,
            workgroup_size:(8,8,1)
        }
    }
}

impl Config{
    pub fn buffer_size(&self)->usize{
        ((self.grid_size+2.)*(self.grid_size+2.)) as usize
    }

    pub fn new(grid_size:u32)->Config{
        Config{
            grid_size:grid_size as f32,
            ..Default::default()
        }
    }

    pub fn grid_size(&self)->f32{
        self.grid_size
    }

    pub fn workgroup_size(&self)->(u32,u32,u32){
        self.workgroup_size
    }
}