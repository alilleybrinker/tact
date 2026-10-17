//! The tile selection cursor.

use bevy::prelude::*;

use crate::iso::{self, GridPos};
use crate::map::Map;
use crate::tiles::{BLOCK_SIZE, TileImages};

/// The tile the cursor is currently over.
#[derive(Resource, Clone, Copy, Debug, Deref, DerefMut)]
pub struct CursorPos(pub GridPos);

/// Marker for the cursor sprite.
#[derive(Component)]
pub struct Cursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPos(GridPos::new(4, 4)))
            .add_systems(Startup, spawn_cursor)
            .add_systems(Update, (move_cursor, place_cursor).chain());
    }
}

fn spawn_cursor(mut commands: Commands, images: Res<TileImages>) {
    commands.spawn((
        Cursor,
        Sprite::from_image(images.cursor.clone()),
        Transform::default(),
    ));
}

/// Moves the cursor with the arrow keys or WASD.
///
/// Directions are screen-relative: "up" moves toward the top of the screen
/// along the grid's up-right axis, matching how the map is drawn.
fn move_cursor(keys: Res<ButtonInput<KeyCode>>, map: Res<Map>, mut cursor: ResMut<CursorPos>) {
    let pressed = |a, b| keys.just_pressed(a) || keys.just_pressed(b);
    let mut delta = IVec2::ZERO;
    if pressed(KeyCode::ArrowUp, KeyCode::KeyW) {
        delta.y -= 1;
    }
    if pressed(KeyCode::ArrowDown, KeyCode::KeyS) {
        delta.y += 1;
    }
    if pressed(KeyCode::ArrowLeft, KeyCode::KeyA) {
        delta.x -= 1;
    }
    if pressed(KeyCode::ArrowRight, KeyCode::KeyD) {
        delta.x += 1;
    }
    let next = GridPos::new(cursor.x + delta.x, cursor.y + delta.y);
    if delta != IVec2::ZERO && map.contains(next) {
        cursor.0 = next;
    }
}

fn place_cursor(
    cursor: Res<CursorPos>,
    map: Res<Map>,
    mut sprite: Single<&mut Transform, With<Cursor>>,
) {
    let Some(cell) = map.get(cursor.0) else {
        return;
    };
    let anchor = Vec2::new(0.0, (BLOCK_SIZE.y as f32 - iso::TILE_HEIGHT) / 2.0);
    let world = iso::grid_to_world(cursor.0, cell.height) - anchor;
    let z = iso::depth(cursor.0, cell.height, 1) * iso::DEPTH_SCALE;
    sprite.translation = world.extend(z);
}
