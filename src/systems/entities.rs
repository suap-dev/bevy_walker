pub mod startup {
    use crate::{components::Position, Rotator, Swinger};
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
            PbrBundle {
                mesh: meshes.add(Cuboid::from_size(vec3(2.0, 3.0, 1.0))),
                material: purple_ish_color,
                transform: Transform::from_xyz(-12.0, 2.0, -10.0),
                ..default()
            },
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
            PbrBundle {
                mesh: meshes.add(Cuboid::from_length(1.0)),
                material: materials.add(Color::WHITE),
                ..default()
            },
        ));
    }

    pub fn spawn_ground(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cylinder::new(40.0, 1.0)),
            material: materials.add(Color::linear_rgb(0.0 / 255.0, 250.0 / 255.0, 162.0 / 255.0)),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        });

        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_length(1.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, -0.499, 0.0),
            ..default()
        });
    }
}

pub mod update {
    use crate::{components::Position, Rotator, Swinger};
    use bevy::prelude::*;

    pub fn swing(mut query: Query<(&mut Transform, &Position, &Swinger)>, time: Res<Time>) {
        for (mut transform, Position(position), swinger) in &mut query {
            transform.translation = swinger.get_translation(*position, time.elapsed_seconds());
        }
    }

    pub fn rotate(mut query: Query<(&mut Transform, &Rotator)>, time: Res<Time>) {
        for (mut transform, rotator) in &mut query {
            transform.rotate_axis(
                rotator.axis,
                rotator.angular_velocity * time.delta_seconds(),
            );
        }
    }
}
