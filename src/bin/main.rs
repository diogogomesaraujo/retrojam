use raylib::prelude::*;
use retrojam::{
    BLOCK_SIZE, BlockType, DEL_SIZE, GRID_HEIGHT, GRID_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
    TARGET_FPS, load_map, player::Player, world::World,
};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(TARGET_FPS);
    let mut world = World::new(&mut rl);

    while !rl.window_should_close() {
        world.player = world.player.after_move(&mut rl);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        world.draw(&mut d);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
