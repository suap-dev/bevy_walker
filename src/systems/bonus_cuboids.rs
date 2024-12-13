use crate::components::bonus::{Position, Rotator, Swinger};
use bevy::{math::vec3, prelude::*};
use std::f32::consts::TAU;

pub fn spawn_rotating_cuboid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let purple_ish_color =
        materials.add(Color::linear_rgb(135.0 / 255.0, 9.0 / 255.0, 120.0 / 255.0));

    commands.spawn((
        Rotator {
            angular_velocity: TAU / 5.0,
            axis: Dir3::from_xyz(0.1, 0.5, 0.7).unwrap(),
        },
        Mesh3d(meshes.add(Cuboid::from_size(vec3(2.0, 3.0, 1.0)))),
        Transform::from_xyz(-12.0, 2.0, -10.0),
        MeshMaterial3d(purple_ish_color),
    ));
}

pub fn spawn_swinging_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Position(vec3(-9.0, 2.0, -12.0)),
        Swinger {
            amplitude: 1.0,
            period: 6.0,
        },
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));
}

pub fn swing(mut query: Query<(&mut Transform, &Position, &Swinger)>, time: Res<Time>) {
    for (mut transform, Position(position), swinger) in &mut query {
        transform.translation = swinger.get_translation(*position, time.elapsed_secs());
    }
}

pub fn rotate(mut query: Query<(&mut Transform, &Rotator)>, time: Res<Time>) {
    for (mut transform, rotator) in &mut query {
        transform.rotate_axis(rotator.axis, rotator.angular_velocity * time.delta_secs());
    }
}
