use nalgebra as na;
use once_cell::sync::Lazy;

pub type Num = f32;
pub type Vec3 = na::Vector3<Num>;

pub trait UnitTrait {
    fn unit(&self) -> Vec3;
}

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

impl UnitTrait for Vec3 {
    fn unit(&self) -> Vec3 {
        let l = self.len() as Num;
        return Vec3::new(self.x / l, self.y / l, self.z / l);
    }
}

pub type Color = na::Vector3<Num>;

#[allow(dead_code)]
pub struct Colors {
    pub white: Color,
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub sky_blue: Color,
}

pub static COLORS: Lazy<Colors> = Lazy::new(|| Colors {
    white: Color::new(1.0, 1.0, 1.0),
    black: Color::new(0.0, 0.0, 0.0),
    red: Color::new(1.0, 0.0, 0.0),
    green: Color::new(0.0, 1.0, 0.0),
    blue: Color::new(0.0, 0.0, 1.0),
    sky_blue: Color::new(0.5, 0.7, 1.0),
});
