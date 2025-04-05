mod hitable;
mod ray;
mod sphere;
mod vectors;

use hitable::{HitableList, HitableTrait};
use ray::Ray;
use sphere::Sphere;
use vectors::*;

fn color(r: &Ray, world: &dyn HitableTrait) -> Vec3 {
    let hit_record = world.hit(r, 0.0, f32::MAX);
    if hit_record.is_some() {
        let rec = hit_record.unwrap();
        return 0.5 * Vec3::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * COLORS.white + t * COLORS.sky_blue;
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = VECTORS.origin;

    let world: HitableList = HitableList {
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
    };

    for j in 0..ny {
        for i in 0..nx {
            let u = (i as Num) / (nx as Num);
            let v = (j as Num) / (ny as Num);
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let col = color(&r, &world);
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;
            let pixel = imgbuf.get_pixel_mut(i, ny - j - 1);
            *pixel = image::Rgb([ir, ig, ib]);
        }
    }
    imgbuf.save("picture.png").unwrap();
}
