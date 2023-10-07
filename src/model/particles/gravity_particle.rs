use crate::constants::G;
use crate::model::vectors::quantity_vector::QuantityVector;

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
        // F = -Gm1m2 / r^2
        let r = self.particle.position() - particle.position();
        let angle = r.get_angle();
        let numerator = G * self.particle.mass() * particle.mass();
        let mut denomerator = r.get_magnitude().powi(2);
        if denomerator < 100.0 {
            denomerator = 100.0;
        }
        QuantityVector::from_angle(-1.0 * numerator / denomerator, angle)
    }
}
