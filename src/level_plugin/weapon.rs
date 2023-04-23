use bevy::prelude::*;
use bevy::utils::Duration;

use super::{physics::*, walls::WallSensor};


const HAMMER_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HAMMER_SIZE: Vec2 = Vec2 {x: 0.2, y: 0.2};
pub const HAMMER_SHAFT_SIZE: Vec2 = Vec2 {x: 0.1, y: 1.0};
const HAMMER_SWING_TIME:f32 = 0.3;

#[derive(Default, Component)]
pub struct Hammer;

#[derive(Default, Component)]
pub struct HammerShaft;


#[derive(Default, Component)]
pub struct HammerTimer(Timer);

#[derive(Default, Bundle)]
pub struct HammerShaftBundle {
    pub hammer: Hammer,
    pub hammer_shaft: HammerShaft,
    pub sprite_bundle: SpriteBundle,
    pub timer: HammerTimer,
    pub body: Body
}

#[derive(Default, Bundle)]
pub struct HammerBundle {
    pub hammer: Hammer,
    pub sprite_bundle: SpriteBundle,
    pub body: Body
}

impl HammerBundle {
    pub fn new(head_size: Vec2, shaft_size: Vec2) -> (HammerShaftBundle, HammerBundle) {
        (HammerShaftBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec3::ONE,
                    ..default()
                },
                sprite: Sprite {
                    color: HAMMER_COLOR,
                    custom_size: Some(shaft_size),
                    ..default()
                },
                ..default()
            },
            timer : HammerTimer(Timer::from_seconds(HAMMER_SWING_TIME, TimerMode::Once)),
            body: Body {
                shape: Shape::Rect(shaft_size),
                ..default()
            },
            ..default()
        } ,
        HammerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::ZERO,
                    scale: Vec3::ONE,
                    ..default()
                },
                sprite: Sprite {
                    color: HAMMER_COLOR,
                    custom_size: Some(head_size),
                    ..default()
                },
                ..default()
            },
            body: Body {
                shape: Shape::Rect(head_size),
                position: Position(Vec2 {x: 0.0, y: shaft_size.y / 2.0}),
                ..default()
            },
            ..default()
        })
    }
}

pub fn tick_hammer_times(mut query: Query<&mut HammerTimer>) {
    for mut hammer in query.iter_mut() {
        hammer.0.tick(Duration::from_secs(PHYSICS_TIME_STEP as u64));
    }
}

/*
pub fn swing_hammer(query: Query<Rotation, With<HammerShaft>>) {
   
}
*/