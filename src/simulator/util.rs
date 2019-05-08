use std::ops;

/* A physics vector */
#[derive(Clone)]
pub struct PhysVector {
    pub x: f32,
    pub y: f32,
}

impl ops::Add<&PhysVector> for &PhysVector {

    type Output = PhysVector;
    fn add(self, b: &PhysVector) -> PhysVector {
        PhysVector {
            x: self.x + b.x,
            y: self.y + b.y,
        }
    }
}

impl ops::Mul<f32> for &PhysVector {

    type Output = PhysVector;
    fn mul(self, s: f32) -> PhysVector {
        PhysVector {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl PhysVector {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).powi(2)
    }
}
