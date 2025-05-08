use crate::vectors::*;
use rand::{rngs::ThreadRng, *};

pub struct Random {
    rng: ThreadRng,
}

pub fn create_random() -> Random {
    Random { rng: rand::rng() }
}

impl Random {
    pub fn random_zero_to_one(&mut self) -> Num {
        self.rng.random::<Num>()
    }

    pub fn random_in_unit_sphere(&mut self) -> Vec3 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_zero_to_one_range() {
        let mut rng = create_random();
        let value = rng.random_zero_to_one();
        assert!(
            value >= 0.0 && value <= 1.0,
            "Value out of range: {}",
            value
        );
    }

    #[test]
    fn test_random_in_unit_sphere_length() {
        let mut rng = create_random();
        let v = rng.random_in_unit_sphere();
        let length_squared = v.dot(&v);
        assert!(
            length_squared < 1.0,
            "Vector is not in unit sphere: {}",
            length_squared
        );
    }
}
