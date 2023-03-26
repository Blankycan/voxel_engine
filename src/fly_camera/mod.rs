use std::f32::EPSILON;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

/// Fly camera, controlled with WASD
///
/// # Example
/// ```
/// pub fn spawn_camera(mut commands: Commands) {
///     commands
///     .spawn(Camera3dBundle {
///         transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
///         ..default()
///     })
///     .insert(FlyCamera::default());
/// }
/// ```
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlyCamera>()
            .add_systems((camera_orientation_system, camera_movement_system));
    }
}

#[derive(Component, Reflect)]
pub struct FlyCamera {
    pub speed: f32,
    pub speed_fast: f32,
    pub rotate_x_sensitivity: f32,
    pub rotate_y_sensitivity: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 5.0,
            speed_fast: 50.0,
            rotate_x_sensitivity: 10.0,
            rotate_y_sensitivity: 10.0,
        }
    }
}

fn camera_orientation_system(
    mut query: Query<(&mut Transform, &FlyCamera)>,
    mouse: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut rotation_move = Vec2::ZERO;

    if mouse.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            rotation_move += ev.delta;
        }
    }

    for (mut transform, fly_camera) in query.iter_mut() {
        if rotation_move.length_squared() > 0.0 {
            let delta_x = -rotation_move.x * fly_camera.rotate_x_sensitivity * time.delta_seconds();
            let delta_y = -rotation_move.y * fly_camera.rotate_y_sensitivity * time.delta_seconds();
            let yaw = Quat::from_rotation_y(delta_x);
            let pitch = Quat::from_rotation_x(delta_y);

            // Rotate around global y axis
            let new_rotation = transform.rotation.slerp(yaw * transform.rotation, 0.2);
            // Rotate around local x axis
            transform.rotation = new_rotation.slerp(new_rotation * pitch, 0.2);
        }
    }
}

fn camera_movement_system(
    mut query: Query<(&mut Transform, &FlyCamera)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::W) {
        direction -= Vec3::Z;
    }
    if input.pressed(KeyCode::S) {
        direction += Vec3::Z;
    }
    if input.pressed(KeyCode::A) {
        direction -= Vec3::X;
    }
    if input.pressed(KeyCode::D) {
        direction += Vec3::X;
    }
    if input.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if input.pressed(KeyCode::LControl) {
        direction -= Vec3::Y;
    }

    if direction.length_squared() < EPSILON {
        return;
    }

    for (mut transform, fly_camera) in query.iter_mut() {
        let speed = if input.pressed(KeyCode::LShift) {
            fly_camera.speed_fast
        } else {
            fly_camera.speed
        };
        let movement = direction.normalize() * speed * time.delta_seconds();
        let orientation = transform.rotation;
        transform.translation += orientation.mul_vec3(movement);
    }
}
