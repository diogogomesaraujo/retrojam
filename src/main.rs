use raylib::prelude::*;
use retrojam::player::Player;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let mut player = Player::new(&mut rl);

    while !rl.window_should_close() {
        player = player.after_move(&mut rl);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        player.draw(&mut d);
    }
}
