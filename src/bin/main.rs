use raylib::prelude::*;
use retrojam::shaders::TORCH_FRAGMENT_SHADER;
use retrojam::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(BASE_WIDTH, BASE_HEIGHT)
        .title("RetroJam 2025")
        .resizable()
        .build();
    rl.set_target_fps(TARGET_FPS);

    let audio = RaylibAudio::init_audio_device()?;
    let music = audio.new_music("src/assets/music.mp3")?;
    let walk_sound = audio.new_sound("src/assets/walk.mp3")?;
    let jump_sound = audio.new_sound("src/assets/jump.mp3")?;
    let fall_sound = audio.new_sound("src/assets/fall.mp3")?;
    Sound::set_volume(&walk_sound, 0.1);
    Sound::set_volume(&jump_sound, 0.1);
    Sound::set_volume(&fall_sound, 0.02);
    Music::play_stream(&music);

    let mut shader = rl.load_shader_from_memory(&thread, None, Some(TORCH_FRAGMENT_SHADER));
    let player_pos_loc = shader.get_shader_location("playerPos");
    let resolution_loc = shader.get_shader_location("resolution");
    let light_radius_loc = shader.get_shader_location("lightRadius");
    let light_intensity_loc = shader.get_shader_location("lightIntensity");
    shader.set_shader_value(resolution_loc, [BASE_WIDTH as f32, BASE_HEIGHT as f32]);
    shader.set_shader_value(light_radius_loc, 200.0f32);
    shader.set_shader_value(light_intensity_loc, 0.95f32);

    let mut render_target =
        rl.load_render_texture(&thread, BASE_WIDTH as u32, BASE_HEIGHT as u32)?;

    let mut world = World::new(&mut rl, &thread)?;

    let player_screen_x = BASE_WIDTH as f32 / 2.0;
    let player_screen_y = BASE_HEIGHT as f32 / 2.0;
    shader.set_shader_value(player_pos_loc, [player_screen_x, player_screen_y]);

    let mut was_grounded = true;
    let mut step_counter = 0;

    while !rl.window_should_close() {
        Music::update_stream(&music);

        let jump_input =
            rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_SPACE);
        if was_grounded && !world.player.grounded && jump_input {
            Sound::play(&jump_sound);
        }
        if !was_grounded && world.player.grounded {
            Sound::play(&fall_sound);
        }
        was_grounded = world.player.grounded;

        // Movement
        let footstep_frame = world.player.after_move(&mut rl, &mut world.map);
        if footstep_frame {
            step_counter += 1;
            if step_counter % 2 == 0 {
                Sound::play(&walk_sound);
            }
        }

        world.update_cam();
        {
            let mut texture_mode = rl.begin_texture_mode(&thread, &mut render_target);
            texture_mode.clear_background(Color::BLACK);
            world.draw(&mut texture_mode);
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            let screen_width = d.get_screen_width() as f32;
            let screen_height = d.get_screen_height() as f32;

            let scale_x = screen_width / BASE_WIDTH as f32;
            let scale_y = screen_height / BASE_HEIGHT as f32;
            let scale = scale_x.min(scale_y);

            let scaled_width = BASE_WIDTH as f32 * scale;
            let scaled_height = BASE_HEIGHT as f32 * scale;

            let offset_x = (screen_width - scaled_width) / 2.0;
            let offset_y = (screen_height - scaled_height) / 2.0;

            let mut shader_mode = d.begin_shader_mode(&mut shader);
            shader_mode.draw_texture_pro(
                render_target.texture(),
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: BASE_WIDTH as f32,
                    height: -BASE_HEIGHT as f32,
                },
                Rectangle {
                    x: offset_x,
                    y: offset_y,
                    width: scaled_width,
                    height: scaled_height,
                },
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }
    }

    Ok(())
}
