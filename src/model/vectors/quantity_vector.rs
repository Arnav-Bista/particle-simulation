use super::vec2::Vec2;

pub struct QuantityVector {
    vector: Vec2,
    magnitude: f32,
    angle: f32
}


impl QuantityVector {

    pub fn new() -> Self {
        Self {
            vector: Vec2::new(0.0, 0.0),
            magnitude: 0.0,
            angle: 0.0
        }
    }

    pub fn from_angle(magnitude: f32, angle: f32) -> Self {
        Self {
            vector: Vec2::new(
                magnitude * angle.cos(),
                magnitude * angle.sin(),
            ),
            magnitude,
            angle
        }
    }

    pub fn from_vec2(vec2: Vec2) -> Self {
        Self {
            magnitude: vec2.get_magnitude(),
            angle: vec2.get_angle(),
            vector: vec2,
        }
    }

    pub fn from_vec2_dirty(vec2: Vec2) -> Self {
        Self {
            vector: vec2,
            magnitude: 0.0,
            angle: 0.0
        }
    }

    pub fn clean(&mut self) {
        self.angle = self.vector.get_angle();
        self.magnitude = self.vector.get_magnitude();
    }

    pub fn x(&self) -> f32 {
        self.vector.x
    }

    pub fn y(&self) -> f32 {
        self.vector.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.vector.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.vector.y = y;
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude
    }

    pub fn set_magnitude(&mut self, magnitude: f32) {
        self.magnitude = magnitude;
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }
}


impl std::ops::Add for &QuantityVector {
    type Output = QuantityVector;

    fn add(self, rhs: Self) -> Self::Output {
        QuantityVector::from_vec2_dirty(
            &self.vector + &rhs.vector
        )
    }
}

impl std::ops::Sub for &QuantityVector {
    type Output = QuantityVector;

    fn sub(self, rhs: Self) -> Self::Output {
        QuantityVector::from_vec2_dirty(
            &self.vector - &rhs.vector
        )
    }
}
