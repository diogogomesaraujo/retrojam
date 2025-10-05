use raylib::prelude::*;
use retrojam::shaders::TORCH_FRAGMENT_SHADER;
use retrojam::*;
use std::error::Error;

struct AudioSystem<'a> {
    music: Music<'a>,
    ambience: Music<'a>,
    walk_sound: Sound<'a>,
    jump_sound: Sound<'a>,
    fall_sound: Sound<'a>,
    laugh_sound: Sound<'a>,
    die_sound: Sound<'a>,
}

impl<'a> AudioSystem<'a> {
    fn new(audio: &'a RaylibAudio) -> Result<Self, Box<dyn Error>> {
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

        Ok(Self {
            music,
            ambience,
            walk_sound,
            jump_sound,
            fall_sound,
            laugh_sound,
            die_sound,
        })
    }

    fn start(&self) {
        Music::play_stream(&self.music);
        Music::play_stream(&self.ambience);
    }

    fn update(&self) {
        Music::update_stream(&self.music);
        Music::update_stream(&self.ambience);
    }
}

struct ShaderSystem {
    shader: Shader,
    player_pos_loc: i32,
    resolution_loc: i32,
    light_radius_loc: i32,
    light_intensity_loc: i32,
}
impl ShaderSystem {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut shader = rl.load_shader_from_memory(thread, None, Some(TORCH_FRAGMENT_SHADER));
        let player_pos_loc = shader.get_shader_location("playerPos");
        let resolution_loc = shader.get_shader_location("resolution");
        let light_radius_loc = shader.get_shader_location("lightRadius");
        let light_intensity_loc = shader.get_shader_location("lightIntensity");

        shader.set_shader_value(resolution_loc, [BASE_WIDTH as f32, BASE_HEIGHT as f32]);

        let player_screen_x = BASE_WIDTH as f32 / 2.0;
        let player_screen_y = BASE_HEIGHT as f32 / 2.0;
        shader.set_shader_value(player_pos_loc, [player_screen_x, player_screen_y]);

        Self {
            shader,
            player_pos_loc,
            resolution_loc,
            light_radius_loc,
            light_intensity_loc,
        }
    }

    fn update_light(&mut self, sight_multiplier: f32) {
        self.shader
            .set_shader_value(self.light_radius_loc, 200.0f32 * sight_multiplier * 1.3);
        self.shader
            .set_shader_value(self.light_intensity_loc, 0.95f32);
    }

    fn get_shader_mut(&mut self) -> &mut Shader {
        &mut self.shader
    }
}

struct GameState {
    was_grounded: bool,
    has_laughed: bool,
    has_played_die_sound: bool,
    end_time: Option<f64>,
    show_ending: bool,
    dialogue_started: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            was_grounded: true,
            has_laughed: false,
            has_played_die_sound: false,
            end_time: None,
            show_ending: false,
            dialogue_started: false,
        }
    }

    fn handle_audio(
        &mut self,
        world: &World,
        audio: &AudioSystem,
        rl: &RaylibHandle,
        footstep: bool,
    ) {
        let jump_input =
            rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_SPACE);
        if self.was_grounded && !world.player.grounded && jump_input {
            Sound::play(&audio.jump_sound);
        }
        if !self.was_grounded && world.player.grounded {
            Sound::play(&audio.fall_sound);
        }
        if footstep {
            Sound::play(&audio.walk_sound);
        }
        if !self.has_laughed && world.player.end_scene_active {
            Sound::play(&audio.laugh_sound);
            self.has_laughed = true;
        }
        if world.player.is_dying && !self.has_played_die_sound {
            Sound::play(&audio.die_sound);
            self.has_played_die_sound = true;
        }
        if !world.player.is_dying {
            self.has_played_die_sound = false;
        }
        self.was_grounded = world.player.grounded;
    }

    fn update_ending(&mut self, current_time: f64, world: &World) {
        if world.player.end_triggered && self.end_time.is_none() {
            self.end_time = Some(current_time);
        }

        if let Some(end_time) = self.end_time {
            if current_time - end_time >= 6.0 {
                self.show_ending = true;
            }
        }
    }

    fn should_show_ending(&self) -> bool {
        self.show_ending
    }

    fn should_start_dialogue(&mut self) -> bool {
        if self.show_ending && !self.dialogue_started {
            self.dialogue_started = true;
            return true;
        }
        false
    }
}

pub struct RenderTarget {
    texture: RenderTexture2D,
    last_width: u32,
    last_height: u32,
}

impl RenderTarget {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            texture: rl.load_render_texture(thread, BASE_WIDTH as u32, BASE_HEIGHT as u32)?,
            last_width: BASE_WIDTH as u32,
            last_height: BASE_HEIGHT as u32,
        })
    }

    pub fn check_resize(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Result<(), Box<dyn Error>> {
        let current_width = rl.get_screen_width() as u32;
        let current_height = rl.get_screen_height() as u32;

        if current_width != self.last_width || current_height != self.last_height {
            self.texture = rl.load_render_texture(thread, BASE_WIDTH as u32, BASE_HEIGHT as u32)?;
            self.last_width = current_width;
            self.last_height = current_height;
        }

        Ok(())
    }

    pub fn get_mut(&mut self) -> &mut RenderTexture2D {
        &mut self.texture
    }

    pub fn draw_to_screen(
        &mut self,
        d: &mut RaylibDrawHandle,
        shader: &mut Shader,
        offset_x: f32,
        offset_y: f32,
        scaled_width: f32,
        scaled_height: f32,
    ) {
        let mut shader_mode = d.begin_shader_mode(shader);
        shader_mode.draw_texture_pro(
            self.texture.texture(),
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

fn calculate_fade_alpha(world: &World, rl: &RaylibHandle) -> u8 {
    if !world.player.is_dying {
        return 0;
    }
    let elapsed = rl.get_time() - world.player.death_start_time;
    let progress = (elapsed / DEATH_ANIMATION_DURATION).clamp(0.0, 1.0);
    let eased = progress * progress * (3.0 - 2.0 * progress);
    (eased * 255.0) as u8
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(BASE_WIDTH, BASE_HEIGHT)
        .title("RetroJam 2025")
        .resizable()
        .build();
    rl.set_target_fps(TARGET_FPS);

    let audio_device = RaylibAudio::init_audio_device()?;
    let audio = AudioSystem::new(&audio_device)?;
    audio.start();

    let mut shader_system = ShaderSystem::new(&mut rl, &thread);
    let mut render_target = RenderTarget::new(&mut rl, &thread)?;
    let mut world = World::new(&mut rl, &thread)?;
    let mut game_state = GameState::new();

    world.dust.spawn(&mut rl, &world.camera);

    let mut step_counter = 0;

    let mut dialogue = DialogueSystem::new(&mut rl, &thread)?;

    while !rl.window_should_close() {
        audio.update();

        let delta_time = rl.get_frame_time();
        let time = rl.get_time();

        // Update ending state
        game_state.update_ending(time, &world);

        // Start dialogue when ending begins
        if game_state.should_start_dialogue() {
            dialogue.start(time);
        }

        // Update dialogue and play sound effects
        if let Some(sound_name) = dialogue.update(time) {
            match sound_name.as_str() {
                "laugh" => Sound::play(&audio.laugh_sound),
                "drip" | _ => {}
            }
        }

        // Handle dialogue choice
        if let Some(choice) = dialogue.handle_choice(&rl) {
            if choice {
                // Live - respawn player
                world.player.respawn();

                // ✅ Preserve has_laughed flag to prevent replay
                let laughed = game_state.has_laughed;

                game_state = GameState::new();
                game_state.has_laughed = laughed;

                dialogue = DialogueSystem::new(&mut rl, &thread)?;
            } else {
                // Die - close program
                break;
            }
        }

        // Only update game logic if not showing ending
        if !game_state.should_show_ending() {
            let footstep = world.player.after_move(&mut rl, &mut world.map);
            if footstep {
                step_counter += 1;
            }
            let should_play_footstep = footstep && step_counter % 2 == 0;

            game_state.handle_audio(&world, &audio, &rl, should_play_footstep);

            world.player.update_sight(delta_time);
            let sight = world.player.get_sight_multiplier(&rl);
            shader_system.update_light(sight);

            world.update_cam();
            world.dust.update(&mut rl);
        }

        let screen_width = rl.get_screen_width() as f32;
        let screen_height = rl.get_screen_height() as f32;

        let fade_alpha = calculate_fade_alpha(&world, &rl);

        if !game_state.should_show_ending() {
            render_target.check_resize(&mut rl, &thread)?;
            {
                let mut texture_mode = rl.begin_texture_mode(&thread, render_target.get_mut());
                texture_mode.clear_background(Color::BLACK);
                world.draw(
                    &mut texture_mode,
                    &(screen_width as i32),
                    &(screen_height as i32),
                    &time,
                );
            }
        }

        render_target.check_resize(&mut rl, &thread)?;

        if game_state.should_show_ending() {
            let mut texture_mode = rl.begin_texture_mode(&thread, render_target.get_mut());
            texture_mode.clear_background(Color::BLACK);
            dialogue.draw(&mut texture_mode, BASE_WIDTH, BASE_HEIGHT);
        }

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            if game_state.should_show_ending() {
                let scale = calculate_scale(screen_width, screen_height);
                let (scaled_width, scaled_height) = calculate_scaled_dimensions(scale);
                let (offset_x, offset_y) =
                    calculate_offsets(screen_width, screen_height, scaled_width, scaled_height);

                d.draw_texture_pro(
                    render_target.get_mut().texture(),
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
            } else {
                let scale = calculate_scale(screen_width, screen_height);
                let (scaled_width, scaled_height) = calculate_scaled_dimensions(scale);
                let (offset_x, offset_y) =
                    calculate_offsets(screen_width, screen_height, scaled_width, scaled_height);

                render_target.draw_to_screen(
                    &mut d,
                    shader_system.get_shader_mut(),
                    offset_x,
                    offset_y,
                    scaled_width,
                    scaled_height,
                );

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
    }

    Ok(())
}

fn calculate_scale(screen_width: f32, screen_height: f32) -> f32 {
    let scale_x = screen_width / BASE_WIDTH as f32;
    let scale_y = screen_height / BASE_HEIGHT as f32;
    scale_x.min(scale_y)
}

fn calculate_scaled_dimensions(scale: f32) -> (f32, f32) {
    (BASE_WIDTH as f32 * scale, BASE_HEIGHT as f32 * scale)
}

fn calculate_offsets(
    screen_width: f32,
    screen_height: f32,
    scaled_width: f32,
    scaled_height: f32,
) -> (f32, f32) {
    (
        (screen_width - scaled_width) / 2.0,
        (screen_height - scaled_height) / 2.0,
    )
}
