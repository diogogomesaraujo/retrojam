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
pub const PLAYER_SPEED: f32 = 1.5;
pub const JUMP_SPEED: f32 = 5.0;
pub const SPRITE_SIZE: f32 = 8.;
pub const GRAVITY: f32 = 0.7;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BlockType {
    Blank,
    StoneLeftUp,
    StoneLeftDown,
    StoneRightUp,
    StoneRightDown,
    StoneSlabLeft,
    StoneSlabRight,
    StoneSlabUp,
    StoneSlabDown,
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

pub fn recompute_stone_borders(map: &mut WorldMap) {
    map.retain(|_, bt| *bt == BlockType::Blank || *bt == BlockType::Start);

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let pos = (x, y);

            // skip occupied
            if map.contains_key(&pos) {
                continue;
            }

            // check blank all dirs
            let has_blank_up = y > 0 && map.get(&(x, y - 1)) == Some(&BlockType::Blank);
            let has_blank_down =
                y < GRID_HEIGHT - 1 && map.get(&(x, y + 1)) == Some(&BlockType::Blank);
            let has_blank_left = x > 0 && map.get(&(x - 1, y)) == Some(&BlockType::Blank);
            let has_blank_right =
                x < GRID_WIDTH - 1 && map.get(&(x + 1, y)) == Some(&BlockType::Blank);

            let has_blank_up_left =
                x > 0 && y > 0 && map.get(&(x - 1, y - 1)) == Some(&BlockType::Blank);
            let has_blank_up_right =
                x < GRID_WIDTH - 1 && y > 0 && map.get(&(x + 1, y - 1)) == Some(&BlockType::Blank);
            let has_blank_down_left =
                x > 0 && y < GRID_HEIGHT - 1 && map.get(&(x - 1, y + 1)) == Some(&BlockType::Blank);
            let has_blank_down_right = x < GRID_WIDTH - 1
                && y < GRID_HEIGHT - 1
                && map.get(&(x + 1, y + 1)) == Some(&BlockType::Blank);

            // match tuple
            let border_type = match (
                has_blank_up,
                has_blank_down,
                has_blank_left,
                has_blank_right,
                has_blank_up_left,
                has_blank_up_right,
                has_blank_down_left,
                has_blank_down_right,
            ) {
                // exact
                (true, false, false, false, false, false, false, false) => {
                    Some(BlockType::StoneSlabUp)
                }
                (false, true, false, false, false, false, false, false) => {
                    Some(BlockType::StoneSlabDown)
                }
                (false, false, true, false, false, false, false, false) => {
                    Some(BlockType::StoneSlabLeft)
                }
                (false, false, false, true, false, false, false, false) => {
                    Some(BlockType::StoneSlabRight)
                }
                (false, false, false, false, true, false, false, false) => {
                    Some(BlockType::StoneLeftUp)
                }
                (false, false, false, false, false, true, false, false) => {
                    Some(BlockType::StoneRightUp)
                }
                (false, false, false, false, false, false, true, false) => {
                    Some(BlockType::StoneLeftDown)
                }
                (false, false, false, false, false, false, false, true) => {
                    Some(BlockType::StoneRightDown)
                }

                // wildcard -> where the magic happens
                (true, _, _, _, _, _, _, _) => Some(BlockType::StoneSlabUp),
                (_, true, _, _, _, _, _, _) => Some(BlockType::StoneSlabDown),
                (_, _, true, _, _, _, _, _) => Some(BlockType::StoneSlabLeft),
                (_, _, _, true, _, _, _, _) => Some(BlockType::StoneSlabRight),
                (_, _, _, _, true, _, _, _) => Some(BlockType::StoneLeftUp),
                (_, _, _, _, _, true, _, _) => Some(BlockType::StoneRightUp),
                (_, _, _, _, _, _, true, _) => Some(BlockType::StoneLeftDown),
                // No blanks adjacent
                _ => None,
            };

            if let Some(bt) = border_type {
                map.insert(pos, bt);
            }
        }
    }
}
