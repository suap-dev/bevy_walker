use bevy::{math::vec3, prelude::*};

use crate::components::{Dog, MaxSpeed, Player};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let beige_color = materials.add(Color::linear_rgb(
        249.0 / 255.0,
        196.0 / 255.0,
        138.0 / 255.0,
    ));

    let transform = Transform {
        translation: vec3(3.0, 0.35, 1.0),
        scale: Vec3::ONE,
        ..default()
    };

    commands.spawn((
        Dog,
        Mesh3d(meshes.add(Cuboid::from_size(vec3(0.4, 0.4, 1.4)))),
        MeshMaterial3d(beige_color),
        transform,
        MaxSpeed(3.5),
    ));
}

#[allow(dead_code)]
pub fn stay_with_player(
    mut dog_transform: Query<&mut Transform, (With<Dog>, Without<Player>)>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let player_transform = player_transform.single();

    for mut dog_transform in &mut dog_transform {
        let player_position = player_transform.translation;
        let distance_from_player = 2.0;
        let new_dog_position =
            player_position + Vec3::ZERO.with_x(distance_from_player).with_y(-0.75);
        dog_transform.translation = new_dog_position;
    }
}

pub fn follow_player(
    mut dog_query: Query<(&mut Transform, &MaxSpeed), (With<Dog>, Without<Player>)>,
    player_transform: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_transform = player_transform.single();
    let player_position = player_transform.translation;

    for mut dog in &mut dog_query {
        let dog_transform = &mut dog.0;
        let dog_speed = dog.1 .0;
        let direction = (player_position - dog_transform.translation).with_y(0.0);

        dog_transform.align(Dir3::Z, direction, Dir3::Y, Dir3::Y);

        if direction.length() > 2.0 {
            dog_transform.translation += direction.normalize() * dog_speed * time.delta_secs();
        }
    }
}
