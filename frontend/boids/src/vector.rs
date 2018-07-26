
use std::ops::Add;
use std::ops::AddAssign;
use rand::XorShiftRng;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}
impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    } 
}

impl Vector {
    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }
    /// Scales the vector's components to have a given length
    pub fn set_length(&mut self, length: f64) {
        self.normalize();
        self.x *= length;
        self.y *= length;
    }
    /// Makes the vector into a unit vector
    /// it will have a length of 1
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
    }

    pub fn generate_noise(rng: &mut XorShiftRng, std_dev: f64) -> Vector {
        use rand::distributions::{Normal, Distribution};
        let mean = 0f64;
        let normal = Normal::new(mean, std_dev);
        Vector {
            x: normal.sample(rng),
            y: normal.sample(rng)
        }
    }
}