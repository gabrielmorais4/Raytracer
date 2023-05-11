mod light;
mod math;
mod object;
mod raytracer;
use image::{ImageBuffer, Rgb};
use math::{Point3D, Vector3D};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::thread::sleep_ms;

use light::{DirectionalLight, Light, PointLight, AmbientLight};
use object::{Cylinder, Object, Plane, Sphere};

use crate::raytracer::{Camera, Rectangle3D, Scene};
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

fn get_camera_data() -> Camera {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("err");
    let cam2 = parse_camera(&json);
    let cam = cam2.unwrap();
    return cam;
}

fn get_objects_data() -> Vec<Box<dyn Object>> {
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

fn get_lights_data() -> Vec<Box<dyn Light>> {
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

fn get_height_width_data() -> (u32, u32) {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args.get(1).expect("error")).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("err");
    let width_height = parse_width_height(&json).expect("error");
    return width_height;
}

// MAIN FOR DEBUG
// fn main() {
//     let height = 640;
//     let width = 860;
//     let screenRatio = width as f64 / height as f64;
//     let cam = Camera::new(
//         Point3D::new(0.0, 0.0, 1.0), 72.0, screenRatio);
//     let objects: Vec<Box<dyn Object>> = vec![
//         // Box::new(Sphere::new(Point3D::new(60.0, 5.0, 40.0), 25.0, Vector3D::new(255.0, 64.0, 64.0))),
//         // Box::new(Sphere::new(Point3D::new(-40.0, 20.0, -10.0), 35.0, Vector3D::new(64.0, 255.0, 64.0))),
//         // Box::new(Sphere::new(Point3D::new(-40.0, 20.0, -5.0), 20.0, Vector3D::new(100.0, 30.0, 24.0))),
//         // Box::new(Plane::new("Z".to_string(), 0, Vector3D::new(64.0, 64.0, 255.0))),
//         Box::new(Sphere::new(Point3D::new(0.0, 1.0, -10.0), 0.5, Vector3D::new(0.0, 255.0, 64.0))),
//         Box::new(Plane::new("Y".to_string(), -1, Vector3D::new(64.0, 64.0, 255.0))),
//     ];
//     let light: Box<PointLight> = Box::new(PointLight::new(Point3D::new(400.0, 100.0, 500.0), Vector3D::new(255.0, 255.0, 255.0), 1.0));
//     let plane = Plane::default();
//     let mut scene = Scene::new(cam, objects, light, plane, width, height);
//     println!("P3\n{}\n{}\n{}", width, height, 255);
//     scene.render();
// }

fn apply_anti_aliasing(file_path: &str) {
    // Abra o arquivo temporário
    let image = image::open(file_path)
        .expect("Falha ao abrir a imagem temporária")
        .to_rgb();
    let width = image.width();
    let height = image.height();

    // Crie uma nova imagem para armazenar a imagem com anti-aliasing
    let mut anti_aliasing_image = ImageBuffer::new(width / 2, height / 2);

    // Percorra cada dois pixels consecutivos e faça a média
    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            let pixel1 = image.get_pixel(x, y);
            let pixel2 = image.get_pixel(x + 1, y);
            let pixel3 = image.get_pixel(x, y + 1);
            let pixel4 = image.get_pixel(x + 1, y + 1);

            let averaged_pixel = average_rgb_pixels(&pixel1, &pixel2, &pixel3, &pixel4);
            anti_aliasing_image.put_pixel(x / 2, y / 2, averaged_pixel);
        }
    }

    // Salve a imagem resultante com anti-aliasing em um novo arquivo PPM
    anti_aliasing_image
        .save("antialising.ppm")
        .expect("Falha ao salvar a imagem com anti-aliasing");
}

fn average_rgb_pixels(
    pixel1: &Rgb<u8>,
    pixel2: &Rgb<u8>,
    pixel3: &Rgb<u8>,
    pixel4: &Rgb<u8>,
) -> Rgb<u8> {
    let r = ((pixel1[0] as u16 + pixel2[0] as u16 + pixel3[0] as u16 + pixel4[0] as u16) / 4) as u8;
    let g = ((pixel1[1] as u16 + pixel2[1] as u16 + pixel3[1] as u16 + pixel4[1] as u16) / 4) as u8;
    let b = ((pixel1[2] as u16 + pixel2[2] as u16 + pixel3[2] as u16 + pixel4[2] as u16) / 4) as u8;

    Rgb([r, g, b])
}

fn main() {
    File::create("data.ppm").expect("cannot create file");
    let mut cam: Camera = get_camera_data();
    let mut objects: Vec<Box<dyn Object>> = get_objects_data();
    let width_height: (u32, u32) = get_height_width_data();
    let width: u32 = width_height.0;
    let height: u32 = width_height.1;
    cam.aspect_ratio = width as f64 / height as f64;
    let lights: Vec<Box<dyn Light>> = get_lights_data();
    // let lights: Vec<Box<dyn Light>> = vec![
        // Box::new(PointLight::new(
        //     Point3D::new(400.0, 100.0, 500.0),
        //     Vector3D::new(255.0, 255.0, 255.0),
        //     1.0,
        // )),
        // Box::new(DirectionalLight::new(
        //     Vector3D::new(0.0, 0.0, 1.0),
        //     Vector3D::new(255.0, 255.0, 255.0),
        //     1.0,
        // ))
    //     Box::new(AmbientLight::new(
    //         Vector3D::new(255.0, 255.0, 255.0),
    //         1.0,
    //     )),
    // ];
    let plane: Plane = Plane::default();
    let mut scene: Scene = Scene::new(cam, objects, lights, plane, width, height);
    println!("P3\n{}\n{}\n{}", width_height.0, width_height.1, 255);
    let mut data_file: File = OpenOptions::new()
        .append(true)
        .open("data.ppm")
        .expect("cannot open file");
    data_file
        .write_all(format!("P3\n{}\n{}\n{}\n", width_height.0, width_height.1, 255).as_bytes())
        .expect("cannot write to file");
    scene.render();
    apply_anti_aliasing("data.ppm");
    fs::remove_file("data.ppm").expect("could not remove file");
}

fn parse_width_height(json: &Value) -> Result<(u32, u32), Box<dyn std::error::Error>> {
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

    let screen: Rectangle3D = Rectangle3D::new(
        Point3D::new(-0.5, -0.5, -1.0),
        Vector3D::new(1.0, 0.0, 0.0),
        Vector3D::new(0.0, 1.0, 0.0),
    );
    let aspect_ratio = width as f64 / height as f64;

    let camera = Camera::new(origin, fov, aspect_ratio);

    Ok(camera)
}
