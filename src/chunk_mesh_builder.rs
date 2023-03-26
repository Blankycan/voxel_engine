use bevy::{
    prelude::{IVec3, Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::chunk::{Chunk, CHUNK_SIZE};

pub fn build_mesh(chunk: &Chunk, chunk_pos: &IVec3) -> Mesh {
    const HALF_SIZE: f32 = 0.45;

    let mut vertices = Vec::<([f32; 3], [f32; 3], [f32; 2])>::new();
    let mut indices = Vec::<u32>::new();
    let mut vert_index = 0;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let index: usize = Chunk::index_from(x, y, z);

                if let Some(voxel) = chunk.get_voxel(index) {
                    if !voxel.active {
                        continue;
                    }
                    let pos = Vec3::new(x as f32, y as f32, z as f32);

                    let cube_vertices = [
                        // Front
                        (
                            [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, 0.0, 1.0],
                            [1.0, 0.0],
                        ), // FTR
                        (
                            [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0],
                        ), // FTL
                        (
                            [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, 0.0, 1.0],
                            [0.0, 1.0],
                        ), // FBL
                        (
                            [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, 0.0, 1.0],
                            [1.0, 1.0],
                        ), // FBR
                        // Right
                        (
                            [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                            [1.0, 0.0, 0.0],
                            [1.0, 0.0],
                        ), // BTR
                        (
                            [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                            [1.0, 0.0, 0.0],
                            [0.0, 0.0],
                        ), // FTR
                        (
                            [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                            [1.0, 0.0, 0.0],
                            [0.0, 1.0],
                        ), // FBR
                        (
                            [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                            [1.0, 0.0, 0.0],
                            [1.0, 1.0],
                        ), // BBR
                        // Back
                        (
                            [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, 0.0, -1.0],
                            [1.0, 0.0],
                        ), // BTL
                        (
                            [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, 0.0, -1.0],
                            [0.0, 0.0],
                        ), // BTR
                        (
                            [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, 0.0, -1.0],
                            [0.0, 1.0],
                        ), // BBR
                        (
                            [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, 0.0, -1.0],
                            [1.0, 1.0],
                        ), // BBL
                        // Left
                        (
                            [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                            [-1.0, 0.0, 0.0],
                            [1.0, 0.0],
                        ), // FTL
                        (
                            [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                            [-1.0, 0.0, 0.0],
                            [0.0, 0.0],
                        ), // BTL
                        (
                            [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                            [-1.0, 0.0, 0.0],
                            [0.0, 1.0],
                        ), // BBL
                        (
                            [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                            [-1.0, 0.0, 0.0],
                            [1.0, 1.0],
                        ), // FBL
                        // Top
                        (
                            [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, 1.0, 0.0],
                            [1.0, 0.0],
                        ), // BTR
                        (
                            [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, 1.0, 0.0],
                            [0.0, 0.0],
                        ), // BTL
                        (
                            [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, 1.0, 0.0],
                            [0.0, 1.0],
                        ), // FTL
                        (
                            [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, 1.0, 0.0],
                            [1.0, 1.0],
                        ), // FTR
                        // Bottom
                        (
                            [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, -1.0, 0.0],
                            [1.0, 0.0],
                        ), // BBL
                        (
                            [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                            [0.0, -1.0, 0.0],
                            [0.0, 0.0],
                        ), // BBR
                        (
                            [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, -1.0, 0.0],
                            [0.0, 1.0],
                        ), // FBR
                        (
                            [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                            [0.0, -1.0, 0.0],
                            [1.0, 1.0],
                        ), // FBL
                    ];
                    vertices.extend(cube_vertices.iter().cloned());

                    for _ in 0..6 {
                        indices.push(vert_index);
                        indices.push(vert_index + 1);
                        indices.push(vert_index + 2);
                        indices.push(vert_index);
                        indices.push(vert_index + 2);
                        indices.push(vert_index + 3);
                        vert_index += 4;
                    }
                }
            }
        }
    }

    let mesh_indices = Indices::U32(indices);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(mesh_indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
