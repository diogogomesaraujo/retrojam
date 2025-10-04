use std::error::Error;

use raylib::prelude::*;
use retrojam::*;

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

        world.draw(&mut d);
    }

    Ok(())
}
