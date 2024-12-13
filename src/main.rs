mod bundles;
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
    components::Velocity,
    systems::{bonus_cuboids, dog, ground, light, player, player_camera},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (
                light::spawn,
                ground::spawn,
                bonus_cuboids::spawn_swinging_cube,
                bonus_cuboids::spawn_rotating_cuboid,
                dog::spawn,
                player_camera::spawn,
                player::spawn,
                cursor_grab,
            ),
        )
        .add_systems(
            Update,
            (
                bonus_cuboids::swing,
                bonus_cuboids::rotate,
                dog::follow_player,
                player_camera::follow_player,
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
