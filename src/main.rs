mod plugins;

use bevy::{
    prelude::*,
    input::common_conditions::input_toggle_active
};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::render::camera::ScalingMode;
use crate::plugins::{
    environment::EnvironmentPlugin,
    zenoh::ZenohPlugin,
    turtle::TurtlePlugin
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Turtlesim".into(),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(ZenohPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(TurtlePlugin)
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    // camera.projection.scaling_mode = ScalingMode::AutoMin {
    //     min_width: 256.0,
    //     min_height: 144.0,
    // };

    commands.spawn(camera);
}
