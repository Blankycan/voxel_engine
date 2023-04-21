use bevy::prelude::Vec2;

use crate::{face::Side, face::Side::*, voxel::VoxelType, voxel::VoxelType::*};

const TEXTURE_WIDTH: f32 = 512.0;
const TEXTURE_HEIGHT: f32 = 512.0;
const SPRITE_SIZE: f32 = 4.0;
const SPRITE_OFFSET: f32 = 1.0;

pub fn get_voxel_type_uv(voxel_type: VoxelType, side: Side) -> [Vec2; 4] {
    match voxel_type {
        Default => get_uv_for_index(0, 0),
        None => get_uv_for_index(0, 0),
        Dirt => get_uv_for_index(1, 0),
        Grass => match side {
            Top => get_uv_for_index(2, 0),
            Bottom => get_uv_for_index(1, 0),
            _ => get_uv_for_index(3, 0),
        },
    }
}

fn get_uv_for_index(x: usize, y: usize) -> [Vec2; 4] {
    let x0: f32 = x as f32;
    let y0: f32 = y as f32;
    let x1: f32 = (x + 1) as f32;
    let y1: f32 = (y + 1) as f32;
    [
        Vec2::new(
            (x1 * SPRITE_SIZE + (x1 * 2.0 - 1.0) * SPRITE_OFFSET) / TEXTURE_WIDTH,
            (y0 * SPRITE_SIZE + (y0 * 2.0 + 1.0) * SPRITE_OFFSET) / TEXTURE_HEIGHT,
        ),
        Vec2::new(
            (x0 * SPRITE_SIZE + (x0 * 2.0 + 1.0) * SPRITE_OFFSET) / TEXTURE_WIDTH,
            (y0 * SPRITE_SIZE + (y0 * 2.0 + 1.0) * SPRITE_OFFSET) / TEXTURE_HEIGHT,
        ),
        Vec2::new(
            (x0 * SPRITE_SIZE + (x0 * 2.0 + 1.0) * SPRITE_OFFSET) / TEXTURE_WIDTH,
            (y1 * SPRITE_SIZE + (y1 * 2.0 - 1.0) * SPRITE_OFFSET) / TEXTURE_HEIGHT,
        ),
        Vec2::new(
            (x1 * SPRITE_SIZE + (x1 * 2.0 - 1.0) * SPRITE_OFFSET) / TEXTURE_WIDTH,
            (y1 * SPRITE_SIZE + (y1 * 2.0 - 1.0) * SPRITE_OFFSET) / TEXTURE_HEIGHT,
        ),
    ]
}
