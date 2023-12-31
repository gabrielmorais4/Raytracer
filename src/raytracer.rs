use std::fs::OpenOptions;
use std::io::Write;

use crate::light::{DirectionalLight, Light};
use crate::math::{Point3D, Vector3D};
use crate::object::{HitResult, Object, Plane, Sphere};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D,
}
impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Point3D::default(),
            direction: Vector3D::default(),
        }
    }
}
impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }
    pub fn from(other: Ray) -> Ray {
        std::mem::replace(
            &mut Ray {
                origin: Point3D::default(),
                direction: Vector3D::default(),
            },
            other,
        )
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Rectangle3D {
    origin: Point3D,
    bottom_side: Vector3D,
    left_side: Vector3D,
}
impl Default for Rectangle3D {
    fn default() -> Rectangle3D {
        Rectangle3D {
            origin: Point3D::new(-1.0, -1.0, -1.0),
            bottom_side: Vector3D::new(1.0, 0.0, 0.0),
            left_side: Vector3D::new(0.0, 1.0, 0.0),
        }
    }
}
impl Rectangle3D {
    pub fn new(origin: Point3D, bottom_side: Vector3D, left_side: Vector3D) -> Rectangle3D {
        Rectangle3D {
            origin,
            bottom_side,
            left_side,
        }
    }
    pub fn point_at(&self, u: f64, v: f64) -> Point3D {
        let p0 = self.origin;
        let v1 = self.bottom_side;
        let v2 = self.left_side;
        p0 + v1.scale(u) + v2.scale(v)
    }
}
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Camera {
    pub origin: Point3D,
    pub screen: Rectangle3D,
    pub fov: f64,
    pub aspect_ratio: f64,
}
impl Default for Camera {
    fn default() -> Camera {
        Camera {
            origin: Point3D::default(),
            screen: Rectangle3D::default(),
            fov: 90.0,
            aspect_ratio: 16.0 / 9.0,
        }
    }
}
impl Camera {
    pub fn new(origin: Point3D, fov: f64, aspect_ratio: f64) -> Camera {
        let screen = Self::calculate_screen(origin, fov, aspect_ratio);
        Camera {
            origin,
            screen,
            fov,
            aspect_ratio,
        }
    }
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            (self.screen.point_at(u, v) - self.origin).normalize(),
        )
    }
    pub fn calculate_screen(origin: Point3D, fov: f64, aspect_ratio: f64) -> Rectangle3D {
        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = aspect_ratio * half_height;
        let bottom_left = Vector3D::new(-half_width, -half_height, -1.0);
        let right = Vector3D::new(2.0 * half_width, 0.0, 0.0);
        let up = Vector3D::new(0.0, 2.0 * half_height, 0.0);

        let bottom_left = origin + bottom_left;
        let right = right.normalize();
        let up = up.normalize();

        let bottom_side = right * (2.0 * half_width);
        let left_side = up * (2.0 * half_height);

        Rectangle3D::new(bottom_left, bottom_side, left_side)
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera,                // camera of the scene
    pub objects: Vec<Box<dyn Object>>, // list of Objects
    pub lights: Vec<Box<dyn Light>>,   // list of Lights
    pub plane: Plane,                  // plane of the scene
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            camera: Camera::default(),
            objects: Vec::new(),
            lights: Vec::new(),
            plane: Plane::default(),
            width: 0,
            height: 0,
        }
    }
}

impl Scene {
    pub fn new(
        camera: Camera,
        objects: Vec<Box<dyn Object>>,
        lights: Vec<Box<dyn Light>>,
        plane: Plane,
        width: u32,
        height: u32,
    ) -> Scene {
        Scene {
            camera,
            objects,
            lights,
            plane,
            width,
            height,
        }
    }
    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }
    pub fn add_plane(&mut self, plane: Plane) {
        self.plane = plane;
    }
    pub fn add_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
    fn write_color(color: Vector3D) {
        let color = color.get_color();
        println!("{} {} {}", color.0, color.1, color.2);
        let mut data_file = OpenOptions::new()
            .append(true)
            .open("data.ppm")
            .expect("cannot open file");
        data_file
            .write_all(format!("{} {} {}\n", color.0, color.1, color.2).as_bytes())
            .expect("cannot write to file");
    }
    pub fn compute_lighting_directional(
        object: &Box<dyn Object>,
        light: &Box<dyn Light>,
        hit_point: &Point3D,
        ray: &Ray,
        objects: &Vec<Box<dyn Object>>,
    ) -> Vector3D {
        let surface_normal = object.surface_normal(hit_point);
        let direction_to_light = light.get_direction().normalize();
        let light_power =
            (surface_normal.dot(&direction_to_light)).max(0.0) * light.get_intensity();
        let mut color: Vector3D = Vector3D::new(0.0, 0.0, 0.0);

        let shadow_ray = Ray::new(hit_point.clone() + (surface_normal * 0.001), direction_to_light);

        let mut is_shadowed = false;
        for other_object in objects.iter() {
            if object.get_center() != other_object.get_center() {
                if let Some(_) = other_object.hits(shadow_ray) {
                    is_shadowed = true;
                    break;
                }
            }
        }

        if is_shadowed {
            color = (object.get_color() * light.get_color()).normalize() * light_power;
            let mut shadow_color: Vector3D = Vector3D::new(color.x * 255.0, color.y * 255.0, color.z * 255.0);
            shadow_color *= 0.5;
            return shadow_color;
        } else {
            color = (object.get_color() * light.get_color()).normalize() * light_power;
        }

        let new_color: Vector3D = Vector3D::new(color.x * 255.0, color.y * 255.0, color.z * 255.0);

        new_color
    }

    pub fn find_greater_z(&self, hitting_points: &Vec<Point3D>) -> usize {
        let mut greater_z = hitting_points[0].z;
        let mut index = 0;
        let mut j = 0;
        for hit_points in hitting_points.iter() {
            if hit_points.z > greater_z {
                greater_z = hit_points.z;
                j = index;
            }
            index += 1;
        }
        j
    }
    pub fn render(&mut self) {
        self.objects
            .sort_by(|a, b| b.get_center().z.partial_cmp(&a.get_center().z).unwrap());
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let u = x as f64 / (self.width - 1) as f64;
                let v = y as f64 / (self.height - 1) as f64;
                let r = self.camera.ray(u, v);
                let mut hit_color = Vector3D::new(0.0, 0.0, 0.0);
                let mut multiple_hit = 0;
                let mut hitting_points: Vec<Point3D> = Vec::new();
                let mut hitting_shapes: Vec<&Box<dyn Object>> = Vec::new();
                for s in self.objects.iter() {
                    if let Some(hit_point) = s.hits(r) {
                        multiple_hit += 1;
                        hitting_points.push(hit_point);
                        hitting_shapes.push(s);
                    }
                }
                if multiple_hit == 0 {
                    hit_color = Vector3D::new(0.0, 0.0, 0.0);
                    Self::write_color(hit_color);
                } else if multiple_hit > 1 {
                    let index = self.find_greater_z(&hitting_points);
                    for light in &self.lights {
                        hit_color += Self::compute_lighting_directional(
                            hitting_shapes[index],
                            light,
                            &hitting_points[index],
                            &r,
                            &self.objects,
                        );
                    }
                    Self::write_color(hit_color);
                } else if multiple_hit == 1 {
                    for light in &self.lights {
                        hit_color += Self::compute_lighting_directional(
                            hitting_shapes[0],
                            light,
                            &hitting_points[0],
                            &r,
                            &self.objects,
                        );
                    }
                    Self::write_color(hit_color);
                }
            }
        }
    }
}
