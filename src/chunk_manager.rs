use std::collections::VecDeque;

use crate::chunk::CHUNK_SIZE;
use crate::voxel::Voxel;
use crate::{chunk::Chunk, chunk_mesh_builder};
use bevy::prelude::*;
use bevy::prelude::{Commands, Transform};
use bevy::render::primitives::Frustum;
use bevy::utils::hashbrown::HashMap;
use rand::Rng;

pub const MAX_CHUNKS: usize = 10000;
pub const MAX_MESHES: usize = 10000;
pub const MAX_CHUNK_LOAD_LIST: usize = 16;
pub const MAX_MESH_LOAD_LIST: usize = 16;
pub const MAX_LOAD_CHUNKS_PER_FRAME: usize = 2;
pub const MAX_LOAD_MESHES_PER_FRAME: usize = 2;
pub const MAX_CHUNK_UNLOAD_LIST: usize = 16;
pub const MAX_MESH_UNLOAD_LIST: usize = 16;
pub const MAX_UNLOAD_CHUNKS_PER_FRAME: usize = 4;
pub const MAX_UNLOAD_MESHES_PER_FRAME: usize = 4;
pub const MAX_MESHES_TO_RENDER_LIST: usize = 32;
pub const MAX_RENDER_MESHES_PER_FRAME: usize = 1;
pub const DEFAULT_RENDER_DISTANCE: i32 = 6;

#[derive(Debug)]
pub enum ChunkError {
    NoChunk,
    NoVoxel,
}

#[derive(Resource)]
pub struct ChunkManager {
    chunks: HashMap<IVec3, Chunk>,
    meshes: HashMap<IVec3, Option<Mesh>>,

    chunk_load_list: VecDeque<IVec3>,
    mesh_load_list: VecDeque<IVec3>,

    chunk_unload_list: VecDeque<IVec3>,
    mesh_unload_list: VecDeque<IVec3>,

    mesh_visible_list: VecDeque<IVec3>,
    rendered_meshes: HashMap<IVec3, Entity>,

    render_distance: i32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        ChunkManager::new()
    }
}

impl ChunkManager {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::with_capacity(MAX_CHUNKS),
            meshes: HashMap::with_capacity(MAX_MESHES),
            chunk_load_list: VecDeque::<IVec3>::with_capacity(MAX_CHUNK_LOAD_LIST),
            mesh_load_list: VecDeque::<IVec3>::with_capacity(MAX_MESH_LOAD_LIST),
            chunk_unload_list: VecDeque::<IVec3>::with_capacity(MAX_CHUNK_UNLOAD_LIST),
            mesh_unload_list: VecDeque::<IVec3>::with_capacity(MAX_MESH_UNLOAD_LIST),
            mesh_visible_list: VecDeque::<IVec3>::with_capacity(MAX_MESHES_TO_RENDER_LIST),
            rendered_meshes: HashMap::with_capacity(MAX_MESHES),
            render_distance: DEFAULT_RENDER_DISTANCE,
        }
    }

    pub fn make_coords_valid(chunk_pos: &mut IVec3, voxel_pos: &mut IVec3) {
        let chunk_size = CHUNK_SIZE as i32;
        // Right
        if voxel_pos.x >= chunk_size {
            voxel_pos.x -= chunk_size;
            chunk_pos.x += 1;
        }
        // Left
        if voxel_pos.x < 0 {
            voxel_pos.x += chunk_size;
            chunk_pos.x -= 1;
        }
        // Top
        if voxel_pos.y >= chunk_size {
            voxel_pos.y -= chunk_size;
            chunk_pos.y += 1;
        }
        // Bottom
        if voxel_pos.y < 0 {
            voxel_pos.y += chunk_size;
            chunk_pos.y -= 1;
        }
        // Front
        if voxel_pos.z >= chunk_size {
            voxel_pos.z -= chunk_size;
            chunk_pos.z += 1;
        }
        // Back
        if voxel_pos.z < 0 {
            voxel_pos.z += chunk_size;
            chunk_pos.z -= 1;
        }
    }

    pub fn get_voxel(&self, chunk_pos: &IVec3, voxel_pos: &IVec3) -> Result<&Voxel, ChunkError> {
        // println!(
        //     "Manager - get_voxel chunk {}, voxel_pos {}",
        //     chunk_pos, voxel_pos
        // );
        let mut new_chunk_pos = *chunk_pos;
        let mut new_voxel_pos = *voxel_pos;
        ChunkManager::make_coords_valid(&mut new_chunk_pos, &mut new_voxel_pos);
        // println!(
        //     "Valid coords are chunk {}, voxel_pos {}",
        //     chunk_pos, voxel_pos
        // );

        if let Some(chunk) = self.chunks.get(&new_chunk_pos) {
            // println!("Got the chunk {}", chunk_pos);
            if let Some(voxel) = chunk.get_voxel(Chunk::get_index(&new_voxel_pos)) {
                return Ok(voxel);
            }
            return Err(ChunkError::NoVoxel);
        }
        Err(ChunkError::NoChunk)
    }

    pub fn get_adjacent_voxels(
        &self,
        chunk_pos: &IVec3,
        voxel_pos: &IVec3,
    ) -> Result<(&Voxel, &Voxel, &Voxel, &Voxel, &Voxel, &Voxel), ChunkError> {
        let (x, y, z) = (voxel_pos.x, voxel_pos.y, voxel_pos.z);
        let right = self.get_voxel(chunk_pos, &IVec3::new(x + 1, y, z))?;
        let left = self.get_voxel(chunk_pos, &IVec3::new(x - 1, y, z))?;
        let top = self.get_voxel(chunk_pos, &IVec3::new(x, y + 1, z))?;
        let bottom = self.get_voxel(chunk_pos, &IVec3::new(x, y - 1, z))?;
        let front = self.get_voxel(chunk_pos, &IVec3::new(x, y, z + 1))?;
        let back = self.get_voxel(chunk_pos, &IVec3::new(x, y, z - 1))?;
        Ok((right, left, top, bottom, front, back))
    }

    pub fn load_chunks(&mut self) {
        let mut chunks_loaded = 0;
        while let Some(chunk_pos) = self.chunk_load_list.pop_front() {
            if self.chunks.len() >= MAX_CHUNKS {
                break;
            }

            let chunk: Chunk = Chunk::new_perlin(chunk_pos, 1337);
            /*
            if chunk_pos.y == 6 {
                chunk = Chunk::new_sphere(
                    rand::thread_rng().gen_range(1.0..(((CHUNK_SIZE / 2) + 1) as f32)) as usize,
                );
            } else {
                let density = match chunk_pos.y {
                    0 => 0.99,
                    3 | 5 => 0.0002,
                    4 => 0.002,
                    _ => 0.0,
                };

                chunk = Chunk::new_random(density);
            }
            */
            self.chunks.insert(chunk_pos, chunk);
            // println!(
            //     " + Chunk {} loaded, empty: {} (Total: {})",
            //     chunk_pos,
            //     chunk.empty,
            //     self.chunks.len()
            // );

            chunks_loaded += 1;
            if chunks_loaded >= MAX_LOAD_CHUNKS_PER_FRAME {
                break;
            }
        }
    }

    pub fn load_meshes(&mut self) {
        let mut meshes_loaded = 0;
        while let Some(chunk_pos) = self.mesh_load_list.pop_front() {
            // Skip if we can't hold more meshes
            if self.meshes.len() >= MAX_MESHES
                || self.mesh_visible_list.len() >= MAX_MESHES_TO_RENDER_LIST
            {
                break;
            }

            // We can only load mesh if we have the chunk
            if let Some(chunk) = self.chunks.get(&chunk_pos) {
                // Empty chunks have no mesh, skip
                if chunk.empty == true {
                    self.meshes.insert(chunk_pos, None);
                    continue;
                }

                let mesh = chunk_mesh_builder::build_mesh(self, chunk, &chunk_pos);
                self.meshes.insert(chunk_pos, Some(mesh));
                self.mesh_visible_list.push_back(chunk_pos);
                // println!(
                //     " + Mesh {} loaded (Total: {})",
                //     chunk_pos,
                //     self.meshes.len()
                // );

                meshes_loaded += 1;
                if meshes_loaded >= MAX_LOAD_MESHES_PER_FRAME {
                    break;
                }
            }
        }
    }

    pub fn unload_chunks(&mut self) {
        let mut chunks_unloaded = 0;
        while let Some(chunk_pos) = self.chunk_unload_list.pop_front() {
            // println!(" - Chunk {} unloaded", chunk_pos);
            self.chunks.remove(&chunk_pos);

            chunks_unloaded += 1;
            if chunks_unloaded >= MAX_UNLOAD_CHUNKS_PER_FRAME {
                break;
            }
        }
    }

    pub fn unload_meshes(&mut self, mut commands: Commands) {
        let mut meshes_unloaded = 0;
        while let Some(chunk_pos) = self.mesh_unload_list.pop_front() {
            // println!(" - Mesh {} unloaded", chunk_pos);
            self.meshes.remove(&chunk_pos);

            if let Some(entity) = self.rendered_meshes.remove(&chunk_pos) {
                // println!(" - Entity removed");
                commands.entity(entity).despawn();
            }

            meshes_unloaded += 1;
            if meshes_unloaded >= MAX_UNLOAD_MESHES_PER_FRAME {
                break;
            }
        }
    }

    pub fn update_visible(&mut self, camera_transform: &Transform, _camera_frustrum: &Frustum) {
        let camera_position = camera_transform.translation;
        let camera_chunk_pos: IVec3 = IVec3::new(
            (camera_position.x / CHUNK_SIZE as f32) as i32,
            (camera_position.y / CHUNK_SIZE as f32) as i32,
            (camera_position.z / CHUNK_SIZE as f32) as i32,
        );

        // Look for Chunks within render distance
        for x in -self.render_distance..(self.render_distance + 1) {
            for y in -self.render_distance..(self.render_distance + 1) {
                for z in -self.render_distance..(self.render_distance + 1) {
                    let chunk_pos = camera_chunk_pos + IVec3::new(x, y, z);

                    // Queue chunk data
                    if !self.chunks.contains_key(&chunk_pos)
                        && !self.chunk_load_list.contains(&chunk_pos)
                        && self.chunk_load_list.len() < MAX_CHUNK_LOAD_LIST
                    {
                        //println!("Queue chunk {} for loading..", chunk_pos);
                        self.chunk_load_list.push_back(chunk_pos);
                    }

                    if !self.meshes.contains_key(&chunk_pos)
                        && !self.mesh_load_list.contains(&chunk_pos)
                        && self.mesh_load_list.len() < MAX_MESH_LOAD_LIST
                    {
                        // Queue mesh if all adjacent chunk's data are loaded
                        let missing_neighbour_data = [
                            IVec3::X,
                            IVec3::NEG_X,
                            IVec3::Y,
                            IVec3::NEG_Y,
                            IVec3::Z,
                            IVec3::NEG_Z,
                        ]
                        .iter_mut()
                        .map(|v| *v + chunk_pos)
                        .any(|v| !self.chunks.contains_key(&v));

                        // Queue mesh
                        if !missing_neighbour_data {
                            // println!("Queue mesh {} for loading..", chunk_pos);
                            self.mesh_load_list.push_back(chunk_pos);
                        }
                    }
                }
            }
        }

        // Unload chunks outside of render distance
        let min_pos = camera_chunk_pos
            - IVec3::new(
                self.render_distance,
                self.render_distance,
                self.render_distance,
            );
        let max_pos = camera_chunk_pos
            + IVec3::new(
                self.render_distance,
                self.render_distance,
                self.render_distance,
            );

        let chunk_pos_outside: Vec<_> = self
            .chunks
            .iter()
            .filter(|(pos, _)| {
                pos.x < min_pos.x
                    || pos.x > max_pos.x
                    || pos.y < min_pos.y
                    || pos.y > max_pos.y
                    || pos.z < min_pos.z
                    || pos.z > max_pos.z
            })
            .map(|(pos, _)| pos)
            .collect();

        for chunk_pos in chunk_pos_outside {
            if self.chunk_unload_list.len() < MAX_CHUNK_UNLOAD_LIST
                && !self.chunk_unload_list.contains(chunk_pos)
            {
                // println!("Queue chunk {} for unloading..", chunk_pos);
                self.chunk_unload_list.push_back(*chunk_pos);
            }
            if self.mesh_unload_list.len() < MAX_MESH_UNLOAD_LIST
                && !self.mesh_unload_list.contains(chunk_pos)
            {
                self.mesh_unload_list.push_back(*chunk_pos);
            }
        }
    }

    pub fn render(
        &mut self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut rendered_meshes = 0;
        while let Some(chunk_pos) = self.mesh_visible_list.pop_front() {
            if self.rendered_meshes.len() >= MAX_MESHES {
                self.mesh_visible_list.push_back(chunk_pos);
                return;
            }

            if let Some(mesh_option) = self.meshes.get(&chunk_pos) {
                if let Some(mesh) = mesh_option {
                    let chunk_entity = commands
                        .spawn(MaterialMeshBundle {
                            mesh: meshes.add(mesh.clone()),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_xyz(
                                chunk_pos.x as f32 * CHUNK_SIZE as f32,
                                chunk_pos.y as f32 * CHUNK_SIZE as f32,
                                chunk_pos.z as f32 * CHUNK_SIZE as f32,
                            ),
                            ..default()
                        })
                        .id();
                    self.rendered_meshes.insert(chunk_pos, chunk_entity);

                    rendered_meshes += 1;
                }
                if rendered_meshes >= MAX_RENDER_MESHES_PER_FRAME {
                    break;
                }
            }
        }
    }
}
