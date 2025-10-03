use raylib::prelude::*;
use retrojam::{TARGET_FPS, player::Player};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    rl.set_target_fps(TARGET_FPS);
    let mut player = Player::new(&mut rl, &thread)?;

    while !rl.window_should_close() {
        player = player.after_move(&mut rl);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text(&d.get_fps().to_string(), 12, 12, 20, Color::WHITE);

        player.draw(&mut d);
    }

    Ok(())
}
