//! Placeholder pixel-art tile graphics, generated at startup.
//!
//! These stand in for real art until we have a tileset. Each terrain kind
//! gets a "block" image: a diamond top face plus one height step of side
//! faces beneath it. Taller columns are built by stacking blocks.

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::iso::{HEIGHT_STEP, TILE_HEIGHT, TILE_WIDTH};

const W: u32 = TILE_WIDTH as u32;
const TOP: u32 = TILE_HEIGHT as u32;
const SIDE: u32 = HEIGHT_STEP as u32;

/// Size of a block image, in pixels.
pub const BLOCK_SIZE: UVec2 = UVec2::new(W, TOP + SIDE);

/// Kinds of terrain a tile can be made of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Terrain {
    Grass,
    Dirt,
    Stone,
    Water,
}

impl Terrain {
    pub const ALL: [Terrain; 4] = [
        Terrain::Grass,
        Terrain::Dirt,
        Terrain::Stone,
        Terrain::Water,
    ];

    /// Top, left side, and right side colors.
    fn palette(self) -> [Srgba; 3] {
        let hex = |s| Srgba::hex(s).unwrap();
        match self {
            Terrain::Grass => [hex("6abe30"), hex("4b692f"), hex("37946e")],
            Terrain::Dirt => [hex("d9a066"), hex("8f563b"), hex("663931")],
            Terrain::Stone => [hex("9badb7"), hex("696a6a"), hex("595652")],
            Terrain::Water => [hex("5fcde4"), hex("306082"), hex("3f3f74")],
        }
    }
}

/// Handles to the generated tile images.
#[derive(Resource)]
pub struct TileImages {
    blocks: [Handle<Image>; Terrain::ALL.len()],
    pub cursor: Handle<Image>,
}

impl TileImages {
    pub fn block(&self, terrain: Terrain) -> Handle<Image> {
        self.blocks[terrain as usize].clone()
    }
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        let mut images = app.world_mut().resource_mut::<Assets<Image>>();
        let blocks = Terrain::ALL.map(|t| images.add(block_image(t)));
        let cursor = images.add(cursor_image());
        app.insert_resource(TileImages { blocks, cursor });
    }
}

/// Whether pixel `(x, y)` lies within the top-face diamond.
fn in_diamond(x: u32, y: u32) -> bool {
    let dx = (x as f32 + 0.5 - W as f32 / 2.0).abs() / (W as f32 / 2.0);
    let dy = (y as f32 + 0.5 - TOP as f32 / 2.0).abs() / (TOP as f32 / 2.0);
    dx + dy <= 1.0
}

/// Whether pixel `(x, y)` lies on the edge of the top-face diamond.
fn on_diamond_edge(x: u32, y: u32) -> bool {
    in_diamond(x, y)
        && [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().any(|&(ox, oy)| {
            let (nx, ny) = (x as i32 + ox, y as i32 + oy);
            nx < 0 || ny < 0 || !in_diamond(nx as u32, ny as u32)
        })
}

fn blank_image() -> Image {
    Image::new_fill(
        Extent3d {
            width: BLOCK_SIZE.x,
            height: BLOCK_SIZE.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

fn block_image(terrain: Terrain) -> Image {
    let [top, left, right] = terrain.palette();
    let mut image = blank_image();
    for y in 0..BLOCK_SIZE.y {
        for x in 0..BLOCK_SIZE.x {
            let color = if in_diamond(x, y) {
                // A sparse dither gives the top face a bit of texture.
                if (x * 7 + y * 13) % 11 == 0 {
                    top.mix(&left, 0.35)
                } else if on_diamond_edge(x, y) {
                    top.mix(&Srgba::WHITE, 0.2)
                } else {
                    top
                }
            } else if y >= TOP / 2 && (y - SIDE..y).any(|sy| in_diamond(x, sy)) {
                // Side faces are the bottom half of the diamond swept down.
                let shade = if x < W / 2 { left } else { right };
                if y == BLOCK_SIZE.y - 1 || !(y + 1 - SIDE..=y).any(|sy| in_diamond(x, sy)) {
                    shade.mix(&Srgba::BLACK, 0.3)
                } else {
                    shade
                }
            } else {
                continue;
            };
            image.set_color_at(x, y, color.into()).unwrap();
        }
    }
    image
}

fn cursor_image() -> Image {
    let mut image = blank_image();
    let color = Srgba::hex("fbf236").unwrap();
    for y in 0..TOP {
        for x in 0..W {
            if on_diamond_edge(x, y) {
                image.set_color_at(x, y, color.into()).unwrap();
            }
        }
    }
    image
}
