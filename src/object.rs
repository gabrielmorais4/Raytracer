use crate::{math::{Point3D, Vector3D}, raytracer::Ray};
#[derive(PartialEq)]
pub enum HitResult {
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

pub struct Plane {
    // pub axis: String,
    // pub position: i32,
    pub origin: Point3D,
    pub normal: Vector3D,
    pub color: Vector3D,
}

impl Default for Plane {
    fn default() -> Plane {
        // Plane { axis: String::from("x"), position: 0, normal: Vector3D::default(), color: Vector3D::default() }
        Plane { origin: Point3D::default(), normal: Vector3D::default(), color: Vector3D::default() }
    }
}

impl Plane {
    pub fn new(origin: Point3D, normal: Vector3D, color: Vector3D) -> Plane {
        Plane { origin, normal, color }
    }
}
impl Object for Plane {
    fn hits(&self, ray: Ray) -> HitResult {
        let normalize = ray.direction.normalize();
        let denom = normalize.dot(&self.normal);
        // println!("denom: {}", denom);
        if denom > 0.0 {
            let p0l0: Vector3D = self.origin - ray.origin;
            let t = p0l0.dot(&self.normal) / denom;
            return HitResult::Hit;
        }
        HitResult::Missed
    }
    fn surface_normal(&self, _hit_point: &Point3D) -> Vector3D {
        self.normal
    }
    fn get_center(&self) -> Point3D {
        self.origin
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}