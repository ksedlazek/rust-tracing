use nalgebra as na;
use once_cell::sync::Lazy;

pub type Num = f32;
pub type Vec3 = na::Vector3<Num>;

pub trait UnitTrait {
    fn unit(&self) -> Vec3;
}

impl UnitTrait for Vec3 {
    fn unit(&self) -> Vec3 {
        let l = self.len() as Num;
        return Vec3::new(self.x / l, self.y / l, self.z / l);
    }
}

#[allow(dead_code)]
pub struct Colors {
    pub white: Vec3,
    pub black: Vec3,
    pub red: Vec3,
    pub green: Vec3,
    pub blue: Vec3,
    pub sky_blue: Vec3,
}

pub static COLORS: Lazy<Colors> = Lazy::new(|| Colors {
    white: Vec3::new(1.0, 1.0, 1.0),
    black: Vec3::new(0.0, 0.0, 0.0),
    red: Vec3::new(1.0, 0.0, 0.0),
    green: Vec3::new(0.0, 1.0, 0.0),
    blue: Vec3::new(0.0, 0.0, 1.0),
    sky_blue: Vec3::new(0.5, 0.7, 1.0),
});

#[allow(dead_code)]
pub struct Vectors {
    pub zero: Vec3,
    pub unit: Vec3,
    pub origin: Vec3,
    pub unit_x: Vec3,
    pub unit_y: Vec3,
    pub unit_z: Vec3,
}

pub static VECTORS: Lazy<Vectors> = Lazy::new(|| Vectors {
    zero: Vec3::new(0.0, 0.0, 0.0),
    origin: Vec3::new(0.0, 0.0, 0.0),
    unit: Vec3::new(1.0, 1.0, 1.0),
    unit_x: Vec3::new(1.0, 0.0, 0.0),
    unit_y: Vec3::new(0.0, 1.0, 0.0),
    unit_z: Vec3::new(0.0, 0.0, 1.0),
});
