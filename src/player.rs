use crate::*;
use raylib::prelude::*;
use std::error::Error;

pub struct AgeAttributes {
    pub sight: f32,
    pub strength: f32,
    pub speed: f32,
    pub jump_cooldown: f32,
}

#[derive(Debug, Clone)]
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

    pub fn attributes(&self) -> AgeAttributes {
        match self {
            Self::Baby => AgeAttributes {
                sight: 0.5,
                strength: 0.2,
                speed: 0.3,
                jump_cooldown: 2.0,
            },
            Self::Child => AgeAttributes {
                sight: 0.7,
                strength: 0.4,
                speed: 0.6,
                jump_cooldown: 1.5,
            },
            Self::Teenager => AgeAttributes {
                sight: 1.0,
                strength: 0.7,
                speed: 0.9,
                jump_cooldown: 0.8,
            },
            Self::Adult => AgeAttributes {
                sight: 1.0,
                strength: 1.0,
                speed: 1.0,
                jump_cooldown: 0.5,
            },
            Self::Elder => AgeAttributes {
                sight: 0.6,
                strength: 0.5,
                speed: 0.4,
                jump_cooldown: 2.5,
            },
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
    Death { count: u32, last_update: f64 },
}

impl PlayerState {
    pub fn increment_count(&mut self, game_handle: &mut RaylibHandle) -> bool {
        let current_time = game_handle.get_time();
        let mut frame_advanced = false;

        match self {
            PlayerState::Walk { count, last_update } | PlayerState::Jump { count, last_update } => {
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
            PlayerState::Death { count, last_update } => {
                if current_time - *last_update > PLAYER_SPRITE_SPEED && *count < 4 {
                    *count += 1;
                    *last_update = current_time;
                    frame_advanced = true;
                }
            }
            _ => {}
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
    pub current_sight: f32,
    pub target_sight: f32,
    pub last_jump_time: f64,
    pub is_dying: bool,
    pub death_start_time: f64,
    pub spawn_position: (f32, f32),
    pub can_age: bool,
    pub end_scene_active: bool, // For StopAging
    pub end_triggered: bool,    // For End block proximity
}

impl Player {
    pub fn new(
        game_handle: &mut RaylibHandle,
        game_thread: &RaylibThread,
        x: f32,
        y: f32,
    ) -> Result<Self, Box<dyn Error>> {
        let initial_sight = PLAYER_INITIAL_AGE.attributes().sight;
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
            current_sight: initial_sight,
            target_sight: initial_sight,
            last_jump_time: 0.0,
            is_dying: false,
            death_start_time: 0.0,
            spawn_position: (x, y),
            can_age: true,
            end_scene_active: false,
            end_triggered: false,
        })
    }

    pub fn get_sight_multiplier(&self, game_handle: &RaylibHandle) -> f32 {
        if self.is_dying {
            let elapsed = game_handle.get_time() - self.death_start_time;
            let progress = (elapsed / DEATH_ANIMATION_DURATION) as f32;
            self.current_sight * (1.0_f32 - progress.min(1.0))
        } else {
            self.current_sight
        }
    }

    pub fn update_sight(&mut self, delta_time: f32) {
        if self.is_dying {
            return;
        }

        // Faster sight expansion during End scene
        let transition_speed = if self.end_triggered { 0.5 } else { 2.0 };

        let diff = self.target_sight - self.current_sight;
        if diff.abs() > 0.01 {
            self.current_sight += diff * transition_speed * delta_time;
        } else {
            self.current_sight = self.target_sight;
        }
    }

    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D) {
        let (sprite_position, sprite_y) = match &self.state {
            PlayerState::Death { count, .. } => (*count as f32 * SPRITE_SIZE, 5.0 * SPRITE_SIZE),
            PlayerState::Idle => (0.0, self.age.to_value() * SPRITE_SIZE),
            PlayerState::Walk { count, .. } | PlayerState::Jump { count, .. } => (
                *count as f32 * SPRITE_SIZE,
                self.age.to_value() * SPRITE_SIZE,
            ),
        };

        d.draw_texture_pro(
            &self.sprite,
            Rectangle {
                x: sprite_position,
                y: sprite_y,
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
        if !self.can_age {
            return;
        }

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
                Age::Elder => {
                    self.is_dying = true;
                    self.death_start_time = game_handle.get_time();
                    self.vel = (0.0, 0.0);
                    self.state = PlayerState::Death {
                        count: 0,
                        last_update: game_handle.get_time(),
                    };
                    Age::Elder
                }
            };
            if !self.is_dying {
                self.target_sight = self.age.attributes().sight;
            }
        }
    }

    pub fn respawn(&mut self) {
        self.age = Age::Baby;
        self.body.x = self.spawn_position.0;
        self.body.y = self.spawn_position.1;
        self.collision_box.x = self.spawn_position.0 + SPRITE_SIZE / 4.0;
        self.collision_box.y = self.spawn_position.1 + SPRITE_SIZE / 4.0;
        self.collision_box.height = PLAYER_SCALE * Age::Baby.collision_box_height();
        self.vel = (0.0, 0.0);
        self.state = PlayerState::Idle;
        self.grounded = true;
        self.is_dying = false;
        self.current_sight = Age::Baby.attributes().sight;
        self.target_sight = Age::Baby.attributes().sight;
        self.can_age = true;
        self.end_scene_active = false;
        self.end_triggered = false;
    }

    pub fn stop_aging(&mut self) {
        if self.can_age {
            self.can_age = false;
            self.end_scene_active = true; // Only stops aging
            println!("=== AGING STOPPED ===");
        }
    }

    fn trigger_end_scene(&mut self) {
        if !self.end_triggered {
            self.end_triggered = true;
            self.target_sight = END_SCENE_SIGHT_MULTIPLIER;
            println!("=== END SCENE TRIGGERED ===");
        }
    }

    fn check_end_proximity(&mut self, map: &WorldMap) {
        if self.end_triggered {
            return;
        }

        for ((x, y), b) in map {
            if *b == BlockType::End {
                let nx = (*x as f32) * BLOCK_SIZE as f32;
                let ny = (*y as f32) * BLOCK_SIZE as f32;
                let dx = (self.collision_box.x - nx).abs() / BLOCK_SIZE as f32;
                let dy = (self.collision_box.y - ny).abs() / BLOCK_SIZE as f32;

                if dx <= 4.0 && dy <= 4.0 {
                    self.trigger_end_scene();
                    return;
                }
            }
        }
    }

    pub fn after_move(&mut self, game_handle: &mut RaylibHandle, map: &mut WorldMap) -> bool {
        if self.is_dying {
            self.state.increment_count(game_handle);
            let elapsed = game_handle.get_time() - self.death_start_time;
            if elapsed >= DEATH_ANIMATION_DURATION {
                self.respawn();
            }
            return false;
        }

        let mut frame_advanced = false;
        let mut moved = false;

        self.increment_age(game_handle);

        let attrs = self.age.attributes();

        let speed_multiplier = attrs.speed;
        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.vel.0 = PLAYER_SPEED * speed_multiplier;
            self.facing = Facing::Right;
            moved = true;
        } else if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.vel.0 = -PLAYER_SPEED * speed_multiplier;
            self.facing = Facing::Left;
            moved = true;
        } else {
            self.vel.0 = 0.0;
        }

        let jump_multiplier = attrs.strength;
        let current_time = game_handle.get_time();
        let time_since_jump = current_time - self.last_jump_time;

        if (game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE))
            && self.grounded
            && time_since_jump >= attrs.jump_cooldown as f64
        {
            self.vel.1 = -JUMP_SPEED * jump_multiplier;
            self.grounded = false;
            moved = true;
            self.last_jump_time = current_time;
            self.state = PlayerState::Jump {
                count: PLAYER_SPRITE_WALK_INIT,
                last_update: game_handle.get_time(),
            };
        }

        self.vel.1 += GRAVITY;

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

        // Handle special zones
        self.check_stop_aging(map);
        self.check_end_proximity(map);

        if moved {
            match self.state {
                PlayerState::Idle => {
                    self.state = PlayerState::Walk {
                        count: PLAYER_SPRITE_WALK_INIT,
                        last_update: game_handle.get_time(),
                    };
                    frame_advanced = true;
                }
                PlayerState::Walk { .. } | PlayerState::Jump { .. } => {
                    frame_advanced = self.state.increment_count(game_handle);
                }
                _ => {}
            }
        } else if self.grounded {
            self.state = PlayerState::Idle;
        }

        frame_advanced && self.grounded && moved
    }

    fn check_stop_aging(&mut self, map: &WorldMap) {
        for ((x, y), b) in map {
            if *b == BlockType::StopAging {
                let nx = (*x as f32) * BLOCK_SIZE as f32;
                let ny = (*y as f32) * BLOCK_SIZE as f32;

                let block_rect = Rectangle {
                    x: nx,
                    y: ny,
                    width: BLOCK_SIZE as f32,
                    height: BLOCK_SIZE as f32,
                };

                if block_rect.check_collision_recs(&self.collision_box) {
                    self.stop_aging();
                    return;
                }
            }
        }
    }

    pub fn collides(&self, map: &WorldMap) -> Option<Rectangle> {
        for ((x, y), b) in map {
            if *b != BlockType::Blank && *b != BlockType::Start && *b != BlockType::StopAging {
                let nx = (*x as f32) * BLOCK_SIZE as f32;
                let ny = (*y as f32) * BLOCK_SIZE as f32;

                let block_rect = match b {
                    BlockType::End => Rectangle {
                        x: nx,
                        y: ny - DEVIL_HEIGHT + SPRITE_SIZE,
                        width: BLOCK_SIZE as f32,
                        height: DEVIL_HEIGHT,
                    },
                    _ => Rectangle {
                        x: nx,
                        y: ny,
                        width: BLOCK_SIZE as f32,
                        height: BLOCK_SIZE as f32,
                    },
                };

                if block_rect.check_collision_recs(&self.collision_box) {
                    return Some(block_rect);
                }
            }
        }
        None
    }
}
