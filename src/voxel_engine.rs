use bevy::{prelude::*, render::primitives::Frustum, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{chunk_manager::ChunkManager, MyCamera};

pub struct VoxelEnginePlugin;

impl Plugin for VoxelEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(load_resources)
            .add_systems((
                load_chunks,
                load_meshes,
                rebuild_data,
                unload_chunks,
                unload_meshes,
                check_visibility,
                render,
            ))
            .add_system(mouse_interaction)
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

fn load_chunks(mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.load_chunks();
}

fn load_meshes(mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.load_meshes();
}

fn rebuild_data(commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.rebuild_chunks(commands);
}

pub fn unload_chunks(mut chunk_manager: ResMut<ChunkManager>) {
    chunk_manager.unload_chunks();
}

pub fn unload_meshes(commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
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

fn mouse_interaction(
    _commands: Commands,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MyCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(window) = window_query.get_single() else { return; };
        let Some(cursor_position) = window.cursor_position() else { return; };
        let Ok((camera, camera_global_transform)) = camera_query.get_single() else { return; };
        let Some(ray) = camera.viewport_to_world(camera_global_transform, cursor_position) else { return; };
        println!(
            "screen: {}, {} ray: {}",
            cursor_position.x, cursor_position.y, ray.origin
        );

        if let Some((entity, toi)) = rapier_context.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            QueryFilter::new(),
        ) {
            let hit_point = ray.get_point(toi);
            println!("Entity {:?} hit at point {}", entity, hit_point);
        }
    }
}
