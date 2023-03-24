use bevy::prelude::IVec3;

use super::voxel::Voxel;

pub const CHUNK_SIZE: usize = 16;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref BIT_SIZE: i32 = (CHUNK_SIZE as f32).log2() as i32;
}

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    pub voxels: [Voxel; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn get_index(coordinate: IVec3) -> usize {
        (coordinate.z | (coordinate.y << *BIT_SIZE) | (coordinate.x << (*BIT_SIZE * 2))) as usize
    }
    pub fn index_from(x: i32, y: i32, z: i32) -> usize {
        (z | (y << *BIT_SIZE) | (x << (*BIT_SIZE * 2))) as usize
    }

    pub fn new() -> Self {
        Self {
            voxels: [Voxel::default(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}
