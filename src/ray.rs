use crate::vectors::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at_parameter(&self, t: Num) -> Vec3 {
        self.origin + t * self.direction
    }
}
