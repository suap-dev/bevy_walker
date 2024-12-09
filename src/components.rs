use std::f32::consts::TAU;

use bevy::{
    math::{vec3, Dir3, Vec3},
    prelude::Component,
};

#[derive(Component)]
pub struct Swinger {
    pub amplitude: f32,
    pub period: f32,
}
impl Swinger {
    pub fn get_translation(&self, position: Vec3, elapsed_seconds: f32) -> Vec3 {
        position + (elapsed_seconds * TAU / self.period).sin() * vec3(0.0, self.amplitude, 0.0)
    }
}

#[derive(Component)]
pub struct Rotator {
    pub angular_velocity: f32,
    pub axis: Dir3,
}
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct MaxSpeed(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);
