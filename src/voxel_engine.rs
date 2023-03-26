use bevy::{prelude::*, render::primitives::Frustum};

use crate::chunk_manager::ChunkManager;

pub struct VoxelEnginePlugin;

impl Plugin for VoxelEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((load_data, unload_data, check_visibility, render).chain())
            .init_resource::<ChunkManager>();
    }
}
fn load_data(mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.load_chunks();
    chunk_manager.load_meshes();
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
    materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    chunk_manager.render(commands, meshes, materials);
}
