//! Tact: an isometric, pixel-art tactics game.

mod camera;
mod cursor;
mod iso;
mod map;
mod tiles;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tact".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                // Nearest-neighbor sampling keeps pixel art crisp when scaled.
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb_u8(0x1b, 0x1d, 0x2b)))
        .add_plugins((
            tiles::TilesPlugin,
            map::MapPlugin,
            cursor::CursorPlugin,
            camera::CameraPlugin,
        ))
        .run();
}
