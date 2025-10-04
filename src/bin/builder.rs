use raylib::prelude::*;
use retrojam::*;
use std::collections::HashMap;

type WorldMap = HashMap<(usize, usize), BlockType>;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Map Builder - Click to toggle blocks, X to set start")
        .build();
    rl.set_target_fps(60);

    let mut map = load_map();

    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        let grid_x = (mouse_pos.x as i32 / BLOCK_SIZE) as usize;
        let grid_y = (mouse_pos.y as i32 / BLOCK_SIZE) as usize;

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
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

        if rl.is_key_pressed(KeyboardKey::KEY_X) {
            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                map.retain(|_, bt| *bt != BlockType::Start);
                let pos = (grid_x, grid_y);
                map.insert(pos, BlockType::Start);
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let pos_x = (x as i32) * BLOCK_SIZE;
                let pos_y = (y as i32) * BLOCK_SIZE;

                let color = match map.get(&(x, y)) {
                    Some(BlockType::Blank) => Color::WHITE,
                    Some(BlockType::Start) => Color::GREEN,

                    Some(BlockType::StoneSlabUp) => Color::RED,
                    Some(BlockType::StoneSlabDown) => Color::MAROON,
                    Some(BlockType::StoneSlabLeft) => Color::BLUE,
                    Some(BlockType::StoneSlabRight) => Color::DARKBLUE,

                    Some(BlockType::StoneRightUp) => Color::PURPLE,
                    Some(BlockType::StoneRightDown) => Color::VIOLET,
                    Some(BlockType::StoneLeftUp) => Color::ORANGE,
                    Some(BlockType::StoneLeftDown) => Color::GOLD,

                    None => Color::BLACK,
                };

                d.draw_rectangle(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, color);
                d.draw_rectangle_lines(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::DARKGRAY);
            }
        }

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            let pos_x = (grid_x as i32) * BLOCK_SIZE;
            let pos_y = (grid_y as i32) * BLOCK_SIZE;
            d.draw_rectangle_lines(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::YELLOW);
        }

        d.draw_text(
            &format!(
                "Left Click: toggle brush ({}x{}) | X: set start position | Close to save",
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
