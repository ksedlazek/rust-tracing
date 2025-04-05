use nalgebra as na;

pub struct Ray {
    pub origin: na::Vector3<f32>,
    pub direction: na::Vector3<f32>,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> na::Vector3<f32> {
        self.origin + t * self.direction
    }
}
