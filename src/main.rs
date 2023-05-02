mod raytracer;
mod math;
mod object;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use math::{Point3D, Vector3D};
use std::env;

use object::{Object, Sphere, Plane};

use crate::raytracer::{Camera, Rectangle3D, Scene, Light};
#[derive(Serialize, Deserialize, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Deserialize)]
struct SphereData {
    x: f64,
    y: f64,
    z: f64,
    r: f64,
    color: Color,
}
#[derive(Serialize, Deserialize, Debug)]
struct PlaneData {
    axis: String,
    position: i32,
    color: Color,
}
#[derive(Debug, Deserialize)]
struct PrimitivesData {
    spheres: Vec<SphereData>,
    planes: Vec<PlaneData>,
}

fn get_camera_data() -> Camera
{
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("err");
    let cam2 = parse_camera(&json);
    let cam = cam2.unwrap();
    return cam;
}

fn get_objects_data() -> Vec<Box<dyn Object>>
{
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let json_data: serde_json::Value = serde_json::from_str(&contents).expect("err");
    let primitives_json_str = json_data["primitives"].to_string();

    let data: PrimitivesData = serde_json::from_str(&primitives_json_str).unwrap();
    let spheres2: Vec<Sphere> = data.spheres.into_iter().map(|sphere_data| {
        Sphere {
            center: Point3D { x: sphere_data.x, y: sphere_data.y, z: sphere_data.z },
            radius: sphere_data.r,
            color: Vector3D { x: sphere_data.color.r as f64, y: sphere_data.color.g as f64, z: sphere_data.color.b as f64 },
        }
    }).collect();
    let planes2: Vec<Plane> = data.planes.into_iter().map(|plane_data| {
        Plane::new(plane_data.axis, plane_data.position, Vector3D { x: plane_data.color.r as f64, y: plane_data.color.g as f64, z: plane_data.color.b as f64 })
    }).collect();
    let objects: Vec<Box<dyn Object>> = planes2
    .into_iter()
    .map(|plane| Box::new(plane) as Box<dyn Object>)
    .chain(spheres2.into_iter().map(|sphere| Box::new(sphere) as Box<dyn Object>))
    .collect();
    return objects;
}

fn get_height_width_data() -> (u32, u32)
{
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("err");
    let width_height = parse_width_height(&json).expect("error");
    return width_height;
}

fn main() {
    let cam = get_camera_data();
    let objects = get_objects_data();
    let width_height = get_height_width_data();
    let lights = vec![
        Light::default()
    ];
    let plane = Plane::default();
    let height = width_height.1;
    let width = width_height.0;
    let mut scene = Scene::new(cam, objects, lights, plane, width, height);
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
    let field_of_view = camera_json["fieldOfView"].as_f64().unwrap_or_default();

    let screen: Rectangle3D =         Rectangle3D::new(
        Point3D::new(-0.5, -0.5, -1.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
    );

    let camera = Camera {
        origin,
        screen,
        field_of_view,
    };

    Ok(camera)
}

