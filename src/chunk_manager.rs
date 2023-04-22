use std::collections::VecDeque;

use crate::chunk::*;
use crate::face::Side;
use crate::voxel::{Voxel, VoxelType};
use crate::{chunk::Chunk, chunk_mesh_builder};
use bevy::asset::HandleId;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::prelude::{Commands, Transform};
use bevy::render::primitives::Frustum;
use bevy::utils::hashbrown::HashMap;
use bevy::utils::Uuid;
use bevy_mod_picking::PickableBundle;
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape};

pub const MAX_CHUNKS: usize = 10000;
pub const MAX_MESHES: usize = 10000;
pub const MAX_CHUNK_LOAD_LIST: usize = 16;
pub const MAX_MESH_LOAD_LIST: usize = 16;
pub const MAX_CHUNK_REBUILD_LIST: usize = 16;
pub const MAX_LOAD_CHUNKS_PER_FRAME: usize = 4;
pub const MAX_LOAD_MESHES_PER_FRAME: usize = 4;
pub const MAX_REBUILD_CHUNKS_PER_FRAME: usize = 8;
pub const MAX_CHUNK_UNLOAD_LIST: usize = 16;
pub const MAX_MESH_UNLOAD_LIST: usize = 16;
pub const MAX_UNLOAD_CHUNKS_PER_FRAME: usize = 8;
pub const MAX_UNLOAD_MESHES_PER_FRAME: usize = 8;
pub const MAX_MESHES_TO_RENDER_LIST: usize = 32;
pub const MAX_RENDER_MESHES_PER_FRAME: usize = 4;
pub const DEFAULT_RENDER_DISTANCE: i32 = 8;

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
    chunk_rebuild_list: VecDeque<IVec3>,
    chunk_unload_list: VecDeque<IVec3>,

    mesh_load_list: VecDeque<IVec3>,
    mesh_unload_list: VecDeque<IVec3>,

    mesh_render_list: VecDeque<IVec3>,
    rendered_meshes: HashMap<IVec3, Entity>,

    render_distance: i32,

    pub spritesheet_handle: Handle<Image>,
    pub material_handle: Handle<StandardMaterial>,
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
            chunk_rebuild_list: VecDeque::<IVec3>::with_capacity(MAX_CHUNK_REBUILD_LIST),
            chunk_unload_list: VecDeque::<IVec3>::with_capacity(MAX_CHUNK_UNLOAD_LIST),
            mesh_load_list: VecDeque::<IVec3>::with_capacity(MAX_MESH_LOAD_LIST),
            mesh_unload_list: VecDeque::<IVec3>::with_capacity(MAX_MESH_UNLOAD_LIST),
            mesh_render_list: VecDeque::<IVec3>::with_capacity(MAX_MESHES_TO_RENDER_LIST),
            rendered_meshes: HashMap::with_capacity(MAX_MESHES),
            render_distance: DEFAULT_RENDER_DISTANCE,
            spritesheet_handle: Handle::<Image>::weak(HandleId::Id(Uuid::nil(), 0)),
            material_handle: Handle::<StandardMaterial>::weak(HandleId::Id(Uuid::nil(), 0)),
        }
    }

    /// If voxel_pos are outside of the given voxel, step to the adjacent voxel
    /// in that direction, and update the positions
    pub fn make_coords_valid(chunk_pos: &mut IVec3, voxel_pos: &mut IVec3) {
        let chunk_size = CHUNK_SIZE as i32;
        // Right
        while voxel_pos.x >= chunk_size {
            voxel_pos.x -= chunk_size;
            chunk_pos.x += 1;
        }
        // Left
        while voxel_pos.x < 0 {
            voxel_pos.x += chunk_size;
            chunk_pos.x -= 1;
        }
        // Top
        while voxel_pos.y >= chunk_size {
            voxel_pos.y -= chunk_size;
            chunk_pos.y += 1;
        }
        // Bottom
        while voxel_pos.y < 0 {
            voxel_pos.y += chunk_size;
            chunk_pos.y -= 1;
        }
        // Front
        while voxel_pos.z >= chunk_size {
            voxel_pos.z -= chunk_size;
            chunk_pos.z += 1;
        }
        // Back
        while voxel_pos.z < 0 {
            voxel_pos.z += chunk_size;
            chunk_pos.z -= 1;
        }
    }

    pub fn get_voxel(&self, chunk_pos: &IVec3, voxel_pos: &IVec3) -> Result<&Voxel, ChunkError> {
        let mut new_chunk_pos = *chunk_pos;
        let mut new_voxel_pos = *voxel_pos;
        ChunkManager::make_coords_valid(&mut new_chunk_pos, &mut new_voxel_pos);

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

    pub fn get_adjacent_voxel(
        &self,
        side: Side,
        chunk_pos: &IVec3,
        voxel_pos: &IVec3,
    ) -> Result<&Voxel, ChunkError> {
        let (x, y, z) = (voxel_pos.x, voxel_pos.y, voxel_pos.z);
        Ok(match side {
            Side::Right => self.get_voxel(chunk_pos, &IVec3::new(x + 1, y, z))?,
            Side::Left => self.get_voxel(chunk_pos, &IVec3::new(x - 1, y, z))?,
            Side::Top => self.get_voxel(chunk_pos, &IVec3::new(x, y + 1, z))?,
            Side::Bottom => self.get_voxel(chunk_pos, &IVec3::new(x, y - 1, z))?,
            Side::Front => self.get_voxel(chunk_pos, &IVec3::new(x, y, z + 1))?,
            Side::Back => self.get_voxel(chunk_pos, &IVec3::new(x, y, z - 1))?,
        })
    }

    pub fn get_chunk(&mut self, chunk_pos: &IVec3) -> Option<&mut Chunk> {
        self.chunks.get_mut(chunk_pos)
    }

    pub fn get_adjacent_chunks(
        &self,
        chunk_pos: &IVec3,
    ) -> (
        Option<&Chunk>,
        Option<&Chunk>,
        Option<&Chunk>,
        Option<&Chunk>,
        Option<&Chunk>,
        Option<&Chunk>,
    ) {
        let (x, y, z) = (chunk_pos.x, chunk_pos.y, chunk_pos.z);
        let right = self.chunks.get(&IVec3::new(x + 1, y, z));
        let left = self.chunks.get(&IVec3::new(x - 1, y, z));
        let top = self.chunks.get(&IVec3::new(x, y + 1, z));
        let bottom = self.chunks.get(&IVec3::new(x, y - 1, z));
        let front = self.chunks.get(&IVec3::new(x, y, z + 1));
        let back = self.chunks.get(&IVec3::new(x, y, z - 1));
        (right, left, top, bottom, front, back)
    }

    pub fn load_chunks(&mut self) {
        let mut chunks_loaded = 0;
        while let Some(chunk_pos) = self.chunk_load_list.pop_front() {
            if self.chunks.len() >= MAX_CHUNKS {
                break;
            }

            let mut chunk: Chunk = Chunk::new();
            chunk.setup_perlin(chunk_pos, 1337);

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

    pub fn rebuild_chunks(&mut self, mut commands: Commands) {
        let mut chunks_rebuilt = 0;
        while let Some(chunk_pos) = self.chunk_rebuild_list.pop_front() {
            if let Some(chunk) = self.chunks.get(&chunk_pos) {
                // First remove the mesh from our world
                if let Some(entity) = self.rendered_meshes.remove(&chunk_pos) {
                    // println!(" - Entity removed");
                    commands.entity(entity).despawn();
                }

                // Empty chunks have no mesh, skip
                if chunk.empty == true {
                    self.meshes.insert(chunk_pos, None);
                    continue;
                }

                let mesh = chunk_mesh_builder::build_mesh(self, chunk, &chunk_pos);
                self.meshes.insert(chunk_pos, Some(mesh));
                if !self.mesh_render_list.contains(&chunk_pos) {
                    self.mesh_render_list.push_back(chunk_pos);
                }

                // Update the data of all our neighbors
                let neighbour_chunk_pos: Vec<IVec3> = [
                    IVec3::X,
                    IVec3::NEG_X,
                    IVec3::Y,
                    IVec3::NEG_Y,
                    IVec3::Z,
                    IVec3::NEG_Z,
                ]
                .iter_mut()
                .map(|v| *v + chunk_pos)
                .filter(|v| self.chunks.contains_key(&v))
                .collect();
                println!(
                    "Not sure what to do with these neighbors: {:?}",
                    neighbour_chunk_pos
                );

                chunks_rebuilt += 1;
                if chunks_rebuilt >= MAX_REBUILD_CHUNKS_PER_FRAME {
                    break;
                }
            }
        }
    }

    pub fn load_meshes(&mut self) {
        let mut meshes_loaded = 0;
        while let Some(chunk_pos) = self.mesh_load_list.pop_front() {
            // Skip if we can't hold more meshes
            if self.meshes.len() >= MAX_MESHES
                || self.mesh_render_list.len() >= MAX_MESHES_TO_RENDER_LIST
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
                if !self.mesh_render_list.contains(&chunk_pos) {
                    self.mesh_render_list.push_back(chunk_pos);
                }
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
                            // Copy the chunk, update voxel data, and put it back.. Not very effective
                            if let Some(chunk) = self.chunks.get(&chunk_pos) {
                                let mut updated_chunk = *chunk;
                                updated_chunk.update_voxel_data(self, &chunk_pos);
                                self.chunks.insert(chunk_pos, updated_chunk);
                            }
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

    pub fn render(&mut self, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        let mut rendered_meshes = 0;
        while let Some(chunk_pos) = self.mesh_render_list.pop_front() {
            if self.rendered_meshes.len() >= MAX_MESHES {
                self.mesh_render_list.push_back(chunk_pos);
                return;
            }

            if let Some(mesh_option) = self.meshes.get(&chunk_pos) {
                if let Some(mesh) = mesh_option {
                    if mesh.count_vertices() == 0 {
                        continue;
                    };
                    let Some(collider) = Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh) else { continue; };
                    let chunk_entity = commands
                        .spawn((
                            MaterialMeshBundle {
                                mesh: meshes.add(mesh.clone()),
                                material: self.material_handle.clone(),
                                transform: Transform::from_xyz(
                                    chunk_pos.x as f32 * CHUNK_SIZE as f32,
                                    chunk_pos.y as f32 * CHUNK_SIZE as f32,
                                    chunk_pos.z as f32 * CHUNK_SIZE as f32,
                                ),
                                ..default()
                            },
                            NotShadowCaster,
                        ))
                        .insert(PickableBundle::default())
                        .insert(collider)
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

    pub fn get_chunk_pos_by_entity(&self, entity: Entity) -> Option<IVec3> {
        self.rendered_meshes
            .iter()
            .find_map(|(key, val)| if *val == entity { Some(*key) } else { None })
    }

    pub fn handle_click(&mut self, chunk_pos: &IVec3, hit_pos: &Vec3) {
        let mut voxel_pos = IVec3::new(
            hit_pos.x.round() as i32 - (chunk_pos.x * CHUNK_SIZE as i32),
            hit_pos.y.round() as i32 - (chunk_pos.y * CHUNK_SIZE as i32),
            hit_pos.z.round() as i32 - (chunk_pos.z * CHUNK_SIZE as i32),
        );
        println!("Looking for Voxel pos {} in chunk {}", voxel_pos, chunk_pos);
        let mut new_chunk_pos = *chunk_pos;
        ChunkManager::make_coords_valid(&mut new_chunk_pos, &mut voxel_pos);

        let Some(chunk) = self.chunks.get(&new_chunk_pos) else { return; };
        let mut updated_chunk = chunk.clone();

        println!(
            "Corrected Voxel pos {} in chunk {}",
            voxel_pos, new_chunk_pos
        );
        let voxel_index = Chunk::get_index(&voxel_pos);
        if let Some(mut voxel) = updated_chunk.get_mut_voxel(voxel_index) {
            voxel.active = true;
            voxel.voxel_type = VoxelType::Default;
            updated_chunk.update_voxel_data(self, &new_chunk_pos);
            self.chunks.insert(new_chunk_pos, updated_chunk);
            self.chunk_rebuild_list.push_back(new_chunk_pos);
        }
    }
}
