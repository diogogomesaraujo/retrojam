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

pub const PLAYER_SCALE: f32 = 1.;

pub const PLAYER_SPEED: f32 = 1.;
pub const JUMP_SPEED: f32 = 2.5;
pub const GRAVITY: f32 = 0.2;

pub const PLAYER_SPRITE_PATH: &str = "src/assets/player.png";

pub const PLAYER_SPRITE_WALK_INIT: u32 = 1;
pub const PLAYER_SPRITE_WALK_END: u32 = 5;
pub const PLAYER_SPRITE_SPEED: f64 = 0.15;

pub const SPRITE_SIZE: f32 = 8.;

pub const CAMERA_ZOOM: f32 = 6.;
pub const CAMERA_SPEED: f32 = 0.1;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Blank,
    Stone,
    Start,
}

#[derive(Serialize, Deserialize)]
struct SerializableMap {
    #[serde(with = "tuple_vec_map")]
    blocks: HashMap<(usize, usize), BlockType>,
}

mod tuple_vec_map {
    use super::BlockType;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    struct Entry {
        x: usize,
        y: usize,
        block_type: BlockType,
    }

    pub fn serialize<S>(
        map: &HashMap<(usize, usize), BlockType>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entries: Vec<Entry> = map
            .iter()
            .map(|(&(x, y), &block_type)| Entry { x, y, block_type })
            .collect();
        entries.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<(usize, usize), BlockType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = Vec::<Entry>::deserialize(deserializer)?;
        Ok(entries
            .into_iter()
            .map(|e| ((e.x, e.y), e.block_type))
            .collect())
    }
}

pub type WorldMap = HashMap<(usize, usize), BlockType>;

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
    match fs::read_to_string("map.json") {
        Ok(content) => match serde_json::from_str::<SerializableMap>(&content) {
            Ok(serializable_map) => {
                println!(
                    "Loaded map.json with {} blocks",
                    serializable_map.blocks.len()
                );
                serializable_map.blocks
            }
            Err(e) => {
                eprintln!("Failed to parse map.json: {}", e);
                HashMap::new()
            }
        },
        Err(_) => {
            println!("No map.json found, starting empty.");
            HashMap::new()
        }
    }
}

/// Save world to map.json
pub fn save_map(map: &WorldMap) {
    let serializable_map = SerializableMap {
        blocks: map.clone(),
    };

    match serde_json::to_string_pretty(&serializable_map) {
        Ok(json) => {
            if let Err(e) = fs::write("map.json", json) {
                eprintln!("Failed to save map: {}", e);
            } else {
                println!("Map saved to map.json with {} blocks", map.len());
            }
        }
        Err(e) => eprintln!("Failed to serialize map: {}", e),
    }
}

pub fn smoothing(a: f32, b: f32, s: f32) -> f32 {
    a + (b - a) * s
}
