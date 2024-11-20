pub mod startup {
    use crate::{Rotation, Swinger};
    use bevy::{math::vec3, prelude::*};
    use std::f32::consts::TAU;

    pub fn spawn_rotating_cuboid(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let cuboid = Cuboid::from_size(vec3(2.0, 3.0, 1.0));
        let mesh_handle = meshes.add(cuboid);

        let purple_ish = Color::linear_rgb(135.0 / 255.0, 9.0 / 255.0, 120.0 / 255.0);
        let color_handle = materials.add(purple_ish);

        let transform = Transform::from_xyz(-12.0, 2.0, -10.0);

        commands.spawn((
            Rotation {
                angular_velocity: TAU / 5.0,
                axis: Dir3::from_xyz(0.1, 0.5, 0.7).unwrap(),
            },
            PbrBundle {
                mesh: mesh_handle,
                material: color_handle,
                transform,
                ..default()
            },
        ));
    }

    pub fn spawn_swinging_cube(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let transform = Transform::from_xyz(-9.0, 2.0, -12.0);
        commands.spawn((
            Swinger::new(transform.translation, 1.0, 6.0),
            PbrBundle {
                mesh: meshes.add(Cuboid::from_length(1.0)),
                material: materials.add(Color::WHITE),
                transform,
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
    use crate::{Rotation, Swinger};
    use bevy::prelude::*;

    pub fn swing(mut query: Query<(&mut Transform, &mut Swinger)>, time: Res<Time>) {
        for (mut transform, mut swinger) in query.iter_mut() {
            // swinger.update(time.delta_seconds());
            // transform.translation.y += swinger.get_delta_height(time.delta_seconds());
            transform.translation = swinger.get_translation(time.elapsed_seconds());
        }
    }

    pub fn rotate(mut query: Query<(&mut Transform, &Rotation)>, time: Res<Time>) {
        for (mut transform, rotation) in &mut query {
            transform.rotate_axis(
                rotation.axis,
                rotation.angular_velocity * time.delta_seconds(),
            );
        }
    }
}
