mod light;
mod math;
mod object;
mod raytracer;
mod parser;
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
use parser::{Parser};

use crate::raytracer::{Camera, Rectangle3D, Scene};


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
    let _parser: Parser = Parser::new();
    let mut cam: Camera = Parser::get_camera_data();
    let mut objects: Vec<Box<dyn Object>> = Parser::get_objects_data();
    let width_height: (u32, u32) = Parser::get_height_width_data();
    let width: u32 = width_height.0;
    let height: u32 = width_height.1;
    cam.aspect_ratio = width as f64 / height as f64;
    let lights: Vec<Box<dyn Light>> = Parser::get_lights_data();
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

