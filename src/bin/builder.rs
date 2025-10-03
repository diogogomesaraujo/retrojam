use raylib::prelude::*;
use std::collections::HashMap;

use retrojam::{
    BLOCK_SIZE, BlockType, DEL_SIZE, GRID_HEIGHT, GRID_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    load_map, recompute_stone_borders, save_map,
};

type WorldMap = HashMap<(usize, usize), BlockType>;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Map Builder - Click to toggle blocks")
        .build();

    rl.set_target_fps(60);

    let mut map = load_map();

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let grid_x = (mouse_pos.x as i32 / BLOCK_SIZE) as usize;
            let grid_y = (mouse_pos.y as i32 / BLOCK_SIZE) as usize;

            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                let pos = (grid_x, grid_y);

                if map.get(&pos) == Some(&BlockType::Blank) {
                    map.remove(&pos);
                } else {
                    let half = DEL_SIZE / 2;
                    for dy in -half..=half {
                        for dx in -half..=half {
                            let nx = grid_x as i32 + dx;
                            let ny = grid_y as i32 + dy;
                            if nx >= 0
                                && ny >= 0
                                && nx < GRID_WIDTH as i32
                                && ny < GRID_HEIGHT as i32
                            {
                                map.insert((nx as usize, ny as usize), BlockType::Blank);
                            }
                        }
                    }
                }
            }

            recompute_stone_borders(&mut map);
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        // Draw grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let pos_x = (x as i32) * BLOCK_SIZE;
                let pos_y = (y as i32) * BLOCK_SIZE;

                let color = match map.get(&(x, y)) {
                    Some(BlockType::Blank) => Color::WHITE,
                    Some(BlockType::Stone) => Color::BROWN,
                    None => Color::BLACK,
                };

                d.draw_rectangle(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, color);
                d.draw_rectangle_lines(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::DARKGRAY);
            }
        }

        d.draw_text(
            &format!(
                "Click to toggle (brush {}x{}) - Close to save",
                DEL_SIZE, DEL_SIZE
            ),
            10,
            SCREEN_HEIGHT - 20,
            10,
            Color::WHITE,
        );
    }
    save_map(&map);
}
