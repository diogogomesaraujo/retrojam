use crate::*;
use raylib::prelude::*;
use std::error::Error;

pub struct AgeAttributes {
    pub sight: f32,
    pub strength: f32,
    pub speed: f32,
    pub jump_cooldown: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn time_to_change(&self) -> f64 {
        match self {
            Self::Baby => BABY_TIME_TO_CHANGE,
            Self::Child => CHILD_TIME_TO_CHANGE,
            Self::Teenager => TEENAGER_TIME_TO_CHANGE,
            Self::Adult => ADULT_TIME_TO_CHANGE,
            Self::Elder => ELDER_TIME_TO_CHANGE,
        }
    }

    pub fn next_age(&self) -> Option<Age> {
        match self {
            Self::Baby => Some(Age::Child),
            Self::Child => Some(Age::Teenager),
            Self::Teenager => Some(Age::Adult),
            Self::Adult => Some(Age::Elder),
            Self::Elder => None,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn to_value(&self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
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
                if current_time - *last_update > PLAYER_SPRITE_SPEED
                    && *count < PLAYER_SPRITE_DYING_END
                {
                    *count += 1;
                    *last_update = current_time;
                    frame_advanced = true;
                }
            }
            _ => {}
        }
        frame_advanced
    }

    pub fn is_moving(&self) -> bool {
        matches!(self, Self::Walk { .. } | Self::Jump { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EndSceneState {
    None,
    AgingStopped,
    EndTriggered,
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
    pub end_scene_state: EndSceneState,
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
                x: x + COLLISION_BOX_OFFSET_X,
                y: y + COLLISION_BOX_OFFSET_Y,
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
            end_scene_state: EndSceneState::None,
        })
    }

    pub fn can_age(&self) -> bool {
        self.end_scene_state == EndSceneState::None
    }

    pub fn is_end_triggered(&self) -> bool {
        self.end_scene_state == EndSceneState::EndTriggered
    }

    pub fn get_sight_multiplier(&self, game_handle: &RaylibHandle) -> f32 {
        if self.is_dying {
            let elapsed = game_handle.get_time() - self.death_start_time;
            let progress = (elapsed / DEATH_ANIMATION_DURATION) as f32;
            self.current_sight * (1.0 - progress.min(1.0))
        } else {
            self.current_sight
        }
    }

    pub fn update_sight(&mut self, delta_time: f32) {
        if self.is_dying {
            return;
        }

        let transition_speed = if self.is_end_triggered() {
            SIGHT_TRANSITION_SPEED_END
        } else {
            SIGHT_TRANSITION_SPEED_NORMAL
        };

        let diff = self.target_sight - self.current_sight;
        if diff.abs() > 0.01 {
            self.current_sight += diff * transition_speed * delta_time;
        } else {
            self.current_sight = self.target_sight;
        }
    }

    fn update_collision_box_position(&mut self) {
        self.collision_box.x = self.body.x + COLLISION_BOX_OFFSET_X;
        self.collision_box.y = self.body.y + COLLISION_BOX_OFFSET_Y;
    }

    fn get_sprite_coords(&self) -> (f32, f32) {
        match &self.state {
            PlayerState::Death { count, .. } => {
                (*count as f32 * SPRITE_SIZE, DEATH_SPRITE_ROW * SPRITE_SIZE)
            }
            PlayerState::Idle => (0.0, self.age.to_value() * SPRITE_SIZE),
            PlayerState::Walk { count, .. } | PlayerState::Jump { count, .. } => (
                *count as f32 * SPRITE_SIZE,
                self.age.to_value() * SPRITE_SIZE,
            ),
        }
    }

    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D) {
        let (sprite_x, sprite_y) = self.get_sprite_coords();

        d.draw_texture_pro(
            &self.sprite,
            Rectangle {
                x: sprite_x,
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

    pub fn update(&mut self, game_handle: &mut RaylibHandle, map: &WorldMap) -> bool {
        if self.is_dying {
            return self.update_death_animation(game_handle);
        }

        self.update_aging(game_handle);
        let moved = self.handle_input(game_handle);
        self.apply_physics(map);
        self.check_special_zones(map);
        self.update_animation_state(game_handle, moved)
    }

    fn update_death_animation(&mut self, game_handle: &mut RaylibHandle) -> bool {
        self.state.increment_count(game_handle);
        let elapsed = game_handle.get_time() - self.death_start_time;
        if elapsed >= DEATH_ANIMATION_DURATION {
            self.respawn();
        }
        false
    }

    fn update_aging(&mut self, game_handle: &mut RaylibHandle) {
        if !self.can_age() {
            return;
        }

        let time_in_life = game_handle.get_time() as u32 % LIFETIME as u32;
        let time_to_change = self.age.time_to_change() as u32;

        if time_to_change == time_in_life {
            if let Some(next_age) = self.age.next_age() {
                self.age = next_age;
                self.set_target_sight(self.age.attributes().sight);
            } else {
                self.start_dying(game_handle);
            }
        }
    }

    fn start_dying(&mut self, game_handle: &mut RaylibHandle) {
        self.is_dying = true;
        self.death_start_time = game_handle.get_time();
        self.vel = (0.0, 0.0);
        self.state = PlayerState::Death {
            count: 0,
            last_update: game_handle.get_time(),
        };
    }

    fn handle_input(&mut self, game_handle: &mut RaylibHandle) -> bool {
        let attrs = self.age.attributes();
        let mut moved = false;

        // Horizontal movement
        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.vel.0 = PLAYER_SPEED * attrs.speed;
            self.facing = Facing::Right;
            moved = true;
        } else if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            self.vel.0 = -PLAYER_SPEED * attrs.speed;
            self.facing = Facing::Left;
            moved = true;
        } else {
            self.vel.0 = 0.0;
        }

        // Jump
        let current_time = game_handle.get_time();
        let time_since_jump = current_time - self.last_jump_time;
        let jump_pressed = game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE);

        if jump_pressed && self.grounded && time_since_jump >= attrs.jump_cooldown as f64 {
            self.vel.1 = -JUMP_SPEED * attrs.strength;
            self.grounded = false;
            self.last_jump_time = current_time;
            self.state = PlayerState::Jump {
                count: PLAYER_SPRITE_WALK_INIT,
                last_update: current_time,
            };
            moved = true;
        }

        moved
    }

    fn apply_physics(&mut self, map: &WorldMap) {
        // Apply gravity
        self.vel.1 += GRAVITY;

        // Horizontal collision
        self.body.x += self.vel.0;
        self.collision_box.x += self.vel.0;

        if let Some(block) = self.collides(map) {
            if self.vel.0 > 0.0 {
                self.body.x = block.x - self.collision_box.width - COLLISION_BOX_OFFSET_X;
            } else if self.vel.0 < 0.0 {
                self.body.x = block.x + block.width - COLLISION_BOX_OFFSET_X;
            }
            self.update_collision_box_position();
            self.vel.0 = 0.0;
        }

        // Vertical collision
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
    }

    fn check_special_zones(&mut self, map: &WorldMap) {
        for ((x, y), block_type) in map {
            if !block_type.is_special_zone() {
                continue;
            }

            let block_x = (*x as f32) * BLOCK_SIZE as f32;
            let block_y = (*y as f32) * BLOCK_SIZE as f32;

            match block_type {
                BlockType::StopAging => {
                    let block_rect = Rectangle {
                        x: block_x,
                        y: block_y,
                        width: BLOCK_SIZE as f32,
                        height: BLOCK_SIZE as f32,
                    };

                    if block_rect.check_collision_recs(&self.collision_box) {
                        self.stop_aging();
                        return;
                    }
                }
                BlockType::End => {
                    let dx = (self.collision_box.x - block_x).abs() / BLOCK_SIZE as f32;
                    let dy = (self.collision_box.y - block_y).abs() / BLOCK_SIZE as f32;

                    if dx <= END_BLOCK_PROXIMITY_THRESHOLD && dy <= END_BLOCK_PROXIMITY_THRESHOLD {
                        self.trigger_end_scene();
                        return;
                    }
                }
                _ => {}
            }
        }
    }

    fn update_animation_state(&mut self, game_handle: &mut RaylibHandle, moved: bool) -> bool {
        if moved {
            match self.state {
                PlayerState::Idle => {
                    self.state = PlayerState::Walk {
                        count: PLAYER_SPRITE_WALK_INIT,
                        last_update: game_handle.get_time(),
                    };
                    true
                }
                PlayerState::Walk { .. } | PlayerState::Jump { .. } => {
                    self.state.increment_count(game_handle)
                }
                _ => false,
            }
        } else if self.grounded {
            self.state = PlayerState::Idle;
            false
        } else {
            false
        }
    }

    fn stop_aging(&mut self) {
        if self.end_scene_state == EndSceneState::None {
            self.end_scene_state = EndSceneState::AgingStopped;
            println!("=== AGING STOPPED ===");
        }
    }

    fn trigger_end_scene(&mut self) {
        if self.end_scene_state != EndSceneState::EndTriggered {
            self.end_scene_state = EndSceneState::EndTriggered;
            self.set_target_sight(END_SCENE_SIGHT_MULTIPLIER);
            println!("=== END SCENE TRIGGERED ===");
        }
    }

    fn set_target_sight(&mut self, sight: f32) {
        self.target_sight = sight;
    }

    fn respawn(&mut self) {
        self.age = Age::Baby;
        self.body.x = self.spawn_position.0;
        self.body.y = self.spawn_position.1;
        self.collision_box.height = PLAYER_SCALE * Age::Baby.collision_box_height();
        self.update_collision_box_position();
        self.vel = (0.0, 0.0);
        self.state = PlayerState::Idle;
        self.grounded = true;
        self.is_dying = false;
        self.current_sight = Age::Baby.attributes().sight;
        self.target_sight = Age::Baby.attributes().sight;
        self.end_scene_state = EndSceneState::None;
    }

    pub fn collides(&self, map: &WorldMap) -> Option<Rectangle> {
        for ((x, y), block_type) in map {
            if !block_type.is_collidable() {
                continue;
            }

            let block_x = (*x as f32) * BLOCK_SIZE as f32;
            let block_y = (*y as f32) * BLOCK_SIZE as f32;

            let block_rect = match block_type {
                BlockType::End => Rectangle {
                    x: block_x,
                    y: block_y - DEVIL_HEIGHT + SPRITE_SIZE,
                    width: BLOCK_SIZE as f32,
                    height: DEVIL_HEIGHT,
                },
                _ => Rectangle {
                    x: block_x,
                    y: block_y,
                    width: BLOCK_SIZE as f32,
                    height: BLOCK_SIZE as f32,
                },
            };

            if block_rect.check_collision_recs(&self.collision_box) {
                return Some(block_rect);
            }
        }
        None
    }
}
