use std::{fs::File, env, io::Read};

use crate::{object::{Object, Sphere, Plane, Cylinder}, math::{Point3D, Vector3D}, light::{Light, PointLight, DirectionalLight}, raytracer::{Camera, Rectangle3D}};
use serde::{Serialize, Deserialize};
use serde_json::Value;


pub struct Parser {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Object>>,
    pub width_height: (u32, u32),
    pub lights: Vec<Box<dyn Light>>,

}

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

#[derive(Serialize, Deserialize, Debug)]
struct CylinderData {
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
    axis: String,
    color: Color,
}
#[derive(Debug, Deserialize)]
struct PrimitivesData {
    spheres: Option<Vec<SphereData>>,
    planes: Option<Vec<PlaneData>>,
    cylinders: Option<Vec<CylinderData>>,
}

#[derive(Debug, Deserialize)]
struct LightData {
    point: Option<Vec<PointLightData>>,
    directional: Option<Vec<DirectionalLightData>>,
}

#[derive(Debug, Deserialize)]
struct PointLightData {
    x: f64,
    y: f64,
    z: f64,
    color: Color,
    intensity: f64,
}

#[derive(Debug, Deserialize)]
struct DirectionalLightData {
    x: f64,
    y: f64,
    z: f64,
    color: Color,
    intensity: f64,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            camera: Self::get_camera_data(),
            objects: Self::get_objects_data(),
            width_height: Self::get_height_width_data(),
            lights: Self::get_lights_data(),
        }
    }


    pub fn get_camera_data() -> Camera {
        let args: Vec<String> = env::args().collect();
        let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        let json: Value = serde_json::from_str(&contents).expect("err");
        let cam2 = Self::parse_camera(&json);
        let cam = cam2.unwrap();
        return cam;
    }

    pub fn get_objects_data() -> Vec<Box<dyn Object>> {
        let args: Vec<String> = env::args().collect();
        let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let json_data: serde_json::Value = serde_json::from_str(&contents).expect("err");
        let primitives_json_str = json_data["primitives"].to_string();

        let data: PrimitivesData = serde_json::from_str(&primitives_json_str).unwrap();
        let spheres2: Vec<Sphere> = data
            .spheres
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|sphere_data| {
                Sphere::new(
                    Point3D {
                        x: sphere_data.x,
                        y: sphere_data.y,
                        z: sphere_data.z,
                    },
                    sphere_data.r,
                    Vector3D {
                        x: sphere_data.color.r as f64,
                        y: sphere_data.color.g as f64,
                        z: sphere_data.color.b as f64,
                    },
                )
            })
            .collect();
        let planes2: Vec<Plane> = data
            .planes
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|plane_data| {
                Plane::new(
                    plane_data.axis,
                    plane_data.position,
                    Vector3D {
                        x: plane_data.color.r as f64,
                        y: plane_data.color.g as f64,
                        z: plane_data.color.b as f64,
                    },
                )
            })
            .collect();
        let cylinders2: Vec<Cylinder> = data
            .cylinders
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|cylinder_data| {
                Cylinder::new(
                    Point3D {
                        x: cylinder_data.x,
                        y: cylinder_data.y,
                        z: cylinder_data.z,
                    },
                    cylinder_data.radius,
                    cylinder_data.axis.as_str(),
                    Vector3D {
                        x: cylinder_data.color.r as f64,
                        y: cylinder_data.color.g as f64,
                        z: cylinder_data.color.b as f64,
                    },
                )
            })
            .collect();
        let objects: Vec<Box<dyn Object>> = planes2
            .into_iter()
            .map(|plane| Box::new(plane) as Box<dyn Object>)
            .chain(
                spheres2
                    .into_iter()
                    .map(|sphere| Box::new(sphere) as Box<dyn Object>),
            )
            .chain(
                cylinders2
                    .into_iter()
                    .map(|cylinder| Box::new(cylinder) as Box<dyn Object>),
            )
            .collect();
        return objects;
    }

    pub fn get_lights_data() -> Vec<Box<dyn Light>> {
        let args: Vec<String> = env::args().collect();
        let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let json_data: serde_json::Value = serde_json::from_str(&contents).expect("err");
        let primitives_json_str = json_data["lights"].to_string();

        let data: LightData = serde_json::from_str(&primitives_json_str).unwrap();
        let point: Vec<PointLight> = data
            .point
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|point_data| {
                PointLight::new(
                    Point3D {
                        x: point_data.x,
                        y: point_data.y,
                        z: point_data.z,
                    },
                    Vector3D {
                        x: point_data.color.r as f64,
                        y: point_data.color.g as f64,
                        z: point_data.color.b as f64,
                    },
                    point_data.intensity,
                )
            })
            .collect();
        let direct: Vec<DirectionalLight> = data
            .directional
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|direct_data| {
                DirectionalLight::new(
                    Vector3D {
                        x: direct_data.x,
                        y: direct_data.y,
                        z: direct_data.z,
                    },
                    Vector3D {
                        x: direct_data.color.r as f64,
                        y: direct_data.color.g as f64,
                        z: direct_data.color.b as f64,
                    },
                    direct_data.intensity,
                )
            })
            .collect();
        let objects: Vec<Box<dyn Light>> = point
            .into_iter()
            .map(|plane| Box::new(plane) as Box<dyn Light>)
            .chain(
                direct
                    .into_iter()
                    .map(|sphere| Box::new(sphere) as Box<dyn Light>),
            )
            .collect();
        return objects;
    }

    pub fn get_height_width_data() -> (u32, u32) {
        let args: Vec<String> = env::args().collect();
        let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        let json: Value = serde_json::from_str(&contents).expect("err");
        let width_height = Self::parse_width_height(&json).expect("error");
        return width_height;
    }


    pub fn parse_width_height(json: &Value) -> Result<(u32, u32), Box<dyn std::error::Error>> {
        let camera_json = json
            .get("camera")
            .ok_or_else(|| "Camera configuration not found in JSON")?;

        let resolution_json = camera_json
            .get("resolution")
            .ok_or_else(|| "Camera resolution not found in JSON")?;
        let width = u32::try_from(resolution_json["width"].as_u64().unwrap_or_default())? * 2;
        let height = u32::try_from(resolution_json["height"].as_u64().unwrap_or_default())? * 2;
        Ok((width, height))
    }

    pub fn parse_camera(json: &Value) -> Result<Camera, Box<dyn std::error::Error>> {
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

        let screen: Rectangle3D = Rectangle3D::new(
            Point3D::new(-0.5, -0.5, -1.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        );
        let aspect_ratio = width as f64 / height as f64;

        let camera = Camera::new(origin, fov, aspect_ratio);

        Ok(camera)
    }
}