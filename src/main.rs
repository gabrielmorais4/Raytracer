mod raytracer;
mod math;
mod object;
mod light;

use math::{Point3D, Vector3D};
use serde_json::Value;
use std::env;

use object::{Object, Sphere, Plane};

use light::{DirectionalLight};

use crate::{raytracer::{Camera, Rectangle3D, Scene}, light::PointLight};

fn main() {
    let height = 640;
    let width = 860;
    let screenRatio = width as f64 / height as f64;
    let cam = Camera::new(
        Point3D::new(0.0, -100.0, 20.0), 72.0, screenRatio);
    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(Point3D::new(60.0, 5.0, 40.0), 25.0, Vector3D::new(255.0, 64.0, 64.0))),
        Box::new(Sphere::new(Point3D::new(-40.0, 20.0, -10.0), 35.0, Vector3D::new(64.0, 255.0, 64.0))),
        Box::new(Sphere::new(Point3D::new(-40.0, 20.0, -5.0), 20.0, Vector3D::new(100.0, 30.0, 24.0))),
        Box::new(Plane::new("Z".to_string(), -20, Vector3D::new(64.0, 64.0, 255.0))),
    ];
    let light: Box<PointLight> = Box::new(PointLight::new(Point3D::new(400.0, 100.0, 500.0), Vector3D::new(255.0, 255.0, 255.0), 1.0));
    let plane = Plane::default();
    let mut scene = Scene::new(cam, objects, light, plane, width, height);
    println!("P3\n{}\n{}\n{}", width, height, 255);
    scene.render();
}

fn parse_width_height(json: &Value) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let camera_json = json
        .get("camera")
        .ok_or_else(|| "Camera configuration not found in JSON")?;

    let resolution_json = camera_json
        .get("resolution")
        .ok_or_else(|| "Camera resolution not found in JSON")?;
    let width = u32::try_from(resolution_json["width"].as_u64().unwrap_or_default())?;
    let height = u32::try_from(resolution_json["height"].as_u64().unwrap_or_default())?;
    Ok((width, height))
}

fn parse_camera(json: &Value) -> Result<Camera, Box<dyn std::error::Error>> {
    let camera_json = json
        .get("camera")
        .ok_or_else(|| "Camera configuration not found in JSON")?;

    let resolution_json = camera_json
        .get("resolution")
        .ok_or_else(|| "Camera resolution not found in JSON")?;
    let width = resolution_json["width"].as_u64().unwrap_or_default();
    let height: u64 = resolution_json["height"].as_u64().unwrap_or_default();

    let position_json = camera_json
        .get("position")
        .ok_or_else(|| "Camera position not found in JSON")?;
    let x = position_json["x"].as_f64().unwrap_or_default();
    let y = position_json["y"].as_f64().unwrap_or_default();
    let z = position_json["z"].as_f64().unwrap_or_default();
    let origin = Point3D { x, y, z };

    let rotation_json = camera_json
        .get("rotation")
        .ok_or_else(|| "Camera rotation not found in JSON")?;
    let fov = camera_json["fieldOfView"].as_f64().unwrap_or_default();

    let screen: Rectangle3D =         Rectangle3D::new(
        Point3D::new(-0.5, -0.5, -1.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
    );
    let aspect_ratio = width as f64 / height as f64;

    let camera = Camera {
        origin,
        screen,
        fov,
        aspect_ratio,
    };

    Ok(camera)
}

