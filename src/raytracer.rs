use std::collections::VecDeque;

use crate::math::{Point3D, Vector3D};

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3D,
    direction: Vector3D
}
impl Default for Ray {
    fn default() -> Ray {
        Ray { origin: Point3D::default(), direction: Vector3D::default() }
    }
}
impl Ray {
    pub fn new(origin: Point3D, direction: Vector3D) -> Ray {
        Ray { origin, direction }
    }
    pub fn from(other: Ray) -> Ray {
        std::mem::replace(&mut Ray {origin: Point3D::default(), direction: Vector3D::default()}, other)
    }
}
#[derive(PartialEq)]
enum HitResult {
    Hit,
    Missed,
}

pub trait Object {
    fn hits(&self, ray: Ray) -> HitResult;
    fn surface_normal(&self, hit_point: &Point3D) -> Vector3D;
    fn get_center(&self) -> Point3D;
    fn get_color(&self) -> Vector3D;
}
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub color: Vector3D,
}
impl Default for Sphere {
    fn default() -> Sphere {
        Sphere { center: Point3D::default(), radius: 0.0, color: Vector3D::default() }
    }
}
impl Sphere {
    pub fn new(center: Point3D, radius: f64, color: Vector3D) -> Sphere {
        Sphere { center, radius, color }
    }
}
impl Object for Sphere {
    fn hits(&self, ray: Ray) -> HitResult {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return HitResult::Missed;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        if t1 < 0.0 && t2 < 0.0 {
            return HitResult::Missed;
        }
        HitResult::Hit
    }
    fn surface_normal(&self, hit_point: &Point3D) -> Vector3D {
        (*hit_point - self.center).normalize()
    }
    fn get_center(&self) -> Point3D {
        self.center
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}

pub struct Light {
    pub direction: Vector3D,
    pub color: Vector3D,
    pub intensity: f32,
}

impl Default for Light {
    fn default() -> Light {
        Light { direction: Vector3D::default(), color: Vector3D::default(), intensity: 0.0 }
    }
}

impl Light {
    pub fn new(direction: Vector3D, color: Vector3D, intensity: f32) -> Light {
        Light { direction, color, intensity }
    }
}
pub struct Plane {
    pub axis: String,
    pub position: i32,
    pub normal: Vector3D,
    pub color: Vector3D,
}

impl Default for Plane {
    fn default() -> Plane {
        Plane { axis: String::from("x"), position: 0, normal: Vector3D::default(), color: Vector3D::default() }
    }
}

impl Plane {
    pub fn new(axis: String, position: i32, normal: Vector3D, color: Vector3D) -> Plane {
        Plane { axis, position, normal, color}
    }
    pub fn hits(&self, ray: Ray) -> bool {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let point = Point3D::new(0.0, 0.0, 0.0);
            let v = point - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return true
            }
        }
        false
    }
}
pub struct Rectangle3D {
    origin: Point3D,
    bottom_side: Vector3D,
    left_side: Vector3D
}
impl Default for Rectangle3D {
    fn default() -> Rectangle3D {
        Rectangle3D { origin: Point3D::new(-1.0, -1.0, -1.0), bottom_side: Vector3D::new(1.0, 0.0, 0.0), left_side: Vector3D::new(0.0, 1.0, 0.0) }
    }
}
impl Rectangle3D {
    pub fn new(origin: Point3D, bottom_side: Vector3D, left_side: Vector3D) -> Rectangle3D {
        Rectangle3D { origin, bottom_side, left_side }
    }
    pub fn point_at(&self, u: f64, v: f64) -> Point3D {
        let p0 = self.origin;
        let v1 = self.bottom_side;
        let v2 = self.left_side;
        p0 + v1.scale(u) + v2.scale(v)
    }
}
pub struct Camera {
    pub origin: Point3D,
    pub screen: Rectangle3D
}
impl Default for Camera {
    fn default() -> Camera {
        Camera { origin: Point3D::default(), screen: Rectangle3D::default() }
    }
}
impl Camera {
    pub fn new(origin: Point3D, screen: Rectangle3D) -> Camera {
        Camera { origin, screen }
    }
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, (self.screen.point_at(u, v) - self.origin).normalize())
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub camera: Camera, // camera of the scene
    pub objects: Vec<Box<dyn Object>>, // list of Objects
    pub lights: Vec<Light>, // list of Lights
    pub plane: Plane // plane of the scene
}

impl Default for Scene {
    fn default() -> Scene {
        Scene { camera: Camera::default(), objects: Vec::new(), lights: Vec::new(), plane: Plane::default(), width: 0, height: 0 }
    }
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Box<dyn Object>>, lights: Vec<Light>, plane: Plane, width: u32, height: u32) -> Scene {
        Scene { camera, objects, lights, plane, width, height }
    }
    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
    pub fn add_light(&mut self, light: Light) {
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
    }
    pub fn render(&mut self) {
        self.objects.sort_by(|a, b| b.get_center().z.partial_cmp(&a.get_center().z).unwrap());
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let u = x as f64 / (self.width - 1) as f64;
                let v = y as f64 / (self.height - 1) as f64;
                let r = self.camera.ray(u, v);
                let mut hit_color = Vector3D::new(0.0, 0.0, 0.0);
                for s in self.objects.iter() {
                    if s.hits(r) == HitResult::Hit {
                        hit_color = s.get_color();
                        break;
                    } else {
                        hit_color = Vector3D::new(0.0, 0.0, 255.0);
                    }
                }
                Self::write_color(hit_color);
            }
        }
    }
}