use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
#[derive(Copy, Clone, Debug)]
pub struct Vector3D {
   pub x: f64,
   pub y: f64,
   pub z: f64,
}
impl Default for Vector3D {
    fn default() -> Vector3D {
        Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}
impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }
    pub fn from(other: Vector3D) -> Vector3D {
        std::mem::replace(&mut Vector3D { x: 0.0, y: 0.0, z: 0.0 }, other)
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn get_color(&self) -> (u8, u8, u8) {
        let r = (255.0 * self.x) as u8;
        let g = (255.0 * self.y) as u8;
        let b = (255.0 * self.z) as u8;
        (r, g, b)
    }
    pub fn normalize(&self) -> Vector3D {
        let len = self.length();
        Vector3D {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
    pub fn scale(&self, s: f64) -> Vector3D {
        Vector3D { x: self.x * s, y: self.y * s, z: self.z * s }
    }
}
impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Div<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn div(self, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, other: Vector3D) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, other: Vector3D) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl DivAssign<Vector3D> for Vector3D {
    fn div_assign(&mut self, other: Vector3D) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
impl SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, other: Vector3D) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl Mul<f64> for Vector3D {
    type Output = Vector3D;
    fn mul(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}
impl Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}
impl Mul<Point3D> for Vector3D {
    type Output = Point3D;
    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Default for Point3D {
    fn default() -> Point3D {
        Point3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}
impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
    pub fn from(other: Point3D) -> Point3D {
        std::mem::replace(&mut Point3D { x: 0.0, y: 0.0, z: 0.0 }, other)
    }
}
impl Add<Vector3D> for Point3D {
    type Output = Point3D;
    fn add(self, other: Vector3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub<Vector3D> for Point3D {
    type Output = Point3D;
    fn sub(self, other: Vector3D) -> Point3D {
        Point3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<Vector3D> for Point3D {
    type Output = Point3D;
    fn mul(self, other: Vector3D) -> Point3D {
        Point3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Div<Vector3D> for Point3D {
    type Output = Point3D;
    fn div(self, other: Vector3D) -> Point3D {
        Point3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl Add<f64> for Point3D {
    type Output = Point3D;
    fn add(self, scalar: f64) -> Point3D {
        Point3D {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
        }
    }
}
impl Sub<f64> for Point3D {
    type Output = Point3D;
    fn sub(self, scalar: f64) -> Point3D {
        Point3D {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
        }
    }
}
impl Mul<f64> for Point3D {
    type Output = Point3D;
    fn mul(self, scalar: f64) -> Point3D {
        Point3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl Div<f64> for Point3D {
    type Output = Point3D;
    fn div(self, scalar: f64) -> Point3D {
        Point3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl Sub<Point3D> for Point3D {
    type Output = Vector3D;
    fn sub(self, other: Point3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Add<Point3D> for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Mul<Point3D> for Point3D {
    type Output = Point3D;
    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Div<Point3D> for Point3D {
    type Output = Point3D;
    fn div(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}