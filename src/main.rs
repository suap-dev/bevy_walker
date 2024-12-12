mod components;
mod setup;
mod systems;

use bevy::{
    app::AppExit,
    math::vec3,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    components::{Rotator, Swinger, Velocity},
    systems::{entities, light, player, player_camera},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                light::startup::spawn,
                entities::startup::spawn_ground,
                entities::startup::spawn_swinging_cube,
                entities::startup::spawn_rotating_cuboid,
                entities::startup::spawn_dog,
                player_camera::startup::spawn,
                player::spawn,
                cursor_grab,
            ),
        )
        .add_systems(
            Update,
            (
                entities::update::swing,
                entities::update::rotate,
                entities::update::noga,
                player_camera::update::follow_player,
                player::controls,
                listen_for_exit_event,
                handle_physics,
            ),
        )
        .run();
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

// FIXME: this should be a part of something like "app_controls"
fn listen_for_exit_event(
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

pub fn handle_physics(
    mut physical_query: Query<(&mut Transform, &mut Velocity)>,
    time_resource: Res<Time>,
) {
    const GRAVITY: Vec3 = vec3(0.0, -9.81, 0.0);

    let (mut transform, mut velocity) = physical_query.single_mut();

    let position = &mut transform.translation;
    let velocity = &mut velocity.0;
    let delta = time_resource.delta_secs();

    *position += *velocity * delta;
    *velocity += GRAVITY * delta;

    if position.y < 1.0 {
        position.y = 1.0;
        velocity.y = 0.0;
    }
}