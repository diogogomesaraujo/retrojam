use crate::*;
use raylib::prelude::*;
use std::error::Error;

pub struct Dust {
    texture: Texture2D,
    particles: Vec<Particle>,
}

pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Dust {
    pub fn new(
        game_handle: &mut RaylibHandle,
        game_thread: &RaylibThread,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            particles: Vec::with_capacity(NUMBER_OF_PARTICLES as usize),
            texture: game_handle.load_texture(game_thread, PARTICLE_PATH)?,
        })
    }

    pub fn spawn(&mut self, game_handle: &mut RaylibHandle, camera: &Camera2D) {
        let (screen_width, screen_height) = (
            game_handle.get_screen_width() + camera.target.x as i32,
            game_handle.get_screen_height() + camera.target.y as i32,
        );
        for i in 0..NUMBER_OF_PARTICLES {
            if i as usize >= self.particles.len() {
                self.particles.push(Particle {
                    position: Vector2::zero(),
                    velocity: Vector2::zero(),
                });
            }

            self.particles[i as usize].position.x = game_handle
                .get_random_value::<i32>(((-screen_width as f32 * 0.4) as i32)..screen_width)
                as f32;
            self.particles[i as usize].position.y =
                game_handle.get_random_value::<i32>((-screen_height as i32 * 2)..0) as f32;

            let max = 10000.0f32;
            // Reduced velocity by multiplying by a lower factor
            self.particles[i as usize].velocity.x =
                (game_handle.get_random_value::<i32>(1..(max as i32)) as f32 / max)
                    * (PARTICLE_VELOCITY * 0.5);
            self.particles[i as usize].velocity.y =
                (game_handle.get_random_value::<i32>(1..(max as i32)) as f32 / max)
                    + (PARTICLE_VELOCITY * 0.5);
        }
    }

    pub fn update(&mut self, game_handle: &mut RaylibHandle) {
        let (screen_width, screen_height) = (
            game_handle.get_screen_width() as f32,
            game_handle.get_screen_height() as f32,
        );

        for i in 0..NUMBER_OF_PARTICLES as usize {
            if i >= self.particles.len() {
                continue;
            }
            let particle = &mut self.particles[i];
            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;

            if particle.position.x > screen_width || particle.position.y > screen_height {
                self.reset_particle(game_handle, i, screen_width, screen_height);
            }
        }
    }

    fn reset_particle(
        &mut self,
        game_handle: &mut RaylibHandle,
        index: usize,
        screen_width: f32,
        screen_height: f32,
    ) {
        let particle = &mut self.particles[index];
        particle.position.x = game_handle
            .get_random_value::<i32>(((-screen_width * 0.4) as i32)..(screen_width as i32))
            as f32;
        particle.position.y =
            game_handle.get_random_value::<i32>((-screen_height as i32 * 2)..0) as f32;
        let max = 10000.0f32;
        // Reduced velocity in reset_particle as well
        particle.velocity.x =
            (game_handle.get_random_value::<i32>(1..(max as i32)) as f32 / max) * 0.25f32;
        particle.velocity.y =
            (game_handle.get_random_value::<i32>(1..(max as i32)) as f32 / max) + 0.4f32;
    }
    pub fn draw<D: RaylibDraw>(&mut self, draw_handle: &mut D) {
        for particle in &self.particles {
            draw_handle.draw_texture(
                &self.texture,
                particle.position.x as i32,
                particle.position.y as i32,
                Color::new(255, 255, 255, 128),
            );
        }
    }
}
