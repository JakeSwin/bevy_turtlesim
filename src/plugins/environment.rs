use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn setup_environment(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(100.0, 15.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -60.0, 0.0)));
}

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_environment);
    }
}