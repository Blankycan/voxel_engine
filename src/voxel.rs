#[derive(Copy, Clone, Debug)]
pub enum VoxelType {
    Default = 0,
    Grass,
    Dirt,
}

#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    active: bool,
    voxel_type: VoxelType,
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            active: true,
            voxel_type: VoxelType::Default,
        }
    }
}
