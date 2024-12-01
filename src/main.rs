mod components;
mod setup;
mod systems;

use bevy::{
    app::AppExit,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    components::{Rotator, Swinger},
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
                player_camera::update::follow_player,
                player::controls,
                handle_exit,
            ),
        )
        .run();
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

// TODO: this should be a part of something like "app_controls"
fn handle_exit(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
