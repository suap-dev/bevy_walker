use std::f32::consts::TAU;

use bevy::{
    math::{vec3, Dir3, Vec3, VectorSpace},
    prelude::Component,
};

#[derive(Component)]
pub struct Swinger {
    pub speed: f32,
    pub period: f32,
    pub amplitude: f32,
    mean_position: Vec3,
    going_up: bool,
    time_spent: f32,
}
impl Swinger {
    pub fn new(mean_position: Vec3, amplitude: f32, period: f32) -> Self {
        Self {
            mean_position,
            amplitude,
            period,
            speed: 0.0,
            going_up: false,
            time_spent: 0.0,
        }
    }

    pub fn get_translation(&self, elapsed_seconds: f32) -> Vec3 {
        self.mean_position
            + (elapsed_seconds * TAU / self.period).sin() * vec3(0.0, self.amplitude, 0.0)
    }

    pub fn get_delta_height(&self, delta_seconds: f32) -> f32 {
        delta_seconds * &self.speed * if self.going_up { 1.0 } else { -1.0 }
    }
}

#[derive(Component)]
pub struct Rotation {
    pub angular_velocity: f32,
    pub axis: Dir3,
}
