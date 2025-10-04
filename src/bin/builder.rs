use raylib::prelude::*;
use retrojam::*;
use std::collections::HashMap;

type WorldMap = HashMap<(usize, usize), BlockType>;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(BASE_WIDTH, BASE_HEIGHT)
        .title("Map Builder - Click to toggle blocks, X to set start, Z to set end")
        .build();

    rl.set_target_fps(60);

    let tileset = rl
        .load_texture(&thread, TILESET_PATH)
        .expect("Failed to load tileset");

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

        if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                map.retain(|_, bt| *bt != BlockType::End);
                let pos = (grid_x, grid_y);
                map.insert(pos, BlockType::End);
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                let pos = (grid_x, grid_y);
                map.insert(pos, BlockType::Slab);
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_E) {
            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                let pos = (grid_x, grid_y);
                map.insert(pos, BlockType::Blank);
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            save_map(&map);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let pos_x = (x as i32) * BLOCK_SIZE;
                let pos_y = (y as i32) * BLOCK_SIZE;

                match map.get(&(x, y)) {
                    Some(block_type) => {
                        let (sprite_x, sprite_y) = block_type.to_sprite_position();

                        let source = Rectangle::new(
                            sprite_x * SPRITE_SIZE,
                            sprite_y * SPRITE_SIZE,
                            SPRITE_SIZE,
                            SPRITE_SIZE,
                        );

                        let dest = Rectangle::new(
                            pos_x as f32,
                            pos_y as f32,
                            BLOCK_SIZE as f32,
                            BLOCK_SIZE as f32,
                        );

                        d.draw_texture_pro(
                            &tileset,
                            source,
                            dest,
                            Vector2::zero(),
                            0.0,
                            Color::WHITE,
                        );

                        if *block_type == BlockType::Start {
                            d.draw_rectangle_lines(
                                pos_x,
                                pos_y,
                                BLOCK_SIZE,
                                BLOCK_SIZE,
                                Color::GREEN,
                            );
                        }
                    }
                    None => {
                        d.draw_rectangle(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::BLACK);
                    }
                }

                d.draw_rectangle_lines(
                    pos_x,
                    pos_y,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                    Color::new(50, 50, 50, 255),
                );
            }
        }

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            let pos_x = (grid_x as i32) * BLOCK_SIZE;
            let pos_y = (grid_y as i32) * BLOCK_SIZE;
            d.draw_rectangle_lines(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::YELLOW);
        }

        d.draw_text(
            &format!(
                "Left Click: toggle brush ({}x{}) | P: pencil (1x1) | E: eraser (1x1) | X: set start position | Z: set devil postion | S: to save | ESC: to leave",
                DEL_SIZE, DEL_SIZE
            ),
            10,
            BASE_HEIGHT - 20,
            10,
            Color::WHITE,
        );
    }
}
