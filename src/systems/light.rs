pub mod startup {
    use bevy::prelude::*;

    use crate::setup::LIGHT_POSITION;

    pub fn spawn(mut commands: Commands) {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                shadows_enabled: true,
                range: 60.0,
                // color: todo!(),
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
