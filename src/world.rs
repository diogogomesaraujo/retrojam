use crate::{dust::Dust, *};
use raylib::prelude::*;
use std::error::Error;

pub struct World {
    pub map: WorldMap,
    pub player: Player,
    pub camera: Camera2D,
    pub tileset_texture: Texture2D,
    pub devil_texture: Texture2D,
    pub bg_texture: Texture2D,
    pub dust: Dust,
    pub camera_offset_y: f32,
    pub target_camera_offset_y: f32,
}

impl World {
    pub fn new(
        game_handle: &mut RaylibHandle,
        game_thread: &RaylibThread,
    ) -> Result<Self, Box<dyn Error>> {
        let map = load_map();
        let spawn_pos = Self::find_spawn_position(&map);
        let player = Player::new(game_handle, game_thread, spawn_pos.0, spawn_pos.1)?;

        Ok(Self {
            map,
            player,
            camera: Camera2D {
                offset: Vector2 {
                    x: BASE_WIDTH as f32 / 2.0,
                    y: BASE_HEIGHT as f32 / 2.0,
                },
                target: Vector2 {
                    x: spawn_pos.0 + SPRITE_SIZE,
                    y: spawn_pos.1 + SPRITE_SIZE,
                },
                rotation: 0.0,
                zoom: CAMERA_ZOOM,
            },
            tileset_texture: game_handle.load_texture(game_thread, TILESET_PATH)?,
            bg_texture: game_handle.load_texture(game_thread, BG_PATH)?,
            devil_texture: game_handle.load_texture(game_thread, DEVIL_PATH)?,
            dust: Dust::new(game_handle, game_thread)?,
            camera_offset_y: 0.0,
            target_camera_offset_y: 0.0,
        })
    }

    fn find_spawn_position(map: &WorldMap) -> (f32, f32) {
        for ((x, y), block_type) in map {
            if *block_type == BlockType::Start {
                return (
                    (*x as f32) * BLOCK_SIZE as f32,
                    (*y as f32) * BLOCK_SIZE as f32,
                );
            }
        }
        (BASE_WIDTH as f32 / 2.0, BASE_HEIGHT as f32 / 2.0)
    }

    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D, _width: &i32, _height: &i32, _time: &f64) {
        let mut d = d.begin_mode2D(self.camera);
        d.clear_background(BG_COLOR);

        self.draw_background(&mut d);
        self.draw_blocks(&mut d);
        self.player.draw(&mut d);
        self.dust.draw(&mut d);
    }

    fn draw_background<D: RaylibDraw>(&self, d: &mut D) {
        let bg_width = self.bg_texture.width() as f32;
        let bg_height = self.bg_texture.height() as f32;

        d.draw_texture_ex(
            &self.bg_texture,
            Vector2 {
                x: self.camera.target.x - bg_width / 2.0,
                y: self.camera.target.y - bg_height / 2.0,
            },
            0.0,
            1.0,
            Color::new(255, 255, 255, 25),
        );
    }

    fn draw_blocks<D: RaylibDraw>(&self, d: &mut D) {
        for ((x, y), block_type) in &self.map {
            if matches!(
                block_type,
                BlockType::Start | BlockType::Blank | BlockType::StopAging
            ) {
                continue;
            }

            let block_x = (*x as i32) * BLOCK_SIZE;
            let block_y = (*y as i32) * BLOCK_SIZE;

            if *block_type == BlockType::End {
                self.draw_devil(d, block_x as f32, block_y as f32);
            } else {
                self.draw_tile(d, block_type, block_x as f32, block_y as f32);
            }
        }
    }

    fn draw_devil<D: RaylibDraw>(&self, d: &mut D, x: f32, y: f32) {
        d.draw_texture_rec(
            &self.devil_texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: -SPRITE_SIZE,
                height: DEVIL_HEIGHT,
            },
            Vector2 {
                x,
                y: y - DEVIL_HEIGHT + SPRITE_SIZE,
            },
            Color::WHITE,
        );
    }

    fn draw_tile<D: RaylibDraw>(&self, d: &mut D, block_type: &BlockType, x: f32, y: f32) {
        let (sprite_x, sprite_y) = block_type.to_sprite_position();

        d.draw_texture_rec(
            &self.tileset_texture,
            Rectangle {
                x: sprite_x * SPRITE_SIZE,
                y: sprite_y * SPRITE_SIZE,
                width: SPRITE_SIZE,
                height: SPRITE_SIZE,
            },
            Vector2 { x, y },
            Color::WHITE,
        );
    }

    pub fn update_cam(&mut self) {
        self.update_camera_offset();
        self.update_camera_target();
    }

    fn update_camera_offset(&mut self) {
        self.target_camera_offset_y = if self.player.is_end_triggered() {
            END_SCENE_CAMERA_OFFSET_Y
        } else {
            0.0
        };

        let diff = self.target_camera_offset_y - self.camera_offset_y;

        if diff.abs() > 0.05 {
            let t = (diff.abs() / END_SCENE_CAMERA_OFFSET_Y.abs()).clamp(0.0, 1.0);
            let smoothstep = t * t * (3.0 - 2.0 * t);
            self.camera_offset_y +=
                diff * (END_SCENE_CAMERA_TRANSITION_SPEED * 2.0 + smoothstep * 0.05);
        } else {
            self.camera_offset_y = self.target_camera_offset_y;
        }
    }

    fn update_camera_target(&mut self) {
        self.camera.target = Vector2 {
            x: smoothing(
                self.camera.target.x,
                self.player.body.x + SPRITE_SIZE,
                CAMERA_SPEED,
            ),
            y: smoothing(
                self.camera.target.y,
                self.player.body.y + SPRITE_SIZE + self.camera_offset_y,
                CAMERA_SPEED,
            ),
        };
    }
}
