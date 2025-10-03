use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 52;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 416;
const BLOCK_SIZE: i32 = SCREEN_WIDTH / GRID_WIDTH as i32;
const DEL_SIZE: i32 = 3;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum BlockType {
    Blank,
    Stone,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Block {
    x: usize,
    y: usize,
    block_type: BlockType,
}

#[derive(Serialize, Deserialize)]
struct MapData {
    width: usize,
    height: usize,
    blocks: Vec<Block>,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Map Builder - Click to toggle blocks")
        .build();

    rl.set_target_fps(60);

    // Internal representation
    let mut grid: Vec<Vec<Option<BlockType>>> = vec![vec![None; GRID_WIDTH]; GRID_HEIGHT];

    // Load existing map if available
    if let Ok(content) = fs::read_to_string("map.json") {
        if let Ok(map_data) = serde_json::from_str::<MapData>(&content) {
            if map_data.height == GRID_HEIGHT && map_data.width == GRID_WIDTH {
                for block in map_data.blocks {
                    if block.x < GRID_WIDTH && block.y < GRID_HEIGHT {
                        grid[block.y][block.x] = Some(block.block_type);
                    }
                }
                println!("Loaded existing map from map.json");
            }
        }
    }

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            let grid_x = (mouse_pos.x as i32 / BLOCK_SIZE) as usize;
            let grid_y = (mouse_pos.y as i32 / BLOCK_SIZE) as usize;

            if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
                if let Some(BlockType::Blank) = grid[grid_y][grid_x] {
                    // Deactivate single blank
                    grid[grid_y][grid_x] = None;
                } else {
                    // Activate brush area with blanks (remove stone if present)
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
                                grid[ny as usize][nx as usize] = Some(BlockType::Blank);
                            }
                        }
                    }
                }
            }

            // recompute stone borders
            recompute_stone_borders(&mut grid);
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let pos_x = (x as i32) * BLOCK_SIZE;
                let pos_y = (y as i32) * BLOCK_SIZE;

                match grid[y][x] {
                    Some(BlockType::Blank) => {
                        d.draw_rectangle(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::WHITE);
                    }
                    Some(BlockType::Stone) => {
                        d.draw_rectangle(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::BROWN);
                    }
                    None => {
                        d.draw_rectangle(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::BLACK);
                    }
                }

                d.draw_rectangle_lines(pos_x, pos_y, BLOCK_SIZE, BLOCK_SIZE, Color::BLACK);
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

    // Save only active blocks
    let mut blocks = Vec::new();
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let Some(block_type) = grid[y][x] {
                blocks.push(Block { x, y, block_type });
            }
        }
    }

    let map_data = MapData {
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
        blocks,
    };

    match serde_json::to_string_pretty(&map_data) {
        Ok(json) => {
            if let Err(e) = fs::write("map.json", json) {
                eprintln!("Failed to save map: {}", e);
            } else {
                println!("Map saved to map.json");
            }
        }
        Err(e) => eprintln!("Failed to serialize map: {}", e),
    }
}

/// recompute stone borders around all blanks
fn recompute_stone_borders(grid: &mut Vec<Vec<Option<BlockType>>>) {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] == Some(BlockType::Stone) {
                grid[y][x] = None;
            }
        }
    }

    // add new stone borders
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if grid[y][x] == Some(BlockType::Blank) {
                for (dx, dy) in dirs {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 && nx < GRID_WIDTH as i32 && ny < GRID_HEIGHT as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if grid[ny][nx].is_none() {
                            grid[ny][nx] = Some(BlockType::Stone);
                        }
                    }
                }
            }
        }
    }
}
