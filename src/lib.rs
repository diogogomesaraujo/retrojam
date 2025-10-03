use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub mod world;
pub use world::World;
pub mod player;
pub use player::Player;

pub const GRID_WIDTH: usize = 100;
pub const GRID_HEIGHT: usize = 52;
pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 416;
pub const BLOCK_SIZE: i32 = SCREEN_WIDTH / GRID_WIDTH as i32;
pub const DEL_SIZE: i32 = 3;
pub const TARGET_FPS: u32 = 60;
pub const PLAYER_SPEED: f32 = 5.0;
pub const JUMP_SPEED: f32 = 20.0;
pub const SPRITE_SIZE: f32 = 8.0;

pub type WorldMap = HashMap<(usize, usize), BlockType>;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Blank,
    Stone,
}

/// Serializable entry for saving/loading
#[derive(Serialize, Deserialize)]
pub struct BlockEntry {
    pub x: usize,
    pub y: usize,
    pub block_type: BlockType,
}

#[derive(Serialize, Deserialize)]
pub struct MapData {
    pub width: usize,
    pub height: usize,
    pub blocks: Vec<BlockEntry>, // FIXED: Was just Vec
}

/// Recompute stone borders around blanks
pub fn recompute_stone_borders(map: &mut WorldMap) {
    // Remove old stone borders
    map.retain(|_, bt| *bt == BlockType::Blank);

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let blanks: Vec<(usize, usize)> = map
        .iter()
        .filter_map(|(&(x, y), bt)| {
            if *bt == BlockType::Blank {
                Some((x, y))
            } else {
                None
            }
        })
        .collect();

    for (x, y) in blanks {
        for (dx, dy) in dirs {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < GRID_WIDTH as i32 && ny < GRID_HEIGHT as i32 {
                let pos = (nx as usize, ny as usize);
                if !map.contains_key(&pos) {
                    map.insert(pos, BlockType::Stone);
                }
            }
        }
    }
}

/// Load map.json into a WorldMap
pub fn load_map() -> WorldMap {
    if let Ok(content) = fs::read_to_string("map.json") {
        if let Ok(map_data) = serde_json::from_str::<MapData>(&content) {
            if map_data.width == GRID_WIDTH && map_data.height == GRID_HEIGHT {
                println!("Loaded map.json");
                let mut map = HashMap::new();
                for block in map_data.blocks {
                    map.insert((block.x, block.y), block.block_type);
                }
                return map;
            }
        }
    }
    println!("No map.json found, starting empty.");
    HashMap::new()
}

/// Save world to map.json
pub fn save_map(map: &WorldMap) {
    let blocks: Vec<BlockEntry> = map
        .iter()
        .map(|(&(x, y), &bt)| BlockEntry {
            x,
            y,
            block_type: bt,
        })
        .collect();

    let map_data = MapData {
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
        blocks,
    };

    match serde_json::to_string_pretty(&map_data) {
        Ok(json) => {
            if let Err(e) = fs::write("map.json", json) {
                eprintln!("Failed to save map: {}", e);
            } else {
                println!("Map saved to map.json");
            }
        }
        Err(e) => eprintln!("Failed to serialize map: {}", e),
    }
}
