use crate::model::vectors::{vec2::Vec2, quantity_vector::QuantityVector};
use std::ops::Range;
use rand::{Rng, thread_rng};


pub struct Particle {
    mass: f32,
    position: Vec2,
    velocity: QuantityVector,
    acceleration: QuantityVector
}

impl Particle {

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            mass: 1.0,
            position: Vec2::new(x, y),
            velocity: QuantityVector::new(),
            acceleration: QuantityVector::new()
        }
    }

    pub fn new_with_mass(x: f32, y: f32, mass: f32) -> Self {
        Self {
            mass,
            position: Vec2::new(x, y),
            velocity: QuantityVector::new(),
            acceleration: QuantityVector::new()
        }
    }

    pub fn new_random(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            mass: rng.gen_range(0.0..50.0),
            position: Vec2::new_random(&x_range, &y_range),
            velocity: QuantityVector::new(),
            acceleration: QuantityVector::new_random(
                &(-10.0..10.0),
                &(-10.0..10.0),
            ),
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
        // F = ma
        // // int F dt = int m dv/dt dt
        // // Ft + C = mv + C
        // // v = (Ft + C) / m
        // // let each frame be 1 second
        // self.velocity.set_x(force.x() / self.mass);
        // self.velocity.set_y(force.y() / self.mass);
        // self.velocity.clean();
        // let acc_x = force.x() / self.mass();
        // let acc_y = force.y() / self.mass();
        //
        // if acc_x.abs() > 1.0 {
        //     self.acceleration.set_x(1.0 * acc_x.signum());
        // }
        // else {
        //     self.acceleration.set_x(force.x() / self.mass);
        // }
        //
        // if acc_y.abs() > 1.0 {
        //     self.acceleration.set_y(1.0 * acc_y.signum());
        // }
        // else {
        //     self.acceleration.set_y(force.y() / self.mass);
        // }

        self.acceleration.set_x(force.x() / self.mass);
        self.acceleration.set_y(force.y() / self.mass);
    }

    pub fn update(&mut self) {
        println!("{} {}", self.acceleration.x(), self.acceleration.y());
        // let each frame be 1
        // S = 1/2 g t^2
        // println!("{} {}", self.acceleration.x(), self.acceleration.y());
        self.position.x += 0.5 * self.acceleration.x();
        self.position.y += 0.5 * self.acceleration.y();
        // println!("{}", self.position.get_angle());
    }

    pub fn get_force(&self) -> QuantityVector {
        QuantityVector::from_vec2(
            Vec2 {
                x: self.acceleration.x() * self.mass(),
                y: self.acceleration.y() * self.mass(),
            }
        )
    }
}
