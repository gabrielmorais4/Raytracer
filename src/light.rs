use serde::{Deserialize, Serialize};

use crate::math::{Point3D, Vector3D};

pub trait Light {
    fn get_direction(&self) -> Vector3D;
    fn get_color(&self) -> Vector3D;
    fn get_intensity(&self) -> f64;
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct DirectionalLight {
    pub direction: Vector3D,
    pub color: Vector3D,
    pub intensity: f64,
}

impl Default for DirectionalLight {
    fn default() -> DirectionalLight {
        DirectionalLight { direction: Vector3D::default(), color: Vector3D::default(), intensity: 1.0 }
    }
}

impl DirectionalLight {
    pub fn new(direction: Vector3D, color: Vector3D, intensity: f64) -> DirectionalLight {
        DirectionalLight { direction, color, intensity }
    }
}

impl Light for DirectionalLight {
    fn get_direction(&self) -> Vector3D {
        self.direction
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
    fn get_intensity(&self) -> f64 {
        self.intensity
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PointLight {
    pub position: Point3D,
    pub color: Vector3D,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(position: Point3D, color: Vector3D, intensity: f64) -> PointLight {
        PointLight { position, color, intensity }
    }
}

impl Light for PointLight {
    fn get_direction(&self) -> Vector3D {
        (self.position - Point3D::new(0.0, 0.0, 0.0)).normalize()
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
    fn get_intensity(&self) -> f64 {
        self.intensity
    }
}

pub struct AmbientLight {
    pub color: Vector3D,
    pub intensity: f64,
}

impl AmbientLight {
    pub fn new(color: Vector3D, intensity: f64) -> AmbientLight {
        AmbientLight { color, intensity }
    }
}

impl Light for AmbientLight {
    fn get_direction(&self) -> Vector3D {
        Vector3D::new(0.0, 0.0, 1.0)
    }

    fn get_color(&self) -> Vector3D {
        self.color
    }

    fn get_intensity(&self) -> f64 {
        self.intensity
    }
}
