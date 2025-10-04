use raylib::prelude::*;
use retrojam::shaders::TORCH_FRAGMENT_SHADER;
use retrojam::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("RetroJam 2025")
        .build();
    rl.set_target_fps(TARGET_FPS);

    // === AUDIO ===
    let audio = RaylibAudio::init_audio_device()?;
    let music = audio.new_music("src/assets/music.mp3")?;
    let walk_sound = audio.new_sound("src/assets/walk.mp3")?;
    let jump_sound = audio.new_sound("src/assets/jump.mp3")?;
    let fall_sound = audio.new_sound("src/assets/fall.mp3")?;

    Sound::set_volume(&walk_sound, 0.1);
    Sound::set_volume(&jump_sound, 0.1);
    Sound::set_volume(&fall_sound, 0.02);
    Music::play_stream(&music);

    // === SHADER ===
    let mut shader = rl.load_shader_from_memory(&thread, None, Some(TORCH_FRAGMENT_SHADER));
    let player_pos_loc = shader.get_shader_location("playerPos");
    let resolution_loc = shader.get_shader_location("resolution");
    let light_radius_loc = shader.get_shader_location("lightRadius");
    let light_intensity_loc = shader.get_shader_location("lightIntensity");
    shader.set_shader_value(resolution_loc, [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32]);
    shader.set_shader_value(light_radius_loc, 200.0f32);
    shader.set_shader_value(light_intensity_loc, 0.95f32);

    let mut render_target =
        rl.load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)?;
    let mut world = World::new(&mut rl, &thread)?;

    let player_screen_x = SCREEN_WIDTH as f32 / 2.0;
    let player_screen_y = SCREEN_HEIGHT as f32 / 2.0;
    shader.set_shader_value(player_pos_loc, [player_screen_x, player_screen_y]);

    // === STATE FLAGS ===
    let mut was_grounded = true;
    let mut step_counter = 0;

    while !rl.window_should_close() {
        Music::update_stream(&music);

        // Jump / Fall
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
            // Play once every 3 switches
            if step_counter % 2 == 0 {
                Sound::play(&walk_sound);
            }
        }

        world.update_cam();

        // === RENDER TO TEXTURE ===
        {
            let mut texture_mode = rl.begin_texture_mode(&thread, &mut render_target);
            texture_mode.clear_background(Color::BLACK);
            world.draw(&mut texture_mode);
        }

        // === DRAW FINAL ===
        {
            let mut d = rl.begin_drawing(&thread);
            let mut shader_mode = d.begin_shader_mode(&mut shader);
            shader_mode.draw_texture_rec(
                render_target.texture(),
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: SCREEN_WIDTH as f32,
                    height: -SCREEN_HEIGHT as f32,
                },
                Vector2::zero(),
                Color::WHITE,
            );
        }
    }

    Ok(())
}
