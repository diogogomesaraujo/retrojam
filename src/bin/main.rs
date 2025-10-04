use raylib::prelude::*;
use retrojam::shaders::TORCH_FRAGMENT_SHADER;
use retrojam::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("RetroJam")
        .build();

    rl.set_target_fps(TARGET_FPS);

    // Load the lighting shader
    let mut shader = rl.load_shader_from_memory(&thread, None, Some(TORCH_FRAGMENT_SHADER));

    // Get shader uniform locations
    let player_pos_loc = shader.get_shader_location("playerPos");
    let resolution_loc = shader.get_shader_location("resolution");
    let light_radius_loc = shader.get_shader_location("lightRadius");
    let light_intensity_loc = shader.get_shader_location("lightIntensity");

    // Set static uniforms once (these don't change)
    shader.set_shader_value(resolution_loc, [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32]);
    shader.set_shader_value(light_radius_loc, 200.0f32);
    shader.set_shader_value(light_intensity_loc, 0.95f32);

    // Create render texture once
    let mut render_target =
        rl.load_render_texture(&thread, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)?;

    // Initialize world
    let mut world = World::new(&mut rl, &thread)?;

    // Player screen position (constant since camera follows player)
    let player_screen_x = SCREEN_WIDTH as f32 / 2.0;
    let player_screen_y = SCREEN_HEIGHT as f32 / 2.0;

    // Set player position for shader (screen center)
    shader.set_shader_value(player_pos_loc, [player_screen_x, player_screen_y]);

    // Main game loop
    while !rl.window_should_close() {
        // Update game logic
        world.player.after_move(&mut rl, &mut world.map);
        world.update_cam();

        // Render to texture
        {
            let mut texture_mode = rl.begin_texture_mode(&thread, &mut render_target);
            texture_mode.clear_background(Color::BLACK);

            // Draw world (world.draw will handle camera internally or you draw with camera here)
            world.draw(&mut texture_mode);
        }

        // Draw final scene with shader
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            // Apply lighting shader to render texture
            {
                let mut shader_mode = d.begin_shader_mode(&mut shader);
                shader_mode.draw_texture_rec(
                    render_target.texture(),
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: SCREEN_WIDTH as f32,
                        height: -SCREEN_HEIGHT as f32, // Flip vertically
                    },
                    Vector2::zero(),
                    Color::WHITE,
                );
            }
        }
    }

    Ok(())
}
