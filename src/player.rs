use raylib::prelude::*;

use crate::{JUMP_SPEED, PLAYER_SPEED};

pub struct Player {
    pub body: Rectangle,
    // pub speed: f32,
    pub can_jump: bool,
}

impl Player {
    pub fn new(game_handle: &mut RaylibHandle) -> Self {
        Self {
            body: Rectangle {
                x: (game_handle.get_screen_width() / 2) as f32,
                y: (game_handle.get_screen_height() / 2) as f32,
                width: 10.,
                height: 10.,
            },
            can_jump: true,
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle_rec(self.body, Color::RED);
    }

    pub fn after_move(self, game_handle: &mut RaylibHandle) -> Self {
        let mut player_after_move = self;

        println!("Player: {:?}", player_after_move.body);

        if game_handle.is_key_down(KeyboardKey::KEY_RIGHT) {
            player_after_move.body.x += PLAYER_SPEED;
        }

        if game_handle.is_key_down(KeyboardKey::KEY_LEFT) {
            player_after_move.body.x -= PLAYER_SPEED;
        }

        if (game_handle.is_key_down(KeyboardKey::KEY_UP)
            || game_handle.is_key_down(KeyboardKey::KEY_SPACE))
            && player_after_move.can_jump
        {
            player_after_move.body.y -= JUMP_SPEED;
        }

        if player_after_move.body.y < 400. {
            player_after_move.body.y += PLAYER_SPEED;
        }

        player_after_move
    }
}
