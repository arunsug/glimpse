use bevy::prelude::*;
use bevy::utils::Duration;

use super::{physics::*, walls::WallSensor};


const HAMMER_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const HAMMER_SIZE: Vec2 = Vec2 {x: 0.75, y: 0.5};
pub const HAMMER_SHAFT_SIZE: Vec2 = Vec2 {x: 0.2, y: 2.0};
pub const HAMMER_SWING_TIME:f32 = 0.5;

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
    pub body: Body,
    pub controller: PhysicsControllerBundle,
    pub angular_velocity: AngularVelocity
}

#[derive(Default, Bundle)]
pub struct HammerBundle {
    pub hammer: Hammer,
    pub sprite_bundle: SpriteBundle,
    pub body: Body,
    pub collider: Collider
}

impl HammerBundle {
    pub fn new(
        position: Vec2, head_size: Vec2, shaft_size: Vec2, 
        start_angle: f32, swing_time: f32, angular_velocity: f32) -> (HammerShaftBundle, HammerBundle) 
{
        (HammerShaftBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
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
            timer : HammerTimer(Timer::from_seconds(swing_time, TimerMode::Once)),
            body: Body {
                shape: Shape::Rect(shaft_size),
                position: Position(position),
                rotation: Rotation(start_angle),
                ..default()
            },
            angular_velocity: AngularVelocity(angular_velocity),
            ..default()
        },
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
                position: Position(Vec2 {x: 0.0, y: (shaft_size.y / 2.0) + (head_size.y / 2.0)}),
                ..default()
            },
            ..default()
        })
    }
}
