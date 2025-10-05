use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub mod world;
pub use world::World;
pub mod player;
pub use player::Player;
pub mod dialogue;
pub use dialogue::DialogueSystem;

pub mod shaders;

pub mod dust;

use crate::player::Age;

pub const BG_COLOR: Color = Color {
    r: 29,
    g: 32,
    b: 33,
    a: 255,
};

pub const GRID_WIDTH: usize = 100;
pub const GRID_HEIGHT: usize = 52;

pub const BASE_WIDTH: i32 = 800;
pub const BASE_HEIGHT: i32 = 416;

pub const BLOCK_SIZE: i32 = 8;

pub const DEL_SIZE: i32 = 3;
pub const TARGET_FPS: u32 = 60;

pub const PLAYER_SCALE: f32 = 1.;

pub const PLAYER_SPEED: f32 = 1.;
pub const JUMP_SPEED: f32 = 2.5;
pub const GRAVITY: f32 = 0.15;

pub const PLAYER_SPRITE_PATH: &str = "src/assets/player.png";
pub const TILESET_PATH: &str = "src/assets/tileset.png";
pub const DEVIL_PATH: &str = "src/assets/devil.png";
pub const BG_PATH: &str = "src/assets/background.png";
pub const PARTICLE_PATH: &str = "src/assets/particle.png";

pub const PLAYER_SPRITE_WALK_INIT: u32 = 1;
pub const PLAYER_SPRITE_WALK_END: u32 = 5;
pub const PLAYER_SPRITE_SPEED: f64 = 0.15;

pub const SPRITE_SIZE: f32 = 8.;
pub const DEVIL_HEIGHT: f32 = SPRITE_SIZE * 3.;

pub const CAMERA_ZOOM: f32 = 6.;
pub const CAMERA_SPEED: f32 = 0.08;

// Collision box constants
pub const COLLISION_BOX_OFFSET_X: f32 = SPRITE_SIZE / 4.0;
pub const COLLISION_BOX_OFFSET_Y: f32 = SPRITE_SIZE / 4.0;
pub const PLAYER_COLLISION_BOX_WIDTH: f32 = SPRITE_SIZE / 2.;

pub const PLAYER_ELDER_COLLISION_BOX_HEIGHT: f32 = SPRITE_SIZE / 4. * 3.;
pub const PLAYER_ADULT_COLLISION_BOX_HEIGHT: f32 = SPRITE_SIZE / 6. * 5.3;
pub const PLAYER_TEEN_COLLISION_BOX_HEIGHT: f32 = SPRITE_SIZE / 4. * 3.;
pub const PLAYER_CHILD_COLLISION_BOX_HEIGHT: f32 = SPRITE_SIZE / 6. * 3.8;
pub const PLAYER_BABY_COLLISION_BOX_HEIGHT: f32 = SPRITE_SIZE / 4. * 3.;

pub const PLAYER_SPRITE_DYING_INIT: u32 = 0;
pub const PLAYER_SPRITE_DYING_END: u32 = 4;
pub const IDLE_DYING_TRIGGER_TIME: f64 = 1.0;

pub const PLAYER_INITIAL_AGE: Age = Age::Baby;

pub const LIFETIME: f64 = ELDER_TIME_TO_CHANGE + 5.;
pub const BABY_TIME_TO_CHANGE: f64 = 5.;
pub const CHILD_TIME_TO_CHANGE: f64 = 10.;
pub const TEENAGER_TIME_TO_CHANGE: f64 = 15.;
pub const ADULT_TIME_TO_CHANGE: f64 = 20.;
pub const ELDER_TIME_TO_CHANGE: f64 = 25.;
pub const DEATH_ANIMATION_DURATION: f64 = 3.0;

pub const NUMBER_OF_PARTICLES: u32 = 400;
pub const PARTICLE_VELOCITY: f32 = 0.2;
pub const END_SCENE_SIGHT_MULTIPLIER: f32 = 1.5;
pub const SIGHT_TRANSITION_SPEED_NORMAL: f32 = 2.0;
pub const SIGHT_TRANSITION_SPEED_END: f32 = 0.5;
pub const END_SCENE_CAMERA_OFFSET_Y: f32 = -25.0;
pub const END_SCENE_CAMERA_TRANSITION_SPEED: f32 = 0.02;
pub const END_BEFORE_DIALOGUE: f64 = 5.;

// Proximity detection
pub const END_BLOCK_PROXIMITY_THRESHOLD: f32 = 4.0;

// Death sprite row
pub const DEATH_SPRITE_ROW: f32 = 5.0;

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
    Slab,
    Start,
    StopAging,
    End,
}

impl BlockType {
    pub fn to_sprite_position(&self) -> (f32, f32) {
        match self {
            Self::Blank | Self::Start | Self::End | Self::StopAging => (1., 1.),
            Self::StoneLeftDown => (0., 2.),
            Self::StoneLeftUp => (0., 0.),
            Self::StoneRightDown => (2., 2.),
            Self::StoneRightUp => (2., 0.),
            Self::StoneSlabDown => (1., 2.),
            Self::StoneSlabLeft => (2., 1.),
            Self::StoneSlabRight => (2., 1.),
            Self::StoneSlabUp | Self::Slab => (1., 2.),
        }
    }

    pub fn is_collidable(&self) -> bool {
        !matches!(self, Self::Blank | Self::Start | Self::StopAging)
    }

    pub fn is_special_zone(&self) -> bool {
        matches!(self, Self::StopAging | Self::End)
    }
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

            if map.contains_key(&pos) {
                continue;
            }

            let up = y > 0 && matches!(map.get(&(x, y - 1)), Some(BlockType::Blank));
            let down =
                y < GRID_HEIGHT - 1 && matches!(map.get(&(x, y + 1)), Some(BlockType::Blank));
            let left = x > 0 && matches!(map.get(&(x - 1, y)), Some(BlockType::Blank));
            let right =
                x < GRID_WIDTH - 1 && matches!(map.get(&(x + 1, y)), Some(BlockType::Blank));

            let up_left =
                x > 0 && y > 0 && matches!(map.get(&(x - 1, y - 1)), Some(BlockType::Blank));
            let up_right = x < GRID_WIDTH - 1
                && y > 0
                && matches!(map.get(&(x + 1, y - 1)), Some(BlockType::Blank));
            let down_left = x > 0
                && y < GRID_HEIGHT - 1
                && matches!(map.get(&(x - 1, y + 1)), Some(BlockType::Blank));
            let down_right = x < GRID_WIDTH - 1
                && y < GRID_HEIGHT - 1
                && matches!(map.get(&(x + 1, y + 1)), Some(BlockType::Blank));

            let border_type = if up_left && !up && !left {
                Some(BlockType::StoneRightDown)
            } else if up_right && !up && !right {
                Some(BlockType::StoneLeftDown)
            } else if down_left && !down && !left {
                Some(BlockType::StoneRightUp)
            } else if down_right && !down && !right {
                Some(BlockType::StoneLeftUp)
            } else if up {
                Some(BlockType::StoneSlabDown)
            } else if down {
                Some(BlockType::StoneSlabUp)
            } else if left {
                Some(BlockType::StoneSlabRight)
            } else if right {
                Some(BlockType::StoneSlabLeft)
            } else {
                None
            };

            if let Some(bt) = border_type {
                map.insert(pos, bt);
            }
        }
    }
}

pub fn smoothing(a: f32, b: f32, s: f32) -> f32 {
    a + (b - a) * s
}
