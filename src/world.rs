use crate::*;
use raylib::prelude::*;
use std::error::Error;

pub struct World {
    pub map: WorldMap,
    pub player: Player,
    pub camera: Camera2D,
    pub tileset_texture: Texture2D,
}

impl World {
    pub fn new(
        game_handle: &mut RaylibHandle,
        game_thread: &RaylibThread,
    ) -> Result<Self, Box<dyn Error>> {
        let map = load_map();
        let mut spawn_x = (game_handle.get_screen_width() / 2) as f32;
        let mut spawn_y = (game_handle.get_screen_height() / 2) as f32;

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
                    x: SCREEN_WIDTH as f32 / 2.0,
                    y: SCREEN_HEIGHT as f32 / 2.0,
                },
                target: Vector2 {
                    x: spawn_x + SPRITE_SIZE,
                    y: spawn_y + SPRITE_SIZE,
                },
                rotation: 0.0,
                zoom: CAMERA_ZOOM,
            },
            tileset_texture: game_handle.load_texture(&game_thread, TILESET_PATH)?,
        })
    }

    /// Generic draw method that accepts any RaylibDraw context
    pub fn draw<D: RaylibDraw>(&mut self, d: &mut D) {
        // Apply camera transformation
        let mut d = d.begin_mode2D(self.camera);
        d.clear_background(BG_COLOR);
        for ((x, y), b) in &self.map {
            let nx = (*x as i32) * BLOCK_SIZE;
            let ny = (*y as i32) * BLOCK_SIZE;
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

        // Draw player
        self.player.draw(&mut d);
    }

    /// Update camera to smoothly follow player
    pub fn update_cam(&mut self) {
        self.camera.target = Vector2 {
            x: smoothing(
                self.camera.target.x,
                self.player.body.x + SPRITE_SIZE,
                CAMERA_SPEED,
            ),
            y: smoothing(
                self.camera.target.y,
                self.player.body.y + SPRITE_SIZE,
                CAMERA_SPEED,
            ),
        };
    }
}
