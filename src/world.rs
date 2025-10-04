use std::error::Error;

use crate::*;
use raylib::prelude::*;

pub struct World {
    pub map: WorldMap,
    pub player: Player,
    pub camera: Camera2D,
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
                    x: SCREEN_WIDTH as f32 / 2.,
                    y: SCREEN_HEIGHT as f32 / 2.,
                },
                target: Vector2 {
                    x: spawn_x + SPRITE_SIZE,
                    y: spawn_y + SPRITE_SIZE,
                },
                rotation: 0.,
                zoom: CAMERA_ZOOM,
            },
        })
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let mut d = d.begin_mode2D(self.camera);
        for ((x, y), b) in &self.map {
            let nx = (*x as i32) * BLOCK_SIZE;
            let ny = (*y as i32) * BLOCK_SIZE;
            let color = match b {
                BlockType::Blank => Color::LIGHTGRAY,
                BlockType::Start => Color::YELLOW,

                BlockType::StoneLeftUp => Color::RED,
                BlockType::StoneLeftDown => Color::ORANGE,
                BlockType::StoneRightUp => Color::BLUE,
                BlockType::StoneRightDown => Color::PURPLE,

                BlockType::StoneSlabLeft => Color::DARKBLUE,
                BlockType::StoneSlabRight => Color::DARKPURPLE,
                BlockType::StoneSlabUp => Color::DARKGREEN,
                BlockType::StoneSlabDown => Color::BROWN,
            };
            d.draw_rectangle(nx, ny, BLOCK_SIZE, BLOCK_SIZE, color);
        }
        self.player.draw(&mut d);
    }

    pub fn update_cam(&mut self) {
        self.camera = Camera2D {
            offset: Vector2 {
                x: SCREEN_WIDTH as f32 / 2.,
                y: SCREEN_HEIGHT as f32 / 2.,
            },
            target: Vector2 {
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
            },
            rotation: 0.,
            zoom: CAMERA_ZOOM,
        };
    }
}
