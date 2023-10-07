use nannou::window;

use super::particles::{gravity_particle::GravityParticle, particle::Particle};

pub struct Model {
    gravity_particles: Vec<GravityParticle>,
    regular_particles: Vec<Particle>,
    screen_size: (f32, f32)
}

impl Model {
    pub fn new(screen_size: (u32, u32)) -> Self {

        Self {
            gravity_particles: Vec::new(),
            regular_particles: Vec::new(),
            screen_size: (screen_size.0 as f32, screen_size.1 as f32)
        }
    }

    pub fn add_regular_particles(&mut self, number: u32) {
        let x_half = self.screen_size.0 / 2.0;
        let y_half = self.screen_size.1 / 2.0;
        for _ in 0..number {
            let x_range = -x_half..x_half;
            let y_range = -y_half..y_half;
            self.regular_particles.push(Particle::new_random(x_range, y_range));
        }
    }

    pub fn add_gravity_particle(&mut self, x: f32, y: f32) {
        self.gravity_particles.push(GravityParticle::new(x, y));
    }


    pub fn regular_particles(&self) -> &[Particle] {
        self.regular_particles.as_ref()
    }

    pub fn regular_particles_mut(&mut self) -> &mut Vec<Particle> {
        &mut self.regular_particles
    }

    pub fn gravity_particles(&self) -> &[GravityParticle] {
        self.gravity_particles.as_ref()
    }

    pub fn gravity_particles_mut(&mut self) -> &mut Vec<GravityParticle> {
        &mut self.gravity_particles
    }
}
