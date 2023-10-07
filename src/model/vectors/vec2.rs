use std::ops::Range;

use rand::Rng;


#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}


impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn new_random(x_range: Range<f32>, y_range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(x_range),
            y: rng.gen_range(y_range)
        }
    }

    pub fn get_magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn get_angle(&self) -> f32 {
        (self.x / self.y).tan()
    }
}

impl std::ops::Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
