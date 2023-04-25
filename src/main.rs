mod raytracer;
mod math;

use math::{Point3D, Vector3D};

use crate::raytracer::{Camera, Sphere, Plane, Ray, Rectangle3D};

fn write_color(color: Vector3D) {
    let color = color.get_color();
    println!("{} {} {}", color.0, color.1, color.2);
}

fn main() {
    let cam = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        Rectangle3D::new(
            Point3D::new(-0.5, -0.5, -1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );

    let mut spheres = vec![
        Sphere::new(Point3D::new(0.0, 0.0, -2.0), 0.8, Vector3D::new(255.0, 0.0, 0.0)),
        Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.2, Vector3D::new(189.0, 130.0, 0.0)),
    ];
    let plane = Plane::default();
    spheres.sort_by(|a, b| b.center.z.partial_cmp(&a.center.z).unwrap());

    let width = 400;
    let height = 400;

    println!("P3\n{}\n{}\n{}", width, height, 255);

    for y in (0..height).rev() {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = y as f64 / (height - 1) as f64;
            let r = cam.ray(u, v);

            let mut hit_color = Vector3D::new(0.0, 0.0, 0.0);
            for s in spheres.iter() {
                if s.hits(r) {
                    hit_color = s.color;
                    break;
                } else {
                    hit_color = Vector3D::new(0.0, 0.0, 255.0);
                }
            }

            write_color(hit_color);
        }
    }
}

