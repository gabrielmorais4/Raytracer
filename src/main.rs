mod raytracer;
mod math;
mod object;

use math::{Point3D, Vector3D};

use object::{Object, Sphere, Plane};

use crate::raytracer::{Camera, Rectangle3D, Scene, Light};

fn main() {
    let cam = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        Rectangle3D::new(
            Point3D::new(-0.5, -0.5, -1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(Point3D::new(0.0, 0.0, -1.0), 0.3, Vector3D::new(0.0, 100.0, 100.0))),
        Box::new(Plane::new("Y".to_string(), 1, Vector3D::new(255.0, 0.0, 0.0))),
    ];
    let lights = vec![
        Light::default()
    ];
    let plane = Plane::default();
    let height = 400;
    let width = 400;
    let mut scene = Scene::new(cam, objects, lights, plane, width, height);
    println!("P3\n{}\n{}\n{}", width, height, 255);
    scene.render();
}

