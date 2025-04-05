use crate::hitable::*;
use crate::ray::*;
use crate::vectors::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: Num,
}

impl HitableTrait for Sphere {
    fn hit(&self, r: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t: temp, p, normal });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t: temp, p, normal });
            }
        }
        None
    }
}
