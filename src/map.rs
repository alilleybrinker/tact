//! The battlefield map.

use bevy::prelude::*;

use crate::iso::{self, GridPos};
use crate::tiles::{BLOCK_SIZE, Terrain, TileImages};

/// A single map cell.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub terrain: Terrain,
    pub height: i32,
}

/// The battlefield: a rectangular grid of cells.
#[derive(Resource, Debug)]
pub struct Map {
    pub width: i32,
    pub depth: i32,
    cells: Vec<Cell>,
}

impl Map {
    pub fn get(&self, pos: GridPos) -> Option<Cell> {
        self.contains(pos)
            .then(|| self.cells[(pos.y * self.width + pos.x) as usize])
    }

    pub fn contains(&self, pos: GridPos) -> bool {
        (0..self.width).contains(&pos.x) && (0..self.depth).contains(&pos.y)
    }

    /// Parses a map from rows of `<terrain><height>` cells, where terrain is
    /// one of `g` (grass), `d` (dirt), `s` (stone), or `w` (water), and
    /// height is a single digit.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut cells = Vec::new();
        let mut width = None;
        let mut depth = 0;
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
            let row = line
                .split_whitespace()
                .map(parse_cell)
                .collect::<Result<Vec<_>, _>>()?;
            match width {
                None => width = Some(row.len() as i32),
                Some(w) if w != row.len() as i32 => {
                    return Err(format!("row {depth} has {} cells, expected {w}", row.len()));
                }
                _ => {}
            }
            cells.extend(row);
            depth += 1;
        }
        Ok(Self {
            width: width.ok_or("map is empty")?,
            depth,
            cells,
        })
    }
}

fn parse_cell(token: &str) -> Result<Cell, String> {
    let mut chars = token.chars();
    let terrain = match chars.next() {
        Some('g') => Terrain::Grass,
        Some('d') => Terrain::Dirt,
        Some('s') => Terrain::Stone,
        Some('w') => Terrain::Water,
        _ => return Err(format!("bad terrain in cell {token:?}")),
    };
    let height = chars
        .as_str()
        .parse()
        .map_err(|_| format!("bad height in cell {token:?}"))?;
    Ok(Cell { terrain, height })
}

/// A small hand-made test map.
const TEST_MAP: &str = "
    s3 s3 s2 g1 g1 g1 g0 g0 w0 w0
    s3 s2 g2 g1 g1 g1 g0 g0 w0 w0
    s2 g2 g2 g1 d1 d1 g0 g0 g0 w0
    g2 g2 g1 g1 d1 d1 g1 g0 g0 w0
    g1 g1 g1 d1 d1 d1 g1 g1 g0 g0
    g1 g1 d1 d1 d0 d0 g1 g2 g1 g0
    g0 g1 d1 d0 d0 g0 g1 g2 g2 g1
    g0 g0 d0 d0 g0 g0 g1 s3 s4 g2
    w0 g0 d0 g0 g0 g0 g1 s3 s4 s3
    w0 w0 d0 g0 g0 g0 g1 g2 s3 g2
";

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::parse(TEST_MAP).expect("test map is valid"))
            .add_systems(Startup, spawn_map);
    }
}

/// A single block of terrain in the rendered map.
#[derive(Component)]
pub struct TerrainBlock;

fn spawn_map(mut commands: Commands, map: Res<Map>, images: Res<TileImages>) {
    // The block's top-face center sits this far above the image center.
    let anchor = Vec2::new(0.0, (BLOCK_SIZE.y as f32 - iso::TILE_HEIGHT) / 2.0);
    for y in 0..map.depth {
        for x in 0..map.width {
            let pos = GridPos::new(x, y);
            let cell = map.get(pos).unwrap();
            for level in 0..=cell.height {
                let world = iso::grid_to_world(pos, level) - anchor;
                let z = iso::depth(pos, level, 0) * iso::DEPTH_SCALE;
                commands.spawn((
                    TerrainBlock,
                    pos,
                    Sprite::from_image(images.block(cell.terrain)),
                    Transform::from_translation(world.extend(z)),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parses() {
        let map = Map::parse(TEST_MAP).unwrap();
        assert_eq!((map.width, map.depth), (10, 10));
        let cell = map.get(GridPos::new(8, 7)).unwrap();
        assert_eq!((cell.terrain, cell.height), (Terrain::Stone, 4));
        assert!(map.get(GridPos::new(10, 0)).is_none());
        assert!(map.get(GridPos::new(0, -1)).is_none());
    }

    #[test]
    fn ragged_rows_are_rejected() {
        assert!(Map::parse("g0 g0\ng0").is_err());
    }
}
