#[derive(Copy, Clone, Debug)]
pub enum VoxelType {
    Default = 0,
    Grass,
    Dirt,
    None,
}

#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    pub active: bool,
    pub voxel_type: VoxelType,
}

impl Default for Voxel {
    fn default() -> Self {
        Self {
            active: true,
            voxel_type: VoxelType::Default,
        }
    }
}

impl Voxel {
    pub fn new(active: bool) -> Self {
        Self {
            active: active,
            voxel_type: VoxelType::Default,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            active: false,
            voxel_type: VoxelType::Default,
        }
    }
}
