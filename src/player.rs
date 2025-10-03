use crate::*;
use raylib::prelude::*;
use std::error::Error;

const PLAYER_SPRITE_PATH: &str = "src/assets/player.png";
const PLAYER_SPRITE_WALK_INIT: u32 = 1;
const PLAYER_SPRITE_WALK_END: u32 = 5;
const PLAYER_SPRITE_SPEED: f64 = 0.15;
const PLAYER_SCALE: f32 = 10.;

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
}

impl Player {
    pub fn new(
        game_handle: &mut RaylibHandle,
        game_thread: &RaylibThread,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            body: Rectangle {
                x: (game_handle.get_screen_width() / 2) as f32,
                y: (game_handle.get_screen_height() / 2) as f32,
                width: PLAYER_SCALE * SPRITE_SIZE,
                height: PLAYER_SCALE * SPRITE_SIZE,
            },
            state: PlayerState::Idle,
            sprite: game_handle.load_texture(game_thread, PLAYER_SPRITE_PATH)?,
        })
    }

    pub fn draw(&mut self, draw_handle: &mut RaylibDrawHandle) {
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

    pub fn after_move(self, game_handle: &mut RaylibHandle) -> Self {
        let mut player_after_move = self;
        let mut moved = false;

        player_after_move.state.increment_count(game_handle);

        println!("Player: {:?}", player_after_move.state);

        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            player_after_move.body.x += PLAYER_SPEED;
            player_after_move.update_state(game_handle);
            moved = true;
        }

        if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            player_after_move.body.x -= PLAYER_SPEED;
            player_after_move.update_state(game_handle);
            moved = true;
        }

        if game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE)
        {
            player_after_move.body.y -= JUMP_SPEED;
            player_after_move.update_state(game_handle);
            moved = true;
        }

        if player_after_move.body.y < 300. {
            player_after_move.body.y += PLAYER_SPEED;
            player_after_move.update_state(game_handle);
            moved = true;
        }

        if !moved {
            player_after_move.state = PlayerState::Idle;
        }

        player_after_move
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
