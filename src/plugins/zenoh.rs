use bevy::prelude::*;
use zenoh::{prelude::sync::*, publication::Publisher, subscriber::Subscriber};

use flume::Receiver;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct Vector3 {
//     x: f64,
//     y: f64,
//     z: f64,
// }
//
// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// struct Twist {
//     linear: Vector3,
//     angular: Vector3,
// }

#[derive(Resource)]
pub struct ZenohSession(pub Arc<Session>);

impl Default for ZenohSession {
    fn default() -> Self {
        Self(zenoh::open(config::default()).res().unwrap().into_arc())
    }
}

#[derive(Component)]
pub struct ZenohPublisher(pub Publisher<'static>);

#[derive(Component)]
pub struct ZenohSubscriber(pub Subscriber<'static, Receiver<Sample>>);

impl ZenohSubscriber {
    pub fn new(topic: &str, zenoh_session: &ZenohSession) -> Self {
        info!("Setting up subscriber for topic: {}", topic);
        Self(
            zenoh_session
                .0
                .declare_subscriber(topic)
                .with(flume::bounded(32))
                .res()
                .unwrap(),
        )
    }
}

// fn setup_subscriber(mut commands: Commands, mut session: ResMut<ZenohSession>) {
//     info!("Setting up subscriber");
//     let subscriber = session
//         .0
//         .declare_subscriber("cmd_vel")
//         .with(flume::bounded(32))
//         .res()
//         .unwrap();
//
//     commands.spawn(ZenohSubscriber(subscriber));
// }
//
// fn check_subscription(subscribers: Query<&ZenohSubscriber>) {
//     let subscriber = subscribers.single();
//     if let Ok(sample) =  subscriber.0.recv_timeout(Duration::from_millis(1)) {
//         match cdr::deserialize_from::<_, Twist, _>(sample.value.payload.reader(), cdr::size::Infinite) {
//             Ok(twist) => info!("Received {:?}", twist),
//             Err(e) => warn!("Error decoding: {}", e)
//         }
//     }
// }

pub struct ZenohPlugin;

impl Plugin for ZenohPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZenohSession>();
        // .add_systems(Startup, setup_subscriber)
        // .add_systems(Update, check_subscription);
    }
}
