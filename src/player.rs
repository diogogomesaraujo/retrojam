use crate::*;
use raylib::{ffi::CheckCollisionRecs, prelude::*};
use std::error::Error;

const PLAYER_SPRITE_PATH: &str = "src/assets/player.png";
const PLAYER_SPRITE_WALK_INIT: u32 = 1;
const PLAYER_SPRITE_WALK_END: u32 = 5;
const PLAYER_SPRITE_SPEED: f64 = 0.15;
const PLAYER_SCALE: f32 = 1.;

#[derive(Clone, Debug)]
pub enum PlayerFacing {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum PlayerState {
    Idle,
    Walk { count: u32, last_update: f64 },
    Jump { count: u32, last_update: f64 },
}

impl PlayerState {
    // returns the last update date!
    pub fn increment_count(&mut self, game_handle: &mut RaylibHandle) {
        let current_time = game_handle.get_time();

        if let PlayerState::Walk { count, last_update } | PlayerState::Jump { count, last_update } =
            self
        {
            if current_time - *last_update > PLAYER_SPRITE_SPEED {
                match *count < PLAYER_SPRITE_WALK_END {
                    true => *count += 1,
                    _ => *count = PLAYER_SPRITE_WALK_INIT,
                }
                *last_update = current_time;
            }
        }
    }
}

pub struct Player {
    pub body: Rectangle,
    pub state: PlayerState,
    pub sprite: Texture2D,
    pub grounded: bool,
}

impl Player {
    pub fn new(
        game_handle: &mut RaylibHandle,
        game_thread: &RaylibThread,
        x: f32,
        y: f32,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            body: Rectangle {
                x,
                y,
                width: PLAYER_SCALE * SPRITE_SIZE,
                height: PLAYER_SCALE * SPRITE_SIZE,
            },
            state: PlayerState::Idle,
            sprite: game_handle.load_texture(game_thread, PLAYER_SPRITE_PATH)?,
            grounded: true,
        })
    }

    pub fn draw(&mut self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle_rec(self.body, Color::RED);
        let sprite_position = match self.state {
            PlayerState::Idle => 0,
            PlayerState::Walk {
                count,
                last_update: _,
            }
            | PlayerState::Jump {
                count,
                last_update: _,
            } => count,
        } as f32
            * SPRITE_SIZE;
        draw_handle.draw_texture_pro(
            &self.sprite,
            Rectangle {
                x: sprite_position,
                y: 0.,
                width: SPRITE_SIZE,
                height: SPRITE_SIZE,
            },
            Rectangle {
                x: self.body.x,
                y: self.body.y,
                width: PLAYER_SCALE * SPRITE_SIZE,
                height: PLAYER_SCALE * SPRITE_SIZE,
            },
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }

    pub fn after_move(&mut self, game_handle: &mut RaylibHandle) {
        let mut moved = false;

        self.state.increment_count(game_handle);

        println!("Player: {:?}", self.state);

        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.body.x += PLAYER_SPEED;
            self.update_state(game_handle);
            moved = true;
        }

        if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.body.x -= PLAYER_SPEED;
            self.update_state(game_handle);
            moved = true;
        }

        if game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE)
        {
            self.body.y -= JUMP_SPEED;
            self.update_state(game_handle);
            moved = true;
        }

        if self.body.y < 200. {
            self.body.y += PLAYER_SPEED;
            self.update_state(game_handle);
            moved = true;
        }

        if !moved {
            self.state = PlayerState::Idle;
        }
    }

    //floor or wall
    pub fn collides(&self, map: &mut WorldMap) -> bool {
        for ((x, y), b) in map {
            if *b == BlockType::Stone {
                let nx = (*x as f32) * BLOCK_SIZE as f32;
                let ny = (*y as f32) * BLOCK_SIZE as f32;

                let block_rect = Rectangle {
                    x: nx,
                    y: ny,
                    width: BLOCK_SIZE as f32,
                    height: BLOCK_SIZE as f32,
                };
                if block_rect.check_collision_recs(&self.body) {
                    return true;
                }
            }
        }
        false
    }

    pub fn update_state(&mut self, game_handle: &mut RaylibHandle) {
        match self.state {
            PlayerState::Jump { count, last_update } | PlayerState::Walk { count, last_update } => {
                self.state = PlayerState::Walk {
                    count: count,
                    last_update: last_update,
                }
            }
            _ => {
                self.state = PlayerState::Walk {
                    count: 0,
                    last_update: game_handle.get_time(),
                }
            }
        }
    }
}
