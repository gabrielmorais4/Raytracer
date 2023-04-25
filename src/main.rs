mod raytracer;
mod math;

use math::{Point3D, Vector3D};

use crate::raytracer::{Camera, Sphere, Ray};

fn write_color(color: Vector3D) {
    let color = color.get_color();
    println!("{} {} {}", color.0, color.1, color.2);
}

fn main() {
    let cam = Camera::default();
    let s = Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.5);
    let width = 400;
    let height = 400;
    println!("P3\n{}\n{}\n{}", width, height, 255);
    let mut toto = height - 1;
    let mut toto2 = width - 1;
    for _y in 0..height {
        for _x in 0..width {
            let u = (toto2 * 1) as f64 / (width - 1) as f64;
            let v = (toto * 1) as f64 / (height - 1) as f64;
            toto2 = toto2 - 1;
            let r:Ray = cam.ray(u, v);
            if s.hits(r) {
                write_color(Vector3D::new(255.0, 0.0, 0.0));
            } else {
                write_color(Vector3D::new(0.0, 0.0, 255.0));
            }
        }
        toto2 = width - 1;
        toto = toto - 1;
    }
}
