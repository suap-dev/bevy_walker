#![allow(warnings)]

mod components;
mod setup;
mod systems;

use std::f32::consts::TAU;

use bevy::{ecs::query, math::vec3, prelude::*};

use crate::{
    components::*,
    systems::{camera, entities, light},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                light::startup::spawn,
                camera::startup::spawn,
                entities::startup::spawn_ground,
                entities::startup::spawn_swinging_cube,
                entities::startup::spawn_rotating_cuboid,
            ),
        )
        .add_systems(Update, (entities::update::swing, entities::update::rotate))
        .run();
}

