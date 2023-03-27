use bevy::{
    prelude::{IVec3, Mesh, Vec3},
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::{
    chunk::{Chunk, CHUNK_SIZE},
    chunk_manager::ChunkManager,
    face::{Face, Side},
};

pub fn build_mesh(chunk_manager: &ChunkManager, chunk: &Chunk, chunk_pos: &IVec3) -> Mesh {
    let mut faces = Vec::<Face>::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let index: usize = Chunk::index_from(x, y, z);

                if let Some(voxel) = chunk.get_voxel(index) {
                    if !voxel.active {
                        continue;
                    }

                    let voxel_pos = IVec3::new(x as i32, y as i32, z as i32);
                    let voxel_pos_local = Vec3::new(x as f32, y as f32, z as f32);

                    /*
                    if voxel.active {
                        faces.push(Face::new(Side::Left, voxel_pos_local));
                        faces.push(Face::new(Side::Bottom, voxel_pos_local));
                        faces.push(Face::new(Side::Back, voxel_pos_local));
                        faces.push(Face::new(Side::Right, voxel_pos_local));
                        faces.push(Face::new(Side::Top, voxel_pos_local));
                        faces.push(Face::new(Side::Front, voxel_pos_local));
                    }
                    */

                    if let Ok((right, left, top, bottom, front, back)) =
                        chunk_manager.get_adjacent_voxels(chunk_pos, &voxel_pos)
                    {
                        if !left.active {
                            faces.push(Face::new(Side::Left, voxel_pos_local, voxel.voxel_type));
                        }
                        if !bottom.active {
                            faces.push(Face::new(Side::Bottom, voxel_pos_local, voxel.voxel_type));
                        }
                        if !back.active {
                            faces.push(Face::new(Side::Back, voxel_pos_local, voxel.voxel_type));
                        }
                        if !right.active {
                            faces.push(Face::new(Side::Right, voxel_pos_local, voxel.voxel_type));
                        }
                        if !top.active {
                            faces.push(Face::new(Side::Top, voxel_pos_local, voxel.voxel_type));
                        }
                        if !front.active {
                            faces.push(Face::new(Side::Front, voxel_pos_local, voxel.voxel_type));
                        }
                    }
                }
            }
        }
    }

    let mut vertices = Vec::<([f32; 3], [f32; 3], [f32; 2])>::new();
    let mut indices = Vec::<u32>::new();
    let mut vert_index = 0;

    for face in faces {
        (0..4).for_each(|index| {
            vertices.push((
                face.vertices[index].into(),
                face.normal.into(),
                face.uv[index].into(),
            ));
        });

        indices.push(vert_index);
        indices.push(vert_index + 1);
        indices.push(vert_index + 2);
        indices.push(vert_index);
        indices.push(vert_index + 2);
        indices.push(vert_index + 3);
        vert_index += 4;
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
