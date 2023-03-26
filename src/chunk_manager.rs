use crate::chunk::CHUNK_SIZE;
use crate::{chunk::Chunk, chunk_mesh_builder};
use bevy::prelude::*;
use bevy::prelude::{Commands, Transform};
use bevy::render::primitives::Frustum;
use bevy::utils::hashbrown::HashMap;

pub const MAX_CHUNKS: usize = 10000;
pub const MAX_CHUNK_LOAD_LIST: usize = 16;
pub const MAX_MESH_LOAD_LIST: usize = 16;
pub const MAX_LOAD_CHUNKS_PER_FRAME: usize = 2;
pub const MAX_LOAD_MESHES_PER_FRAME: usize = 2;
pub const DEFAULT_RENDER_DISTANCE: i32 = 5;

#[derive(Resource)]
pub struct ChunkManager {
    chunks: HashMap<IVec3, Chunk>,
    meshes: HashMap<IVec3, Mesh>,

    chunk_load_list: Vec<IVec3>,
    mesh_load_list: Vec<IVec3>,

    chunk_unload_list: Vec<IVec3>,
    mesh_unload_list: Vec<IVec3>,

    mesh_visible_list: Vec<IVec3>,
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
            meshes: HashMap::with_capacity(MAX_CHUNKS),
            chunk_load_list: Vec::<IVec3>::with_capacity(MAX_CHUNK_LOAD_LIST),
            mesh_load_list: Vec::<IVec3>::with_capacity(MAX_MESH_LOAD_LIST),
            chunk_unload_list: Vec::<IVec3>::with_capacity(MAX_CHUNK_LOAD_LIST),
            mesh_unload_list: Vec::<IVec3>::with_capacity(MAX_MESH_LOAD_LIST),
            mesh_visible_list: Vec::<IVec3>::with_capacity(MAX_CHUNKS),
            rendered_meshes: HashMap::with_capacity(MAX_CHUNKS),
            render_distance: DEFAULT_RENDER_DISTANCE,
        }
    }

    pub fn load_chunks(&mut self) {
        let mut chunks_loaded = 0;
        for chunk_pos in self.chunk_load_list.iter() {
            if self.chunks.len() >= self.chunks.capacity() {
                break;
            }

            let density = if chunk_pos.y == 0 { 0.8 } else { 0.002 };
            let chunk = Chunk::new_random(density);
            self.chunks.insert(*chunk_pos, chunk);
            //println!(" + Chunk {} loaded (Total: {})", chunk_pos, self.chunks.len());

            chunks_loaded += 1;
            if chunks_loaded >= MAX_LOAD_CHUNKS_PER_FRAME {
                break;
            }
        }
        self.chunk_load_list.clear();
    }

    pub fn load_meshes(&mut self) {
        let mut meshes_loaded = 0;
        for chunk_pos in self.mesh_load_list.iter() {
            if self.meshes.len() >= self.meshes.capacity() {
                break;
            }
            if let Some(chunk) = self.chunks.get(chunk_pos) {
                let mesh = chunk_mesh_builder::build_mesh(chunk, chunk_pos);
                self.meshes.insert(*chunk_pos, mesh);
                //println!(" + Mesh {} loaded (Total: {})", chunk_pos, self.meshes.len());
                self.mesh_visible_list.push(*chunk_pos);

                meshes_loaded += 1;
                if meshes_loaded >= MAX_LOAD_MESHES_PER_FRAME {
                    break;
                }
            }
        }
        self.mesh_load_list.clear();
    }

    pub fn unload_chunks(&mut self) {
        for chunk_pos in self.chunk_unload_list.iter() {
            //println!(" - Chunk {} unloaded", chunk_pos);
            self.chunks.remove(&chunk_pos);
        }
        self.chunk_unload_list.clear();
    }

    pub fn unload_meshes(&mut self, mut commands: Commands) {
        for chunk_pos in self.mesh_unload_list.iter() {
            // println!(" - Mesh {} unloaded", chunk_pos);
            self.meshes.remove(chunk_pos);

            if let Some(entity) = self.rendered_meshes.get(chunk_pos) {
                // println!(" - Entity removed");
                commands.entity(*entity).despawn();
                self.rendered_meshes.remove(chunk_pos);
            }
        }
        self.mesh_unload_list.clear();
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

                    // Queue chunk
                    if !self.chunks.contains_key(&chunk_pos)
                        && self.chunk_load_list.len() < self.chunk_load_list.capacity()
                    {
                        //println!("Queue chunk {} for loading..", chunk_pos);
                        self.chunk_load_list.push(chunk_pos);
                    }

                    // Queue mesh
                    if !self.meshes.contains_key(&chunk_pos)
                        && self.mesh_load_list.len() < self.mesh_load_list.capacity()
                    {
                        //println!("Queue mesh {} for loading..", chunk_pos);
                        self.mesh_load_list.push(chunk_pos);
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
            if self.chunk_unload_list.len() < self.chunk_unload_list.capacity() {
                // println!("Queue chunk {} for unloading..", chunk_pos);
                self.chunk_unload_list.push(*chunk_pos);
            }
            if self.mesh_unload_list.len() < self.mesh_unload_list.capacity() {
                self.mesh_unload_list.push(*chunk_pos);
            }
        }
    }

    pub fn render(
        &mut self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for chunk_pos in self.mesh_visible_list.iter() {
            if self.rendered_meshes.len() >= self.rendered_meshes.capacity() {
                return;
            }

            if let Some(mesh) = self.meshes.get(chunk_pos) {
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
                self.rendered_meshes.insert(*chunk_pos, chunk_entity);
            }
        }
        self.mesh_visible_list.clear();
    }
}
