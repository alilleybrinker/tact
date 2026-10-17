//! Isometric projection math.
//!
//! The battlefield is a grid of tiles addressed by `(x, y)` with an integer
//! height. Tiles use a 2:1 dimetric projection, the classic "isometric"
//! look of pixel-art tactics games: each tile's top face is a diamond twice
//! as wide as it is tall.

use bevy::prelude::*;

/// Width of a tile's top face, in pixels.
pub const TILE_WIDTH: f32 = 32.0;
/// Height of a tile's top face, in pixels.
pub const TILE_HEIGHT: f32 = 16.0;
/// Vertical pixels per unit of tile height.
pub const HEIGHT_STEP: f32 = 8.0;

/// A position on the battlefield grid.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Screen-space (world-space, for the 2D camera) position of the center of
/// a tile's top face at the given height.
pub fn grid_to_world(pos: GridPos, height: i32) -> Vec2 {
    Vec2::new(
        (pos.x - pos.y) as f32 * TILE_WIDTH / 2.0,
        -(pos.x + pos.y) as f32 * TILE_HEIGHT / 2.0 + height as f32 * HEIGHT_STEP,
    )
}

/// Inverse of [`grid_to_world`] for a known height, rounding to the nearest
/// tile.
#[allow(dead_code)]
pub fn world_to_grid(world: Vec2, height: i32) -> GridPos {
    let wx = world.x / (TILE_WIDTH / 2.0);
    let wy = -(world.y - height as f32 * HEIGHT_STEP) / (TILE_HEIGHT / 2.0);
    // wx = x - y, wy = x + y
    GridPos::new(
        ((wy + wx) / 2.0).round() as i32,
        ((wy - wx) / 2.0).round() as i32,
    )
}

/// Draw-order depth for something at a grid position and height.
///
/// Tiles closer to the viewer (larger `x + y`) draw on top, and within a
/// column, higher layers draw over lower ones. `layer` separates things
/// sharing a tile (terrain, cursor, units) and must be below
/// [`LAYERS_PER_HEIGHT`].
pub fn depth(pos: GridPos, height: i32, layer: u32) -> f32 {
    const MAX_HEIGHT: f32 = 64.0;
    let per_cell = MAX_HEIGHT * LAYERS_PER_HEIGHT as f32;
    (pos.x + pos.y) as f32 * per_cell + height as f32 * LAYERS_PER_HEIGHT as f32 + layer as f32
}

/// Number of distinct draw layers available within a single height level.
pub const LAYERS_PER_HEIGHT: u32 = 4;

/// Scale factor applied to depths so they fit in the 2D camera's default
/// clip range.
pub const DEPTH_SCALE: f32 = 0.001;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_maps_to_origin() {
        assert_eq!(grid_to_world(GridPos::new(0, 0), 0), Vec2::ZERO);
    }

    #[test]
    fn axes_run_down_and_sideways() {
        // +x runs down-right, +y runs down-left.
        assert_eq!(grid_to_world(GridPos::new(1, 0), 0), Vec2::new(16.0, -8.0));
        assert_eq!(grid_to_world(GridPos::new(0, 1), 0), Vec2::new(-16.0, -8.0));
    }

    #[test]
    fn height_raises_tiles() {
        assert_eq!(grid_to_world(GridPos::new(0, 0), 2), Vec2::new(0.0, 16.0));
    }

    #[test]
    fn round_trips() {
        for x in -5..5 {
            for y in -5..5 {
                for h in 0..4 {
                    let pos = GridPos::new(x, y);
                    assert_eq!(world_to_grid(grid_to_world(pos, h), h), pos);
                }
            }
        }
    }

    #[test]
    fn nearer_tiles_draw_on_top() {
        let back = depth(GridPos::new(0, 0), 10, 3);
        let front = depth(GridPos::new(1, 0), 0, 0);
        assert!(front > back);
        assert!(depth(GridPos::new(0, 0), 1, 0) > depth(GridPos::new(0, 0), 0, 3));
    }
}
