use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use zenoh::prelude::sync::*;

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::plugins::zenoh::{ZenohSession, ZenohSubscriber};

#[derive(Event)]
struct CmdVelEvent(Twist);

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Twist {
    linear: Vector3,
    angular: Vector3,
}

fn setup_turtle(mut commands: Commands, session: Res<ZenohSession>) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(25.0))
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.2,
            angular_damping: 1.0,
        })
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 0.0)))
        .insert(ZenohSubscriber::new("cmd_vel", session.into_inner()));
}

fn check_subscription(
    subscribers: Query<&ZenohSubscriber, With<RigidBody>>,
    mut ev_cmd_vel: EventWriter<CmdVelEvent>
) {
    let subscriber = subscribers.single();
    if let Ok(sample) = subscriber.0.recv_timeout(Duration::from_millis(1)) {
        if let Ok(twist) =
            cdr::deserialize_from::<_, Twist, _>(sample.value.payload.reader(), cdr::size::Infinite)
        {
            info!("Writing Event");
            ev_cmd_vel.send(CmdVelEvent(twist))
        }
    }
}

fn apply_velocity(
    mut ev_cmd_vel: EventReader<CmdVelEvent>,
    mut player_velocity: Query<(&mut Velocity, &Transform), With<RigidBody>>,
    mut gizmos: Gizmos
) {
    for ev in ev_cmd_vel.read() {
        for (mut vel, transform) in &mut player_velocity {
            let angle = Vec2::from_angle(transform.rotation.to_euler(EulerRot::XYZ).2);
            gizmos.ray_2d(transform.translation.truncate(), angle * 100.0, Color::GREEN);
            let rotated_new_linear = angle.rotate(Vec2::new(ev.0.linear.x as f32, ev.0.linear.y as f32));
            gizmos.ray_2d(transform.translation.truncate(), rotated_new_linear * 100.0, Color::RED);
            vel.linvel += rotated_new_linear;
            vel.angvel += ev.0.angular.z as f32;
        }
    }
}

fn cast_ray_down(
    rapier_context: Res<RapierContext>,
    mut gizmos: Gizmos,
    position: Query<&Transform, With<RigidBody>>,
) {
    let ball_transform = position.single();

    let ray_origin = ball_transform.translation.truncate() + Vec2::new(0.0, -30.0);
    let ray_dir = Vec2::new(0.0, -100.0);
    let max_toi = 4.0;
    let solid = true;
    let filter = QueryFilter::default();

    if let Some((_entity, toi)) =
        rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, filter)
    {
        let hit_point = ray_origin + ray_dir * toi;
        gizmos.line_2d(ray_origin, hit_point, Color::BLUE);
    }
}

pub struct TurtlePlugin;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CmdVelEvent>()
            .add_systems(Startup, setup_turtle)
            .add_systems(Update, (check_subscription, apply_velocity, cast_ray_down));
    }
}
