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

    let audio = RaylibAudio::init_audio_device()?;

    let music = audio.new_music("src/assets/music.mp3")?;
    let walk_sound = audio.new_sound("src/assets/walk.mp3")?;
    Sound::set_volume(&walk_sound, 0.5);

    Music::play_stream(&music);

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

    let mut walk_sound_playing = false;
    let mut fade_out_timer = 0.0f32;
    let mut is_fading = false;
    let fade_duration = 0.3f32;

    while !rl.window_should_close() {
        Music::update_stream(&music);

        let delta = rl.get_frame_time();

        let is_walking =
            rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_LEFT);

        if is_walking {
            is_fading = false;
            fade_out_timer = 0.0;
            Sound::set_volume(&walk_sound, 0.5);

            if !walk_sound_playing {
                Sound::play(&walk_sound);
                walk_sound_playing = true;
            }

            if !Sound::is_playing(&walk_sound) {
                Sound::play(&walk_sound);
            }
        } else {
            if walk_sound_playing && !is_fading {
                is_fading = true;
                fade_out_timer = 0.0;
            }

            if is_fading {
                fade_out_timer += delta;
                let progress = (fade_out_timer / fade_duration).min(1.0);
                let volume = 0.5 * (1.0 - progress);
                Sound::set_volume(&walk_sound, volume);

                if progress >= 1.0 {
                    Sound::stop(&walk_sound);
                    walk_sound_playing = false;
                    is_fading = false;
                }
            }
        }

        world.player.after_move(&mut rl, &mut world.map);
        world.update_cam();

        {
            let mut texture_mode = rl.begin_texture_mode(&thread, &mut render_target);
            texture_mode.clear_background(Color::BLACK);
            world.draw(&mut texture_mode);
        }

        {
            let mut d = rl.begin_drawing(&thread);
            {
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
    }

    Ok(())
}
