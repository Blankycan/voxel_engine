use bevy::prelude::{Vec2, Vec3};

use crate::{voxel::VoxelType, voxel_textures::*};

pub const HALF_SIZE: f32 = 0.5;
pub const UVS: [Vec2; 4] = [
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 1.0),
];

#[derive(Copy, Clone)]
pub enum Side {
    Right,
    Left,
    Top,
    Bottom,
    Front,
    Back,
}

pub fn get_normal(side: Side) -> Vec3 {
    match side {
        Side::Right => Vec3::X,
        Side::Left => Vec3::NEG_X,
        Side::Top => Vec3::Y,
        Side::Bottom => Vec3::NEG_Y,
        Side::Front => Vec3::Z,
        Side::Back => Vec3::NEG_Z,
    }
}

#[derive(Copy, Clone)]
pub struct Face {
    pub uv: [Vec2; 4],
    pub normal: Vec3,
    pub vertices: [Vec3; 4],
    pub side: Side,
}

impl Face {
    pub fn new(side: Side, pos: Vec3, voxel_type: VoxelType) -> Self {
        let vertices = match side {
            Side::Left => [
                Vec3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
            ],
            Side::Right => [
                Vec3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Side::Top => [
                Vec3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
            ],
            Side::Bottom => [
                Vec3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
            ],
            Side::Back => [
                Vec3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Side::Front => [
                Vec3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vec3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
            ],
        };

        Self {
            uv: get_voxel_type_uv(voxel_type, side),
            normal: get_normal(side),
            vertices: vertices,
            side: side,
        }
    }
}
