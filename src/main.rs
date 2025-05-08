mod camera;
mod hitable;
mod random;
mod ray;
mod scene;
mod sphere;
mod vectors;

use hitable::HitableTrait;
use indicatif::{ProgressBar, ProgressStyle};
use random::*;
use ray::Ray;
use std::time::Duration;
use vectors::*;

fn get_color(r: &Ray, world: &dyn HitableTrait, rnd: Random) -> Color {
    let hit_record = world.hit(r, 0.0, Num::MAX);
    if hit_record.is_some() {
        let rec = hit_record.unwrap();
        let target = rec.p + rec.normal + rnd.random_in_unit_sphere();
        let reflected = &Ray {
            origin: rec.p,
            direction: target - rec.p,
        };
        return 0.5 * get_color(reflected, world, rnd);
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * COLORS.white + t * COLORS.sky_blue;
    }
}

fn create_progress(total_size: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%")
            .unwrap()
            .tick_chars("⠋⠙⠚⠛⠋⠙⠚⠛"),
    );
    pb.enable_steady_tick(Duration::from_millis(500));
    pb.set_message("Rendering:");
    return pb;
}

fn main() {
    let mut rnd = random::create_random();
    let nx = 400;
    let ny = 200;
    let ns = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);
    let pb = create_progress((nx * ny * ns) as u64);
    let mut progress = 0;

    let scene = scene::create_chapter5_scene();

    for i in 0..nx {
        for j in 0..ny {
            let mut col = COLORS.white;
            for _ in 0..ns {
                progress += 1;
                pb.set_position(progress as u64);
                let u = (rnd.random_zero_to_one() + (i as Num)) / (nx as Num);
                let v = (rnd.random_zero_to_one() + (j as Num)) / (ny as Num);
                let r = scene.camera.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                col += get_color(&r, &scene.world, rnd);
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
