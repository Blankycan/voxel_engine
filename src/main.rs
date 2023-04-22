extern crate lazy_static;

use bevy::prelude::*;
use bevy::window::PresentMode;
use voxel_interaction::VoxelInteractionPlugin;

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
mod voxel_interaction;
pub mod voxel_textures;

use voxel_engine::VoxelEnginePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(FlyCameraPlugin)
        .add_plugin(DebugInfoPlugin)
        .add_plugin(VoxelEnginePlugin)
        .add_plugin(VoxelInteractionPlugin)
        .add_startup_system(setup)
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

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-1.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-1.5) * Quat::from_rotation_y(-0.3),
            ..default()
        }
        .into(),
        ..default()
    });
}
