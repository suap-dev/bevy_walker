pub mod startup {
    use bevy::{color::palettes::css::ALICE_BLUE, prelude::*};

    use crate::setup::LIGHT_POSITION;

    pub fn spawn(mut commands: Commands) {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                range: 60.0,
                color: bevy::prelude::Color::Srgba(ALICE_BLUE),
                // intensity: todo!(),
                // radius: todo!(),
                // shadow_depth_bias: todo!(),
                // shadow_normal_bias: todo!(),
                ..default()
            },
            transform: Transform::from_translation(LIGHT_POSITION),
            ..default()
        });
    }
}
