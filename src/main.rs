#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::primitives::Frustum;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::PresentMode;

mod debug_info;
mod fly_camera;
use crate::debug_info::DebugInfoPlugin;
use crate::fly_camera::{FlyCamera, FlyCameraPlugin};

pub mod chunk;
mod chunk_manager;
mod chunk_mesh_builder;
pub mod face;
pub mod voxel;
mod voxel_engine;

use chunk::*;
use voxel_engine::VoxelEnginePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(FlyCameraPlugin)
        .add_plugin(DebugInfoPlugin)
        .add_plugin(VoxelEnginePlugin)
        .add_startup_system(setup)
        //.add_startup_system(create_chunk)
        //.add_startup_system(create_chunk_by_individual_cubes)
        //.add_startup_system(setup_cube)
        //.add_startup_system(setup_triangle)
        .run();
}

#[derive(Component)]
struct MyCamera;

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(30.0, 25.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCamera::default())
        .insert(MyCamera)
        .insert(Name::new("Fly Camera"));

    /*
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.8,
    });
    */

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-1.5),
            ..default()
        }
        .into(),
        ..default()
    });
}

fn create_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk = Chunk::new_random(0.8);

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(create_chunk_mesh(&chunk).into()),
        material: materials.add(Color::WHITE.into()),
        //transform: Transform::from_xyz(x as f32, y as f32, z as f32),
        ..default()
    });
}

fn create_chunk_by_individual_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk = Chunk::new_random(0.8);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let index: usize = Chunk::index_from(x, y, z);

                if let Some(voxel) = chunk.get_voxel(index) {
                    if !voxel.active {
                        continue;
                    }

                    commands.spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(create_cube().into()),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                            ..default()
                        },
                        Cube,
                    ));
                }
            }
        }
    }
}

#[derive(Component)]
struct Triangle;

#[derive(Component)]
struct Cube;

fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(create_cube().into()),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
        Cube,
    ));
    /*
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(2.0, -0.5, -2.0),
            ..default()
        },
        Cube,
    ));
    */
}

fn create_chunk_mesh(chunk: &Chunk) -> Mesh {
    const HALF_SIZE: f32 = 0.5;

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

fn create_cube() -> Mesh {
    /*
    [size, size, size],    // FTR
    [-size, size, size],   // FTL
    [-size, -size, size],  // FBL
    [size, -size, size],   // FBR
    [size, size, -size],   // BTR
    [-size, size, -size],  // BTL
    [-size, -size, -size], // BBL
    [size, -size, -size],  // BBR
    */
    let size = 0.5;
    let vertices = [
        // Front
        ([size, size, size], [0.0, 0.0, 1.0], [1.0, 0.0]), // FTR
        ([-size, size, size], [0.0, 0.0, 1.0], [0.0, 0.0]), // FTL
        ([-size, -size, size], [0.0, 0.0, 1.0], [0.0, 1.0]), // FBL
        ([size, -size, size], [0.0, 0.0, 1.0], [1.0, 1.0]), // FBR
        // Right
        ([size, size, -size], [1.0, 0.0, 0.0], [1.0, 0.0]), // BTR
        ([size, size, size], [1.0, 0.0, 0.0], [0.0, 0.0]),  // FTR
        ([size, -size, size], [1.0, 0.0, 0.0], [0.0, 1.0]), // FBR
        ([size, -size, -size], [1.0, 0.0, 0.0], [1.0, 1.0]), // BBR
        // Back
        ([-size, size, -size], [0.0, 0.0, -1.0], [1.0, 0.0]), // BTL
        ([size, size, -size], [0.0, 0.0, -1.0], [0.0, 0.0]),  // BTR
        ([size, -size, -size], [0.0, 0.0, -1.0], [0.0, 1.0]), // BBR
        ([-size, -size, -size], [0.0, 0.0, -1.0], [1.0, 1.0]), // BBL
        // Left
        ([-size, size, size], [-1.0, 0.0, 0.0], [1.0, 0.0]), // FTL
        ([-size, size, -size], [-1.0, 0.0, 0.0], [0.0, 0.0]), // BTL
        ([-size, -size, -size], [-1.0, 0.0, 0.0], [0.0, 1.0]), // BBL
        ([-size, -size, size], [-1.0, 0.0, 0.0], [1.0, 1.0]), // FBL
        // Top
        ([size, size, -size], [0.0, 1.0, 0.0], [1.0, 0.0]), // BTR
        ([-size, size, -size], [0.0, 1.0, 0.0], [0.0, 0.0]), // BTL
        ([-size, size, size], [0.0, 1.0, 0.0], [0.0, 1.0]), // FTL
        ([size, size, size], [0.0, 1.0, 0.0], [1.0, 1.0]),  // FTR
        // Bottom
        ([-size, -size, -size], [0.0, -1.0, 0.0], [1.0, 0.0]), // BBL
        ([size, -size, -size], [0.0, -1.0, 0.0], [0.0, 0.0]),  // BBR
        ([size, -size, size], [0.0, -1.0, 0.0], [0.0, 1.0]),   // FBR
        ([-size, -size, size], [0.0, -1.0, 0.0], [1.0, 1.0]),  // FBL
    ];

    let front_indices = vec![0, 1, 2, 0, 2, 3];
    let right_indices = vec![4, 5, 6, 4, 6, 7];
    let back_indices = vec![8, 9, 10, 8, 10, 11];
    let left_indices = vec![12, 13, 14, 12, 14, 15];
    let top_indices = vec![16, 17, 18, 16, 18, 19];
    let bottom_indices = vec![20, 21, 22, 20, 22, 23];
    let mut indices = front_indices;
    indices.extend(right_indices);
    indices.extend(back_indices);
    indices.extend(left_indices);
    indices.extend(top_indices);
    indices.extend(bottom_indices);
    let indices = Indices::U32(indices);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (position, normal, uv) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
