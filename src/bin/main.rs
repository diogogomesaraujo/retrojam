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

    // === AUDIO ===
    let audio = RaylibAudio::init_audio_device()?;
    let music = audio.new_music("src/assets/music2.mp3")?;
    let ambience = audio.new_music("src/assets/ambience.mp3")?;
    let walk_sound = audio.new_sound("src/assets/walk.mp3")?;
    let jump_sound = audio.new_sound("src/assets/jump.mp3")?;
    let fall_sound = audio.new_sound("src/assets/fall.mp3")?;
    let laugh_sound = audio.new_sound("src/assets/laugh.mp3")?;
    let die_sound = audio.new_sound("src/assets/die.mp3")?;

    Sound::set_volume(&walk_sound, 0.1);
    Sound::set_volume(&jump_sound, 0.1);
    Sound::set_volume(&fall_sound, 0.02);
    Sound::set_volume(&laugh_sound, 0.2);
    Sound::set_volume(&die_sound, 0.4);
    Music::set_volume(&music, 0.8);
    Music::set_volume(&ambience, 0.15);

    Music::play_stream(&music);
    Music::play_stream(&ambience);

    // === SHADER ===
    let mut shader = rl.load_shader_from_memory(&thread, None, Some(TORCH_FRAGMENT_SHADER));
    let player_pos_loc = shader.get_shader_location("playerPos");
    let resolution_loc = shader.get_shader_location("resolution");
    let light_radius_loc = shader.get_shader_location("lightRadius");
    let light_intensity_loc = shader.get_shader_location("lightIntensity");
    shader.set_shader_value(resolution_loc, [BASE_WIDTH as f32, BASE_HEIGHT as f32]);

    let mut render_target =
        rl.load_render_texture(&thread, BASE_WIDTH as u32, BASE_HEIGHT as u32)?;

    let mut world = World::new(&mut rl, &thread)?;

    let player_screen_x = BASE_WIDTH as f32 / 2.0;
    let player_screen_y = BASE_HEIGHT as f32 / 2.0;
    shader.set_shader_value(player_pos_loc, [player_screen_x, player_screen_y]);

    let mut was_grounded = true;
    let mut has_laughed = false;
    let mut step_counter = 0;
    let mut has_played_die_sound = false;

    world.dust.spawn(&mut rl, &world.camera);

    let mut last_rt_width = BASE_WIDTH as u32;
    let mut last_rt_height = BASE_HEIGHT as u32;

    while !rl.window_should_close() {
        // === AUDIO UPDATE ===
        Music::update_stream(&music);
        Music::update_stream(&ambience);

        let delta_time = rl.get_frame_time();

        let jump_input =
            rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_SPACE);
        if was_grounded && !world.player.grounded && jump_input {
            Sound::play(&jump_sound);
        }
        if !was_grounded && world.player.grounded {
            Sound::play(&fall_sound);
        }

        if !has_laughed && world.player.end_scene_active {
            Sound::play(&laugh_sound);
            has_laughed = true;
        }

        was_grounded = world.player.grounded;

        let footstep_frame = world.player.after_move(&mut rl, &mut world.map);
        if footstep_frame {
            step_counter += 1;
            if step_counter % 2 == 0 {
                Sound::play(&walk_sound);
            }
        }

        if world.player.is_dying && !has_played_die_sound {
            Sound::play(&die_sound);
            has_played_die_sound = true;
        }

        if !world.player.is_dying {
            has_played_die_sound = false;
        }

        world.player.update_sight(delta_time);
        let sight = world.player.get_sight_multiplier(&rl);
        shader.set_shader_value(light_radius_loc, 200.0f32 * sight * 1.3);
        shader.set_shader_value(light_intensity_loc, 0.95f32);

        world.update_cam();

        let fade_alpha = if world.player.is_dying {
            let elapsed = rl.get_time() - world.player.death_start_time;
            let progress = (elapsed / DEATH_ANIMATION_DURATION).clamp(0.0, 1.0);
            let eased = progress * progress * (3.0 - 2.0 * progress);
            (eased * 255.0) as u8
        } else {
            0
        };

        world.dust.update(&mut rl);

        let screen_width = rl.get_screen_width() as f32;
        let screen_height = rl.get_screen_height() as f32;
        let time = rl.get_time();

        {
            let current_width = rl.get_screen_width() as u32 * 4;
            let current_height = rl.get_screen_height() as u32 * 4;

            if current_width != last_rt_width || current_height != last_rt_height {
                render_target =
                    rl.load_render_texture(&thread, BASE_WIDTH as u32, BASE_HEIGHT as u32)?;
                last_rt_width = current_width;
                last_rt_height = current_height;
            }

            let mut texture_mode = rl.begin_texture_mode(&thread, &mut render_target);
            texture_mode.clear_background(Color::BLACK);
            world.draw(&mut texture_mode, &time);
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            let scale_x = screen_width / BASE_WIDTH as f32;
            let scale_y = screen_height / BASE_HEIGHT as f32;
            let scale = scale_x.min(scale_y);
            let scaled_width = BASE_WIDTH as f32 * scale;
            let scaled_height = BASE_HEIGHT as f32 * scale;
            let offset_x = (screen_width - scaled_width) / 2.0;
            let offset_y = (screen_height - scaled_height) / 2.0;

            {
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

            if world.end_triggered_world == true {
                match time - world.end_scene_start {
                    x if x < DIALOGUE_1_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_1_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_2_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_2_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_3_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_3_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_4_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_4_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_5_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_5_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_6_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_6_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_7_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_7_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_8_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_8_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_9_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_9_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_10_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_10_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_11_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_11_TEXT, &world.tiny_font);
                    }
                    x if x < DIALOGUE_12_TIME => {
                        world
                            .player
                            .draw_text(&mut d, DIALOGUE_12_TEXT, &world.tiny_font);
                    }
                    _ => {}
                }
            };

            if fade_alpha > 0 {
                d.draw_rectangle(
                    0,
                    0,
                    d.get_screen_width(),
                    d.get_screen_height(),
                    Color::new(0, 0, 0, fade_alpha),
                );
            }
        }
    }

    Ok(())
}
