use bevy::{pbr::NotShadowCaster, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{chunk_manager::ChunkManager, voxel::VoxelType, MyCamera};

pub struct VoxelInteractionPlugin;

impl Plugin for VoxelInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_voxel_indicator)
            .add_system(mouse_interaction);
    }
}

#[derive(Component)]
pub struct VoxelIndicator;

enum SelectorColor {
    Default,
    Blue,
    Red,
}

fn selector_color(color: SelectorColor) -> Color {
    match color {
        SelectorColor::Default => Color::rgba(1.0, 1.0, 1.0, 0.2),
        SelectorColor::Blue => Color::rgba(0.0, 0.584, 0.914, 0.3),
        SelectorColor::Red => Color::rgba(1.0, 0.0, 0.266, 0.3),
    }
}

fn setup_voxel_indicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: selector_color(SelectorColor::Default),
                alpha_mode: AlphaMode::Premultiplied,
                unlit: false,
                ..default()
            }),
            ..default()
        },
        VoxelIndicator,
        NotShadowCaster,
        Name::new("Selector"),
    ));
}

fn mouse_interaction(
    _commands: Commands,
    mouse: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MyCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut voxel_indicator_query: Query<
        (&mut Transform, &Handle<StandardMaterial>, &mut Visibility),
        With<VoxelIndicator>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(window) = window_query.get_single() else { return; };
    let Some(cursor_position) = window.cursor_position() else { return; };
    let Ok((camera, camera_global_transform)) = camera_query.get_single() else { return; };
    let Some(ray) = camera.viewport_to_world(camera_global_transform, cursor_position) else { return; };

    if let Some((entity, toi)) = rapier_context.cast_ray(
        ray.origin,
        ray.direction,
        f32::MAX,
        true,
        QueryFilter::new(),
    ) {
        let Ok((mut selector_transform, selector_material_handle, mut selector_visibility)) = voxel_indicator_query.get_single_mut() else { return; };
        let Some(material) = materials.get_mut(selector_material_handle) else { return; };

        // Check if it's a chunk we're interacting with
        if let Some(chunk_pos) = chunk_manager.get_chunk_pos_by_entity(entity) {
            *selector_visibility = Visibility::Inherited;

            // Left mouse click - Create voxels
            if mouse.pressed(MouseButton::Left) {
                let hit_point = ray.get_point(toi - 0.01);
                if let Some(position) = chunk_manager.get_voxel_position(&chunk_pos, &hit_point) {
                    selector_transform.translation = position;
                    material.base_color = selector_color(SelectorColor::Blue);
                }
            } else if mouse.just_released(MouseButton::Left) {
                let hit_point = ray.get_point(toi - 0.01);
                chunk_manager.update_voxel(&chunk_pos, &hit_point, true, VoxelType::Grass);
            }
            // Right mouse click - Remove voxels
            else if mouse.pressed(MouseButton::Right) {
                let hit_point = ray.get_point(toi + 0.01);
                if let Some(position) = chunk_manager.get_voxel_position(&chunk_pos, &hit_point) {
                    selector_transform.translation = position;
                    material.base_color = selector_color(SelectorColor::Red);
                }
            } else if mouse.just_released(MouseButton::Right) {
                let hit_point = ray.get_point(toi + 0.01);
                chunk_manager.update_voxel(&chunk_pos, &hit_point, false, VoxelType::None);
            }
            // No click, just show voxel indicator
            else {
                let hit_point = ray.get_point(toi - 0.01);
                if let Some(position) = chunk_manager.get_voxel_position(&chunk_pos, &hit_point) {
                    selector_transform.translation = position;
                    material.base_color = selector_color(SelectorColor::Default);
                }
            }
        } else {
            *selector_visibility = Visibility::Hidden;
            material.base_color = selector_color(SelectorColor::Default);
        }
    }
}
