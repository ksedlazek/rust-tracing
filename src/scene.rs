use crate::camera::*;
use crate::hitable::HitableList;
use crate::sphere::Sphere;
use crate::vectors::*;

pub struct Scene {
    pub world: HitableList,
    pub camera: Camera,
}

pub fn create_chapter5_scene() -> Scene {
    Scene {
        world: create_world(),
        camera: create_camera(),
    }
}

pub fn create_camera() -> Camera {
    Camera {
        origin: VECTORS.origin,
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
    }
}

fn create_world() -> HitableList {
    HitableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            }),
        ],
    }
}
