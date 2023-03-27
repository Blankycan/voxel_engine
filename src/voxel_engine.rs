use bevy::{prelude::*, render::primitives::Frustum};

use crate::chunk_manager::ChunkManager;

pub struct VoxelEnginePlugin;

impl Plugin for VoxelEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_resources)
            .add_systems(
                (
                    load_data,
                    rebuild_data,
                    unload_data,
                    check_visibility,
                    render,
                )
                    .chain(),
            )
            .init_resource::<ChunkManager>();
    }
}

fn load_resources(
    mut chunk_manager: ResMut<ChunkManager>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let spritesheet_handle = asset_server.load("spritesheet.png");
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        base_color_texture: Some(spritesheet_handle.clone()),
        alpha_mode: AlphaMode::Opaque,
        unlit: false,
        fog_enabled: true,
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.125,
        ..default()
    });

    chunk_manager.spritesheet_handle = spritesheet_handle;
    chunk_manager.material_handle = material_handle;
}

fn load_data(mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.load_chunks();
    chunk_manager.load_meshes();
}

fn rebuild_data(commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.rebuild_chunks(commands);
}

pub fn unload_data(commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.unload_chunks();
    chunk_manager.unload_meshes(commands);
}

fn check_visibility(
    camera_query: Query<(&Transform, &Frustum)>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (transform, frustrum) = camera_query.single();
    chunk_manager.update_visible(transform, frustrum);
}

fn render(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    chunk_manager.render(commands, meshes);
}
