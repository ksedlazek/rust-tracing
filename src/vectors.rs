use nalgebra as na;

pub type Dim = f32;
pub type Vec3 = na::Vector3<Dim>;

pub trait UnitTrait {
    fn unit(&self) -> Vec3;
}

impl UnitTrait for Vec3 {
    fn unit(&self) -> Vec3 {
        let l = self.len() as f32;
        return Vec3::new(self.x / l, self.y / l, self.z / l);
    }
}

pub fn unit() -> Vec3 {
    return Vec3::new(1.0, 1.0, 1.0);
}

pub fn origin() -> Vec3 {
    return Vec3::new(0.0, 0.0, 0.0);
}
