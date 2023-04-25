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
        pub fn hits(&self, ray: Ray) -> bool {
            let oc = ray.origin - self.center;
            let a = ray.direction.dot(&ray.direction);
            let b = 2.0 * oc.dot(&ray.direction);
            let c = oc.dot(&oc) - self.radius.powi(2);
            let discriminant = b.powi(2) - 4.0 * a * c;
            if discriminant < 0.0 {
                return false;
            }
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);
            if t1 < 0.0 && t2 < 0.0 {
                return false;
            }
            true
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
