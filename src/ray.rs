use nalgebra as na;

pub struct Ray {
    pub o: na::Vector3<f32>,
    pub d: na::Vector3<f32>,
}

pub trait RayTrait {
    fn origin(&self) -> na::Vector3<f32>;
    fn direction(&self) -> na::Vector3<f32>;
    fn point_at_parameter(&self, t: f32) -> na::Vector3<f32>;
}

impl RayTrait for Ray {
    fn origin(&self) -> na::Vector3<f32> {
        self.o
    }

    fn direction(&self) -> na::Vector3<f32> {
        self.d
    }

    fn point_at_parameter(&self, t: f32) -> na::Vector3<f32> {
        self.o + t * self.d
    }
}
