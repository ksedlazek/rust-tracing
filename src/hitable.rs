use crate::ray::*;
use crate::vectors::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HitRecord {
    pub t: Num,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait HitableTrait {
    fn hit(&self, r: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord>;
}

pub struct HitableList {
    pub list: Vec<Box<dyn HitableTrait>>,
}

impl HitableTrait for HitableList {
    fn hit(&self, r: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        let mut closest_so_far: HitRecord = HitRecord {
            t: t_max,
            p: VECTORS.origin,
            normal: VECTORS.origin,
        };
        let mut hit_anything = false;
        for hitable in self.list.iter() {
            let hit_record = hitable.hit(r, t_min, closest_so_far.t);
            if hit_record.is_some() {
                hit_anything = true;
                closest_so_far = hit_record.unwrap();
            }
        }
        if hit_anything {
            return Some(closest_so_far);
        }
        return None;
    }
}
