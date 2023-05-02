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