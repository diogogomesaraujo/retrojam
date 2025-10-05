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
        let mut spawn_x = BASE_WIDTH as f32 / 2.0;
        let mut spawn_y = BASE_HEIGHT as f32 / 2.0;
        for ((x, y), b) in &map {
            match b {
                BlockType::Start => {
                    spawn_x = (*x as f32) * BLOCK_SIZE as f32;
                    spawn_y = (*y as f32) * BLOCK_SIZE as f32;
                    break;
                }
                _ => continue,
            }
        }
        let player = Player::new(game_handle, game_thread, spawn_x, spawn_y)?;
        Ok(Self {
            map,
            player,
            camera: Camera2D {
                offset: Vector2 {
                    x: BASE_WIDTH as f32 / 2.0,
                    y: BASE_HEIGHT as f32 / 2.0,
                },
                target: Vector2 {
                    x: spawn_x + SPRITE_SIZE,
                    y: spawn_y + SPRITE_SIZE,
                },
                rotation: 0.0,
                zoom: CAMERA_ZOOM,
            },
            tileset_texture: game_handle.load_texture(&game_thread, TILESET_PATH)?,
            bg_texture: game_handle.load_texture(&game_thread, BG_PATH)?,
            devil_texture: game_handle.load_texture(&game_thread, DEVIL_PATH)?,
            dust: Dust::new(game_handle, &game_thread)?,
            camera_offset_y: 0.0,
            target_camera_offset_y: 0.0,
        })
    }

    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D, width: &i32, height: &i32) {
        let mut d = d.begin_mode2D(self.camera);
        d.clear_background(BG_COLOR);
        let bg_width = self.bg_texture.width() as f32 / 1.;
        let bg_height = self.bg_texture.height() as f32 / 1.;
        d.draw_texture_ex(
            &self.bg_texture,
            Vector2 {
                x: self.camera.target.x - bg_width / 2.0,
                y: self.camera.target.y - bg_height / 2.0,
            },
            0.0,
            1.,
            Color::new(255, 255, 255, 25),
        );
        for ((x, y), b) in &self.map {
            let nx = (*x as i32) * BLOCK_SIZE;
            let ny = (*y as i32) * BLOCK_SIZE;
            if matches!(
                b,
                BlockType::Start | BlockType::Blank | BlockType::StopAging
            ) {
                continue;
            }
            if matches!(b, BlockType::End) {
                d.draw_texture_rec(
                    &self.devil_texture,
                    Rectangle {
                        x: 0.,
                        y: 0.,
                        width: -SPRITE_SIZE as f32,
                        height: DEVIL_HEIGHT,
                    },
                    Vector2 {
                        x: nx as f32,
                        y: ny as f32 - DEVIL_HEIGHT + SPRITE_SIZE,
                    },
                    Color::WHITE,
                );
                continue;
            }
            let (sprite_x, sprite_y) = b.to_sprite_position();
            d.draw_texture_rec(
                &self.tileset_texture,
                Rectangle {
                    x: (sprite_x * SPRITE_SIZE) as f32,
                    y: (sprite_y * SPRITE_SIZE) as f32,
                    width: SPRITE_SIZE as f32,
                    height: SPRITE_SIZE as f32,
                },
                Vector2 {
                    x: nx as f32,
                    y: ny as f32,
                },
                Color::WHITE,
            );
        }
        self.player.draw(&mut d);
        self.dust.draw(&mut d);
    }

    pub fn update_cam(&mut self) {
        if self.player.end_triggered {
            self.target_camera_offset_y = END_SCENE_CAMERA_OFFSET_Y;
        } else {
            self.target_camera_offset_y = 0.0;
        }

        let diff = self.target_camera_offset_y - self.camera_offset_y;

        // Use a smootherstep curve for natural motion without slowdown lag
        if diff.abs() > 0.05 {
            let t = (diff.abs() / END_SCENE_CAMERA_OFFSET_Y.abs()).clamp(0.0, 1.0);
            let smoothstep = t * t * (3.0 - 2.0 * t); // smoother ease without exponential decay
            self.camera_offset_y +=
                diff * (END_SCENE_CAMERA_TRANSITION_SPEED * 2.0 + smoothstep * 0.05);
        } else {
            self.camera_offset_y = self.target_camera_offset_y;
        }

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
