use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_resource::PrimitiveTopology;

#[macro_use]
extern crate lazy_static;

pub mod chunk;
pub mod voxel;

use chunk::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(test_chunk)
        //.add_startup_system(setup_cube)
        //.add_startup_system(setup_triangle)
        .add_system(update)
        .run();
}

#[derive(Component)]
struct MyCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(60.0, 45.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCamera,
    ));

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

fn test_chunk() {
    print!("Creating chunk.. ");
    let chunk = Chunk::new();
    println!("Done!");

    println!("(0, 0, 0): {}", Chunk::get_index(IVec3::new(0, 0, 0)));
    println!("(1, 0, 0): {}", Chunk::get_index(IVec3::new(1, 0, 0)));
    println!("(0, 1, 0): {}", Chunk::get_index(IVec3::new(0, 1, 0)));
    println!("(0, 0, 1): {}", Chunk::get_index(IVec3::new(0, 0, 1)));
    println!("(1, 1, 1): {}", Chunk::get_index(IVec3::new(1, 1, 1)));
    println!("(15, 15, 15): {}", Chunk::get_index(IVec3::new(15, 15, 15)));
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
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
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
        ([size, -size, -size], [0.0, -1.0, 0.0], [1.0, 0.0]), // BBR
        ([-size, -size, -size], [0.0, -1.0, 0.0], [0.0, 0.0]), // BBL
        ([size, -size, size], [0.0, -1.0, 0.0], [0.0, 1.0]),  // FBR
        ([-size, -size, size], [0.0, -1.0, 0.0], [1.0, 1.0]), // FBL
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

fn update(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds());
        transform.rotate_x(time.delta_seconds() * 0.5);
    }
}
