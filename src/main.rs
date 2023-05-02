mod raytracer;
mod math;
mod object;
mod light;

use math::{Point3D, Vector3D};

use object::{Object, Sphere, Plane};

use light::{DirectionalLight};

use crate::raytracer::{Camera, Rectangle3D, Scene};

fn main() {
    let cam = Camera::new(
        Point3D::new(0.0, -10.0, 0.0),
        Rectangle3D::new(
            Point3D::new(-0.5, -10.5, 1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        ),
    );
    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(Point3D::new(0.0, 0.0, 250.0), 25.0, Vector3D::new(255.0, 64.0, 64.0))),
        Box::new(Sphere::new(Point3D::new(-40.0, 20.0, -10.0), 35.0, Vector3D::new(64.0, 255.0, 64.0))),
        Box::new(Plane::new("Y".to_string(), -20, Vector3D::new(64.0, 64.0, 255.0))),
    ];
    let light = Box::new(DirectionalLight::new(Vector3D::new(-1.0, -1.0, 0.0).normalize(), Vector3D::new(255.0, 255.0, 255.0), 1.0));
    let plane = Plane::default();
    let height = 640;
    let width = 860;
    let mut scene = Scene::new(cam, objects, light, plane, width, height);
    println!("P3\n{}\n{}\n{}", width, height, 255);
    scene.render();
}

