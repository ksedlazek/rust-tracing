mod ray;
mod vectors;

use ray::{Ray, RayTrait};
use vectors::*;

fn hit_sphere(r: &Ray, center: &Vec3, radius: Dim) -> Dim {
    let oc = r.origin - center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn color(r: &Ray) -> Vec3 {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let hs = hit_sphere(r, &center, 0.5);
    if hs > 0.0 {
        let n = (r.point_at_parameter(hs) - center).unit();
        return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * unit_direction.y + 1.0;
    return (1.0 - t) * unit() + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = origin();
    for j in 0..ny {
        for i in 0..nx {
            let u = (i as Dim) / (nx as Dim);
            let v = (j as Dim) / (ny as Dim);
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let col = color(&r);
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;
            let pixel = imgbuf.get_pixel_mut(i, ny - j - 1);
            *pixel = image::Rgb([ir, ig, ib]);
        }
    }

    imgbuf.save("picture.png").unwrap();
}
