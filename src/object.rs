use crate::{math::{Point3D, Vector3D}, raytracer::Ray};
use serde::{Serialize, Deserialize};
#[derive(PartialEq)]
pub enum HitResult {
    Hit,
    Missed,
}


pub trait Object {
    fn hits(&self, ray: Ray) -> Option<Point3D>;
    fn surface_normal(&self, hit_point: &Point3D) -> Vector3D;
    fn get_center(&self) -> Point3D;
    fn get_color(&self) -> Vector3D;
    fn get_albedo(&self, hit_point: &Point3D, light_dir: &Vector3D) -> Vector3D;
}

use std::fmt::{self, Debug};
impl Debug for dyn Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
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
    fn hits(&self, ray: Ray) -> Option<Point3D> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        if t1 < 0.0 && t2 < 0.0 {
            return None;
        }
        let t = if t1 < t2 { t1 } else { t2 };
        let hit_point = ray.origin + ray.direction * t;
        Some(hit_point)
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
    fn get_albedo(&self, hit_point: &Point3D, light_dir: &Vector3D) -> Vector3D {
        let radius_sq = self.radius.powi(2);
        let distance_sq = light_dir.length();
        let factor = 4.0 * std::f64::consts::PI * radius_sq / distance_sq;
        self.color * factor
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Plane {
    pub origin: Point3D,
    pub normal: Vector3D,
    pub color: Vector3D,
    pub axis: String,
    pub position: i32,
}

impl Default for Plane {
    fn default() -> Plane {
        Plane { origin: Point3D::default(), normal: Vector3D::default(), color: Vector3D::default(), axis: "Z".to_string(), position: 0 }
    }
}

impl Plane {
    pub fn new(axis: String, position: i32, color: Vector3D) -> Plane {
        if axis == "X" {
            Plane { origin: Point3D::new(position as f64, 0.0, 0.0), normal: Vector3D::new(0.0, 0.0, 1.0), color, axis, position }
        } else if axis == "Y" {
            Plane { origin: Point3D::new(0.0, position as f64, 0.0), normal: Vector3D::new(-1.0, 0.0, 0.0), color, axis, position }
        } else {
            Plane { origin: Point3D::new(0.0, 0.0, position as f64), normal: Vector3D::new(0.0, -1.0, 0.0), color, axis, position }
        }
    }
    pub fn rotate(&mut self, axis: String, angle: f64) {
        let mut new_normal = Vector3D::default();
        if axis == "X" {
            new_normal.x = self.normal.x;
            new_normal.y = self.normal.y * angle.cos() - self.normal.z * angle.sin();
            new_normal.z = self.normal.y * angle.sin() + self.normal.z * angle.cos();
        } else if axis == "Y" {
            new_normal.x = self.normal.x * angle.cos() + self.normal.z * angle.sin();
            new_normal.y = self.normal.y;
            new_normal.z = -self.normal.x * angle.sin() + self.normal.z * angle.cos();
        } else {
            new_normal.x = self.normal.x * angle.cos() - self.normal.y * angle.sin();
            new_normal.y = self.normal.x * angle.sin() + self.normal.y * angle.cos();
            new_normal.z = self.normal.z;
        }
        self.normal = new_normal;
    }
}
impl Object for Plane {
    fn hits(&self, ray: Ray) -> Option<Point3D> {
        let mut value = 0.0;
        if (self.axis == "Z") {
            value = self.position as f64 - ray.origin.z;
            if (value.abs() < (ray.direction.z + value).abs()) {
                return None;
            }
            let firstPoint = Point3D::new(100.0, 200.0, self.position as f64);
            let secondPoint = Point3D::new(200.0, -100.0, self.position as f64);
            let thirdPoint = Point3D::new(-30.0, -50.0, self.position as f64);
            let firstVec = firstPoint - secondPoint;
            let secondVec = thirdPoint - secondPoint;
            let mut normal = firstVec.cross(secondVec);
            normal = normal.normalize();
            let t = normal.dot(&(firstPoint - ray.origin)) / normal.dot(&ray.direction);
            let point = ray.origin + ray.direction * t;
            return Some(point);
        }
        if (self.axis == "Y") {
            value = self.position as f64 - ray.origin.y * -1.0;
            if (value.abs() < (ray.direction.y * -1.0 + value).abs()) {
                return None;
            }
            let firstPoint = Point3D::new(100.0, self.position as f64, 200.0);
            let secondPoint = Point3D::new(200.0, self.position as f64, -100.0);
            let thirdPoint = Point3D::new(-30.0, self.position as f64, -50.0);
            let firstVec = firstPoint - secondPoint;
            let secondVec = thirdPoint - secondPoint;
            let mut normal = firstVec.cross(secondVec);
            normal = normal.normalize();
            let t = normal.dot(&(firstPoint - ray.origin)) / normal.dot(&ray.direction);
            let point = ray.origin + ray.direction * t;
            return Some(point);
        }
        if (self.axis == "X") {
            value = self.position as f64 - ray.origin.x;
            if (value.abs() < (ray.direction.x + value).abs()) {
                return None;
            }
            let firstPoint = Point3D::new(self.position as f64, 100.0, 200.0);
            let secondPoint = Point3D::new(self.position as f64, 200.0, -100.0);
            let thirdPoint = Point3D::new(self.position as f64, -30.0, -50.0);
            let firstVec = firstPoint - secondPoint;
            let secondVec = thirdPoint - secondPoint;
            let mut normal = firstVec.cross(secondVec);
            normal = normal.normalize();
            let t = normal.dot(&(firstPoint - ray.origin)) / normal.dot(&ray.direction);
            let point = ray.origin + ray.direction * t;
            return Some(point);
        }
        None
    }
    fn surface_normal(&self, _hit_point: &Point3D) -> Vector3D {
        self.normal * -1.0
    }
    fn get_center(&self) -> Point3D {
        self.origin
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
    fn get_albedo(&self, hit_point: &Point3D, light_dir: &Vector3D) -> Vector3D {
        self.color
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Cylinder {
    pub position: Point3D,
    pub radius: f64,
    pub color: Vector3D,
}

impl Cylinder {
    pub fn new(position: Point3D, radius: f64, axis: &str, color: Vector3D) -> Cylinder {
        let position_value = match axis {
            "X" => position.x,
            "Y" => position.y,
            "Z" => position.z,
            _ => position.x,
        };
        Cylinder {
            position: position,
            radius: radius,
            color: color,
        }
    }
}

impl Object for Cylinder {
    fn hits(&self, ray: Ray) -> Option<Point3D> {
        let oc = ray.origin - self.position;
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * (oc.x * ray.direction.x + oc.z * ray.direction.z);
        let c = oc.x.powi(2) + oc.z.powi(2) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b - sqrt_discriminant) / (2.0 * a);
        let t2 = (-b + sqrt_discriminant) / (2.0 * a);
        let t = if t1 < 0.0 { t2 } else { t1 };
        let hit_point = ray.origin + ray.direction * t;
        Some(hit_point)
    }

    fn surface_normal(&self, hit_point: &Point3D) -> Vector3D {
        let y = hit_point.y;
        let normal = Vector3D::new(hit_point.x - self.position.x, 0.0, hit_point.z - self.position.z).normalize();
        if y < self.position.y + 1.0 {
            return normal;
        }
        normal * -1.0
    }

    fn get_center(&self) -> Point3D {
        self.position + Vector3D::new(0.0, 0.5, 0.0)
    }

    fn get_color(&self) -> Vector3D {
        self.color
    }

    fn get_albedo(&self, hit_point: &Point3D, light_dir: &Vector3D) -> Vector3D {
        let radius_sq = self.radius.powi(2);
        let distance_sq = light_dir.length();
        let factor = 4.0 * std::f64::consts::PI * radius_sq / distance_sq;
        self.color * factor
    }
}
