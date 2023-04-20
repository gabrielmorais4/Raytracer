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
    let s = Sphere::new(Point3D::new(0.5, 0.5, 0.0), 0.5);
    println!("P3\n400 400\n255");
    for _y in 1..400 {
        for _x in 1..400 {
            let u = (_x * 1) as f64 / 400.0;
            let v = (_y * 1) as f64 / 400.0;
            let r:Ray = cam.ray(u, v);
            if s.hits(r) {
                write_color(Vector3D::new(255.0, 0.0, 0.0));
            } else {
                write_color(Vector3D::new(0.0, 0.0, 255.0));
            }
        }
    }
}
