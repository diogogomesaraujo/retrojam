use std::error::Error;

use raylib::prelude::*;
use retrojam::{
    BLOCK_SIZE, BlockType, DEL_SIZE, GRID_HEIGHT, GRID_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    TARGET_FPS, load_map, player::Player, world::World,
};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(TARGET_FPS);
    let mut world = World::new(&mut rl, &thread)?;

    while !rl.window_should_close() {
        world.player.after_move(&mut rl, &mut world.map);
        world.update_cam();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        world.draw(&mut d);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        if world.player.collides(&mut world.map).is_some() {
            d.draw_text("Collision!", 12, 36, 20, Color::WHITE);
        }
    }

    Ok(())
}
