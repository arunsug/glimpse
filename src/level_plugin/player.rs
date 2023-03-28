use std::time::Duration;

use bevy::prelude::*;

use super::{physics::*, walls::{WallCollider, WallSensor}};

#[derive(Component, Default)]
pub struct Player;

#[derive(Component)]
pub struct Jumper {
    is_jumping: bool,
    can_jump: bool,
    timer: Timer
}

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

const PLAYER_JUMP_ACCEL: Vec2 = Vec2 {x:0.0, y:50.0};
const PLAYER_RUN_ACCEL: Vec2 = Vec2 {x:25.0, y:0.0};

const PLAYER_JUMP_TIME:f32 = 0.1;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteBundle,
    pub position: Position,
    pub shape: Shape,
    pub accel: Acceleration,
    pub vel: Velocity,
    pub gravity: Gravity,
    pub wall_collider: WallCollider,
    pub wall_sensor: WallSensor,
    pub resistance: Resistance,
    pub friction: Friction,
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
            resistance: Resistance(Vec2 {x:0.9, y:0.2}),
            friction: Friction(Vec2 { x:5.0, y:0.0 }),
            shape: Shape::Rect(size),
            ..default()
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ev_move_player: EventWriter<AccelerateEntityEvent>,    
    mut query: Query<(Entity, &WallSensor, &mut Jumper), With<Player>>
) {
    let (player_entity, wall_sensor, mut jumper) = query.single_mut();
    if keyboard_input.pressed(KeyCode::Left) {
       ev_move_player.send(AccelerateEntityEvent(player_entity, -PLAYER_RUN_ACCEL));
    }

    if keyboard_input.pressed(KeyCode::Right) {
       ev_move_player.send(AccelerateEntityEvent(player_entity, PLAYER_RUN_ACCEL));
    }

    // this doesn't work we dont' now how many we are sending and how often
    debug!("jumping info timer {}        is jumping {}       wallsensor.down {}", jumper.timer.finished(), jumper.is_jumping, wall_sensor.down);
    if keyboard_input.pressed(KeyCode::Up) && (wall_sensor.down || jumper.is_jumping) {
        // we need to send a jump signal to the thing I guess and the apply the acceleration in the physcis system
        // no just send the jump acceleration here
        ev_move_player.send(AccelerateEntityEvent(player_entity, PLAYER_JUMP_ACCEL)); //*(PLAYER_JUMP_TIME - jumper.timer.elapsed_secs()) / PLAYER_JUMP_TIME));
        jumper.is_jumping = true;
        jumper.timer.tick(time.delta());
    }
    if keyboard_input.just_released(KeyCode::Up) || jumper.timer.finished() {
        jumper.is_jumping = false;
        jumper.timer.reset();
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
