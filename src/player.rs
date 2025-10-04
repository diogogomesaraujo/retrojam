use crate::*;
use raylib::prelude::*;
use std::error::Error;

#[derive(Clone, Debug)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn to_value(&self) -> f32 {
        match self {
            Self::Left => -1.,
            _ => 1.,
        }
    }
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
    pub vel: (f32, f32),
    pub state: PlayerState,
    pub sprite: Texture2D,
    pub grounded: bool,
    pub facing: Facing,
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
            vel: (0., 0.),
            facing: Facing::Right,
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
                y: 0. * SPRITE_SIZE,
                width: SPRITE_SIZE * self.facing.to_value(),
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

    pub fn after_move(&mut self, game_handle: &mut RaylibHandle, map: &mut WorldMap) {
        let mut moved = false;
        self.state.increment_count(game_handle);

        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.vel.0 = PLAYER_SPEED;
            match self.facing {
                Facing::Left => self.facing = Facing::Right,
                _ => {}
            }
            moved = true;
        } else if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.vel.0 = -PLAYER_SPEED;
            match self.facing {
                Facing::Right => self.facing = Facing::Left,
                _ => {}
            }
            moved = true;
        } else {
            self.vel.0 = 0.0;
        }

        if (game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE))
            && self.grounded
        {
            self.vel.1 = -JUMP_SPEED;
            self.grounded = false;
            moved = true;
        }

        self.vel.1 += GRAVITY;

        self.body.x += self.vel.0;
        if let Some(block) = self.collides(map) {
            if self.vel.0 > 0.0 {
                self.body.x = block.x - self.body.width;
            } else if self.vel.0 < 0.0 {
                self.body.x = block.x + block.width;
            }
            self.vel.0 = 0.0;
        }

        self.body.y += self.vel.1;
        if let Some(block) = self.collides(map) {
            if self.vel.1 > 0.0 {
                self.body.y = block.y - self.body.height;
                self.grounded = true;
            } else if self.vel.1 < 0.0 {
                self.body.y = block.y + block.height;
            }
            self.vel.1 = 0.0;
        } else {
            self.grounded = false;
        }

        if moved {
            self.update_state(game_handle);
        } else if self.grounded {
            self.state = PlayerState::Idle;
        }
    }

    //floor or wall
    pub fn collides(&self, map: &WorldMap) -> Option<Rectangle> {
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
                    return Some(block_rect);
                }
            }
        }
        None
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
