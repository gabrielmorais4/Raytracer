use crate::math::{Point3D, Vector3D};
use crate::object::{Object, Sphere, Plane, HitResult};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Vector3D
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
                        // println!("hit color");
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