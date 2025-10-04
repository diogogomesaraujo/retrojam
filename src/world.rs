use std::error::Error;

use crate::{BLOCK_SIZE, BlockType, WorldMap, load_map, player::Player};
use raylib::prelude::*;

pub struct World {
    pub map: WorldMap,
    pub player: Player,
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
                BlockType::Stone => continue,
                BlockType::Blank => continue,
            }
        }

        Ok(Self {
            map,
            player: Player::new(game_handle, game_thread, spawn_x, spawn_y)?,
        })
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for ((x, y), b) in &self.map {
            let nx = (*x as i32) * BLOCK_SIZE;
            let ny = (*y as i32) * BLOCK_SIZE;
            let color = match b {
                BlockType::Blank => Color::LIGHTGRAY,
                BlockType::Stone => Color::DARKGRAY,
                BlockType::Start => Color::YELLOW,
            };
            d.draw_rectangle(nx, ny, BLOCK_SIZE, BLOCK_SIZE, color);
        }
        self.player.draw(d);
    }
}
