use bevy::prelude::IVec3;
use rand::prelude::*;

use crate::{chunk_manager::ChunkManager, face::Side, voxel::VoxelType};

use super::voxel::Voxel;

pub const CHUNK_SIZE: usize = 16;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref BIT_SIZE: i32 = (CHUNK_SIZE as f32).log2() as i32;
}

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    pub voxels: [Voxel; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
    pub empty: bool,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: [Voxel::default(); CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
            empty: true,
        }
    }

    pub fn setup_random(&mut self, density: f32) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = Chunk::index_from(x, y, z);
                    if let Some(voxel) = self.voxels.get_mut(index) {
                        let active = thread_rng().gen_range(0.0..1.0) < density;
                        voxel.active = active;
                        voxel.voxel_type = if active {
                            VoxelType::Default
                        } else {
                            VoxelType::None
                        };
                    }
                }
            }
        }
        self.check_empty();
    }

    pub fn setup_sphere(&mut self, radius: usize) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = Chunk::index_from(x, y, z);
                    if let Some(voxel) = self.voxels.get_mut(index) {
                        let (f_x, f_y, f_z) = (x as f32, y as f32, z as f32);
                        let f_radius = radius as f32;
                        if f32::sqrt(
                            (f_x - f_radius) * (f_x - f_radius)
                                + (f_y - f_radius) * (f_y - f_radius)
                                + (f_z - f_radius) * (f_z - f_radius),
                        ) <= f_radius
                        {
                            voxel.active = true;
                        } else {
                            voxel.active = false;
                        }
                    }
                }
            }
        }
        self.check_empty();
    }

    pub fn setup_perlin(&mut self, chunk_pos: IVec3, seed: u32) {
        use noise::{NoiseFn, Perlin};

        let perlin = Perlin::new(seed);
        for (index, voxel) in self.voxels.iter_mut().enumerate() {
            let coord = Self::get_coordinate(index);
            let (chunk_x, chunk_y, chunk_z) = (
                chunk_pos.x as f64 * CHUNK_SIZE as f64,
                chunk_pos.y as f64 * CHUNK_SIZE as f64,
                chunk_pos.z as f64 * CHUNK_SIZE as f64,
            );

            let down_scale = 0.027f64;
            let x = (chunk_x + coord.x as f64) * down_scale;
            let y = (chunk_y + coord.y as f64) * down_scale;
            let z = (chunk_z + coord.z as f64) * down_scale;
            let density = perlin.get([x, y, z]);
            if density > 0.3f64 {
                voxel.active = true;
                voxel.voxel_type = VoxelType::Grass;
            } else {
                voxel.active = false;
                voxel.voxel_type = VoxelType::None;
            }
        }

        self.check_empty();
    }

    pub fn get_index(coordinate: &IVec3) -> usize {
        (coordinate.z | (coordinate.y << *BIT_SIZE) | (coordinate.x << (*BIT_SIZE * 2))) as usize
    }
    pub fn index_from(x: usize, y: usize, z: usize) -> usize {
        (z | (y << *BIT_SIZE) | (x << (*BIT_SIZE * 2))) as usize
    }

    pub fn get_coordinate(index: usize) -> IVec3 {
        IVec3::new(
            (index as f32 / (CHUNK_SIZE * CHUNK_SIZE) as f32) as i32,
            ((index as f32 / CHUNK_SIZE as f32) % CHUNK_SIZE as f32) as i32,
            (index as f32 % CHUNK_SIZE as f32) as i32,
        )
    }

    pub fn get_voxel(&self, index: usize) -> Option<&Voxel> {
        self.voxels.get(index)
    }
    pub fn get_mut_voxel(&mut self, index: usize) -> Option<&mut Voxel> {
        self.voxels.get_mut(index)
    }

    pub fn check_empty(&mut self) -> bool {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = Chunk::index_from(x, y, z);
                    if let Some(voxel) = self.voxels.get(index) {
                        if voxel.active {
                            self.empty = false;
                            return false;
                        }
                    }
                }
            }
        }
        self.empty = true;
        true
    }

    pub fn update_voxel_types(&mut self, chunk_manager: &ChunkManager, chunk_pos: &IVec3) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = Chunk::index_from(x, y, z);
                    if let Some(voxel) = self.voxels.get_mut(index) {
                        if voxel.active {
                            let voxel_pos = IVec3::new(x as i32, y as i32, z as i32);
                            voxel.voxel_type = match voxel.voxel_type {
                                // If Grass and has a block on top, make into Dirt
                                VoxelType::Grass => {
                                    if let Ok(top_voxel) = chunk_manager.get_adjacent_voxel(
                                        Side::Top,
                                        chunk_pos,
                                        &voxel_pos,
                                    ) {
                                        if top_voxel.active {
                                            VoxelType::Dirt
                                        } else {
                                            VoxelType::Grass
                                        }
                                    } else {
                                        VoxelType::Grass
                                    }
                                }
                                _ => voxel.voxel_type,
                            }
                        }
                    }
                }
            }
        }
    }
}
