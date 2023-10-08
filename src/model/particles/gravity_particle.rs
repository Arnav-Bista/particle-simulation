use crate::constants::G;
use crate::model::vectors::quantity_vector::QuantityVector;
use crate::model::vectors::vec2::Vec2;

use super::particle::Particle;


pub struct GravityParticle {
    particle: Particle,
}


impl GravityParticle {
    
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            particle: Particle::new_with_mass(x, y, 1.0e14)
        }
    }

    pub fn particle(&self) -> &Particle {
        &self.particle
    }

    pub fn calculate_force(&self, particle: &Particle) -> QuantityVector {
        // F = Gm1m2 / r^2
        let r = self.particle.position() - particle.position();
        let numerator =  G * self.particle.mass() * particle.mass();
        let angle = r.get_angle();
        let denomerator = r.get_magnitude().powi(2);
        println!("{}",angle);
        QuantityVector::from_angle(numerator / denomerator, angle)
    }
}
