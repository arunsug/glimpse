use std::time::Duration;

use bevy::prelude::*;

use super::{physics::*, walls::{WallCollider, WallSensor}};

#[derive(Component, Default)]
pub struct Player;

impl Default for Jumper {
    fn default() -> Self {
        Jumper {
            is_jumping: false,
            can_jump: true,
            timer: Timer::new(Duration::from_secs_f32(PLAYER_JUMP_TIME), TimerMode::Once)
        }
    }
}

pub struct DoubleJumper {
    is_jumping: bool,
    can_jump: bool
}

const PLAYER_COLOR: Color = Color::rgb(0.2, 0.0, 0.2);

const PLAYER_RUN_ACCEL: Vec2 = Vec2 {x:25.0, y:0.0};

const PLAYER_JUMP_ACCEL: Vec2 = Vec2 {x:0.0, y:100.0};
const PLAYER_JUMP_VEL:Vec2 = Vec2 {x: 0.0, y: 5.0};
const PLAYER_JUMP_TIME:f32 = 0.3;



#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteBundle,
    pub physics: BasePhysicsBundle,
    pub gravity: Gravity,
    pub wall_collider: WallCollider,
    pub wall_sensor: WallSensor,
    pub physics_controller: PhysicsControllerBundle,
    pub jumper: Jumper
}

impl PlayerBundle {
    pub fn new(position: Vec2, size: Vec2) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            physics: BasePhysicsBundle {
                resistance: Resistance(Vec2 {x:0.9, y:0.2}),
                friction: Friction(Vec2 { x:5.0, y:0.0 }),
                body: Body {
                    position: Position(size),
                    shape: Shape::Rect(size) 
                },
                ..default()
            },
            ..default()
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&WallSensor, &mut Jumper, &mut AdjustAcceleration, &mut OverrideVelocity), With<Player>>
) {
    let (wall_sensor, mut jumper, mut adjust_accel, mut over_vel, ) = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Left) {
        adjust_accel.0 -= PLAYER_RUN_ACCEL;
    } else if keyboard_input.just_released(KeyCode::Left) {
        adjust_accel.0 += PLAYER_RUN_ACCEL;
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
       adjust_accel.0 += PLAYER_RUN_ACCEL;
    } else if keyboard_input.just_released(KeyCode::Right) {
        adjust_accel.0 -= PLAYER_RUN_ACCEL;
    }

    if jumper.is_jumping {    
        jumper.timer.tick(time.delta());
    }
    if keyboard_input.pressed(KeyCode::Up) && wall_sensor.down && !jumper.is_jumping {
        jumper.is_jumping = true;
        //adjust_accel.0 += PLAYER_JUMP_ACCEL;
        over_vel.1 = PLAYER_JUMP_VEL;
        over_vel.0 = true;
    }
    if jumper.is_jumping && (keyboard_input.just_released(KeyCode::Up) || jumper.timer.finished()) {
        jumper.is_jumping = false;
        jumper.timer.reset();
        //adjust_accel.0 -= PLAYER_JUMP_ACCEL;
        over_vel.1 = Vec2::ZERO;
        over_vel.0 = false;
    }

    if keyboard_input.pressed(KeyCode::Down) {
    }

}

#[cfg(debug_assertions)]
pub fn debug_player(query: Query<(&GlobalTransform, &Visibility, &Position, &Velocity, &Acceleration), With<Player>>) {
    let (global, vis, pos, vel, accel) = query.get_single().unwrap();
    let (scale, _rot, trans) = global.to_scale_rotation_translation();
    //debug!("{}", trans);
    //debug!("{}", scale);
    //debug!("{:?}", pos);
    //debug!("{:?}", vel);
    //debug!("{:?}", accel);
    //debug!("{:?}", vis);


}
