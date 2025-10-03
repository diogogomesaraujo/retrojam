use crate::{BLOCK_SIZE, BlockType, WorldMap, load_map, player::Player};
use raylib::prelude::*;

pub struct World {
    pub map: WorldMap,
    pub player: Player,
}

impl World {
    pub fn new(_handle: &mut RaylibHandle) -> Self {
        let map = load_map();

        Self {
            map,
            player: Player {
                body: Rectangle::new(50.0, 50.0, BLOCK_SIZE as f32, BLOCK_SIZE as f32),
                can_jump: true,
            },
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for ((x, y), b) in &self.map {
            let nx = (*x as i32) * BLOCK_SIZE;
            let ny = (*y as i32) * BLOCK_SIZE;
            let color = match b {
                BlockType::Blank => Color::LIGHTGRAY,
                BlockType::Stone => Color::DARKGRAY,
            };
            d.draw_rectangle(nx, ny, BLOCK_SIZE, BLOCK_SIZE, color);
        }
        d.draw_rectangle_rec(self.player.body, Color::RED);
    }
}
