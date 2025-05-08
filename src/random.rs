use crate::vectors::*;
use rand::{rngs::ThreadRng, *};

pub struct Random {
    rng: ThreadRng,
}

pub fn create_random() -> Random {
    Random { rng: rand::rng() }
}

impl Random {
    pub fn random_zero_to_one(&self) -> Num {
        self.rng.random::<Num>()
    }

    pub fn random_in_unit_sphere(&self) -> Vec3 {
        loop {
            let p = 2.0
                * Vec3::new(
                    self.rng.random::<Num>() - 1.0,
                    self.rng.random::<Num>() - 1.0,
                    self.rng.random::<Num>() - 1.0,
                );
            if p.dot(&p) < 1.0 {
                return p;
            }
        }
    }
}
