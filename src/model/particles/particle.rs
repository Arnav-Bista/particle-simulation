use crate::{model::vectors::{vec2::Vec2, quantity_vector::QuantityVector}, constants::MAX_VELOCITY};
use std::ops::Range;


pub struct Particle {
    mass: f32,
    position: Vec2,
    velocity: QuantityVector
}

impl Particle {

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            mass: 1.0,
            position: Vec2::new(x, y),
            velocity: QuantityVector::new()
        }
    }

    pub fn new_with_mass(x: f32, y: f32, mass: f32) -> Self {
        Self {
            mass,
            position: Vec2::new(x, y),
            velocity: QuantityVector::new()
        }
    }

    pub fn new_random(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        Self {
            mass: 1.0,
            position: Vec2::new_random(x_range, y_range),
            velocity: QuantityVector::new()
        }
    }
    
    pub fn position(&self) -> &Vec2 {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    pub fn apply_force(&mut self, force: &QuantityVector) {
        // velocity = force / mass
        if force.magnitude() > MAX_VELOCITY {
            self.velocity = QuantityVector::from_angle(MAX_VELOCITY / self.mass, force.angle());
        }
        else {
            self.velocity = QuantityVector::from_angle(force.magnitude() / self.mass, force.angle());
        }
    }

    pub fn update(&mut self) {
        // S = V * t
        // let each frame be 1
        println!("V: {} {}", self.velocity.x(), self.velocity.y());
        self.position.x += self.velocity.x();
        self.position.y += self.velocity.y();
    }
}
