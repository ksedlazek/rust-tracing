mod camera;
mod hitable;
mod ray;
mod sphere;
mod vectors;

use camera::Camera;
use hitable::{HitableList, HitableTrait};
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vectors::*;

fn get_color(r: &Ray, world: &dyn HitableTrait) -> Color {
    let hit_record = world.hit(r, 0.0, Num::MAX);
    if hit_record.is_some() {
        let rec = hit_record.unwrap();
        return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * COLORS.white + t * COLORS.sky_blue;
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

fn main() {
    let mut rng = rand::rng();
    let nx = 400;
    let ny = 200;
    let ns = 100;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let cam = Camera::create_default();
    let world = create_world();

    for i in 0..nx {
        for j in 0..ny {
            let mut col = COLORS.white;
            for _ in 0..ns {
                let u = (rng.random::<Num>() + (i as Num)) / (nx as Num);
                let v = (rng.random::<Num>() + (j as Num)) / (ny as Num);
                let r = cam.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                col += get_color(&r, &world);
            }
            col /= ns as Num;
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;
            let pixel = imgbuf.get_pixel_mut(i, ny - j - 1);
            *pixel = image::Rgb([ir, ig, ib]);
        }
    }
    imgbuf.save("picture.png").unwrap();
}
