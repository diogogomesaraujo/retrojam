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

        Ok(Self {
            map,
            player: Player::new(game_handle, game_thread)?,
        })
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for ((x, y), b) in &self.map {
            let nx = (*x as i32) * BLOCK_SIZE;
            let ny = (*y as i32) * BLOCK_SIZE;
            let color = match b {
                BlockType::Blank => Color::LIGHTGRAY,
                BlockType::Stone => Color::DARKGRAY,
            };
            d.draw_rectangle(nx, ny, BLOCK_SIZE, BLOCK_SIZE, color);
        }
        self.player.draw(d);
    }
}
