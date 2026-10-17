//! The battlefield camera.

use bevy::prelude::*;

use crate::cursor::Cursor;

/// How many screen pixels each art pixel covers.
const PIXEL_SCALE: f32 = 4.0;

/// How quickly the camera catches up to the cursor; higher is snappier.
const FOLLOW_RATE: f32 = 8.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            PostUpdate,
            follow_cursor.before(TransformSystems::Propagate),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0 / PIXEL_SCALE,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Smoothly pans the camera to keep the cursor centered.
fn follow_cursor(
    time: Res<Time>,
    cursor: Single<&Transform, (With<Cursor>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
) {
    let target = cursor.translation.truncate();
    let current = camera.translation.truncate();
    let next = current.lerp(target, 1.0 - (-FOLLOW_RATE * time.delta_secs()).exp());
    camera.translation = next.extend(camera.translation.z);
}
