use crate::*;
use raylib::prelude::*;
use std::error::Error;

pub enum Age {
    Baby,
    Child,
    Teenager,
    Adult,
    Elder,
}

impl Age {
    pub fn to_value(&self) -> f32 {
        match self {
            Self::Baby => 0.0,
            Self::Child => 1.0,
            Self::Teenager => 2.0,
            Self::Adult => 3.0,
            Self::Elder => 4.0,
        }
    }

    pub fn collision_box_height(&self) -> f32 {
        match self {
            Self::Baby => PLAYER_BABY_COLLISION_BOX_HEIGHT,
            Self::Child => PLAYER_CHILD_COLLISION_BOX_HEIGHT,
            Self::Teenager => PLAYER_TEEN_COLLISION_BOX_HEIGHT,
            Self::Adult => PLAYER_ADULT_COLLISION_BOX_HEIGHT,
            Self::Elder => PLAYER_ELDER_COLLISION_BOX_HEIGHT,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn to_value(&self) -> f32 {
        match self {
            Self::Left => -1.0,
            _ => 1.0,
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
    /// Returns true if sprite frame advanced
    pub fn increment_count(&mut self, game_handle: &mut RaylibHandle) -> bool {
        let current_time = game_handle.get_time();
        let mut frame_advanced = false;

        if let PlayerState::Walk { count, last_update } | PlayerState::Jump { count, last_update } =
            self
        {
            if current_time - *last_update > PLAYER_SPRITE_SPEED {
                *count = if *count < PLAYER_SPRITE_WALK_END {
                    *count + 1
                } else {
                    PLAYER_SPRITE_WALK_INIT
                };
                *last_update = current_time;
                frame_advanced = true;
            }
        }
        frame_advanced
    }
}

pub struct Player {
    pub body: Rectangle,
    pub collision_box: Rectangle,
    pub vel: (f32, f32),
    pub state: PlayerState,
    pub sprite: Texture2D,
    pub grounded: bool,
    pub facing: Facing,
    pub age: Age,
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
            collision_box: Rectangle {
                x: x + SPRITE_SIZE / 4.0,
                y: y + SPRITE_SIZE / 4.0,
                width: PLAYER_SCALE * PLAYER_COLLISION_BOX_WIDTH,
                height: PLAYER_SCALE * PLAYER_INITIAL_AGE.collision_box_height(),
            },
            state: PlayerState::Idle,
            sprite: game_handle.load_texture(game_thread, PLAYER_SPRITE_PATH)?,
            grounded: true,
            vel: (0.0, 0.0),
            facing: Facing::Right,
            age: PLAYER_INITIAL_AGE,
        })
    }

    /// Draw player sprite with animation frame
    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D) {
        let sprite_position = match self.state {
            PlayerState::Idle => 0,
            PlayerState::Walk { count, .. } | PlayerState::Jump { count, .. } => count,
        } as f32
            * SPRITE_SIZE;

        d.draw_texture_pro(
            &self.sprite,
            Rectangle {
                x: sprite_position,
                y: self.age.to_value() * SPRITE_SIZE,
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

    fn increment_age(&mut self, game_handle: &mut RaylibHandle) {
        let time_to_change = match &self.age {
            Age::Baby => BABY_TIME_TO_CHANGE,
            Age::Child => CHILD_TIME_TO_CHANGE,
            Age::Teenager => TEENAGER_TIME_TO_CHANGE,
            Age::Adult => ADULT_TIME_TO_CHANGE,
            Age::Elder => ELDER_TIME_TO_CHANGE,
        };

        let time_in_life = game_handle.get_time() as u32 % LIFETIME as u32;

        if time_to_change as u32 == time_in_life {
            self.age = match &self.age {
                Age::Baby => Age::Child,
                Age::Child => Age::Teenager,
                Age::Teenager => Age::Adult,
                Age::Adult => Age::Elder,
                Age::Elder => Age::Baby,
            }
        }
    }

    /// Moves player and returns true if a new walk frame advanced (footstep)
    pub fn after_move(&mut self, game_handle: &mut RaylibHandle, map: &mut WorldMap) -> bool {
        let mut frame_advanced = false;
        let mut moved = false;

        // === AGE UPDATE ===
        self.increment_age(game_handle);

        // === MOVEMENT INPUT ===
        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.vel.0 = PLAYER_SPEED;
            self.facing = Facing::Right;
            moved = true;
        } else if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.vel.0 = -PLAYER_SPEED;
            self.facing = Facing::Left;
            moved = true;
        } else {
            self.vel.0 = 0.0;
        }

        // === JUMP ===
        if (game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE))
            && self.grounded
        {
            self.vel.1 = -JUMP_SPEED;
            self.grounded = false;
            moved = true;
            self.state = PlayerState::Jump {
                count: PLAYER_SPRITE_WALK_INIT,
                last_update: game_handle.get_time(),
            };
        }

        // === GRAVITY ===
        self.vel.1 += GRAVITY;

        // === HORIZONTAL COLLISION ===
        self.body.x += self.vel.0;
        self.collision_box.x += self.vel.0;
        if let Some(block) = self.collides(map) {
            if self.vel.0 > 0.0 {
                self.body.x =
                    block.x - self.collision_box.width - (PLAYER_COLLISION_BOX_WIDTH / 2.0);
                self.collision_box.x = block.x - self.collision_box.width;
            } else if self.vel.0 < 0.0 {
                self.body.x = block.x + block.width - (PLAYER_COLLISION_BOX_WIDTH / 2.0);
                self.collision_box.x = block.x + block.width;
            }
            self.vel.0 = 0.0;
        }

        // === VERTICAL COLLISION ===
        self.body.y += self.vel.1;
        self.collision_box.y += self.vel.1;
        if let Some(block) = self.collides(map) {
            if self.vel.1 > 0.0 {
                self.body.y = block.y - self.body.height;
                self.collision_box.y = block.y - self.collision_box.height;
                self.grounded = true;
            } else if self.vel.1 < 0.0 {
                self.body.y = block.y + block.height;
                self.collision_box.y = block.y + block.height;
            }
            self.vel.1 = 0.0;
        } else {
            self.grounded = false;
        }

        // === STATE UPDATE ===
        if moved {
            match self.state {
                PlayerState::Idle => {
                    // Start walking animation immediately
                    self.state = PlayerState::Walk {
                        count: PLAYER_SPRITE_WALK_INIT,
                        last_update: game_handle.get_time(),
                    };
                    frame_advanced = true;
                }
                PlayerState::Walk { .. } | PlayerState::Jump { .. } => {
                    frame_advanced = self.state.increment_count(game_handle);
                }
            }
        } else if self.grounded {
            self.state = PlayerState::Idle;
        }

        frame_advanced && self.grounded && moved
    }

    pub fn collides(&self, map: &WorldMap) -> Option<Rectangle> {
        for ((x, y), b) in map {
            if *b != BlockType::Blank && *b != BlockType::Start {
                let nx = (*x as f32) * BLOCK_SIZE as f32;
                let ny = (*y as f32) * BLOCK_SIZE as f32;

                let block_rect = Rectangle {
                    x: nx,
                    y: ny,
                    width: BLOCK_SIZE as f32,
                    height: BLOCK_SIZE as f32,
                };

                if block_rect.check_collision_recs(&self.collision_box) {
                    return Some(block_rect);
                }
            }
        }
        None
    }

    pub fn update_state(&mut self, game_handle: &mut RaylibHandle) {
        match self.state {
            PlayerState::Jump { count, last_update } | PlayerState::Walk { count, last_update } => {
                self.state = PlayerState::Walk { count, last_update }
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
