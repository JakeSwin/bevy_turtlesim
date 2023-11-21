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
    mut player_velocity: Query<(&mut Velocity, &Transform), With<RigidBody>>
) {
    for ev in ev_cmd_vel.read() {
        for (mut vel, transform) in &mut player_velocity {
            let angle = Vec2::from_angle(transform.rotation.z);
            let rotated_new_linear = angle.rotate(Vec2::new(ev.0.linear.x as f32, ev.0.linear.y as f32));
            info!("Angle: {:?}, rotated_linear: {:?}", angle, rotated_new_linear);
            vel.linvel += rotated_new_linear;
            vel.angvel += ev.0.angular.z as f32;
        }
    }
}

pub struct TurtlePlugin;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CmdVelEvent>()
            .add_systems(Startup, setup_turtle)
            .add_systems(Update, (check_subscription, apply_velocity));
    }
}