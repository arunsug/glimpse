use std::time::Duration;

use bevy::{prelude::*, transform::commands};

use super::{physics::*, weapon::*, walls::{WallCollider, WallSensor}};

#[derive(Component, Default)]
pub struct Player;
#[derive(Component, Default)]
pub struct Health(f32);


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

const PLAYER_RESIST: Vec2 = Vec2 { x:1.0, y:0.05 };
const PLAYER_FRICTION: Vec2 = Vec2 { x:7.0, y:0.0 };

const PLAYER_RUN_ACCEL: Vec2 = Vec2 {x:30.0, y:0.0};
const PLAYER_JUMP_ACCEL: Vec2 = Vec2 {x:0.0, y:100.0};
const PLAYER_JUMP_VEL:Vec2 = Vec2 {x: 0.0, y: 8.0};
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
    pub jumper: Jumper,
    pub health: Health
}

impl PlayerBundle {
    pub fn new(position: Vec2, size: Vec2) -> PlayerBundle {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3::ONE,
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
                resistance: Resistance(PLAYER_RESIST),
                friction: Friction(PLAYER_FRICTION),
                body: Body {
                    position: Position(size),
                    shape: Shape::Rect(size),
                    ..default() 
                },
                ..default()
            },
            ..default()
        }
    }
}

// shouldk i split this up more?
// like a jump and then side movement and stuff hmmmm
pub fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(Entity, &WallSensor, &mut Jumper, &mut AdjustAcceleration, &mut OverrideVelocity), With<Player>>
) {
    let (player, wall_sensor, mut jumper, mut adjust_accel, mut over_vel, ) = query.single_mut();
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
        over_vel.1 = Some(PLAYER_JUMP_VEL.y);
    }
    if jumper.is_jumping && (keyboard_input.just_released(KeyCode::Up) || jumper.timer.finished()) {
        jumper.is_jumping = false;
        jumper.timer.reset();
        //adjust_accel.0 -= PLAYER_JUMP_ACCEL;
        over_vel.1 = None;
    }

    if keyboard_input.pressed(KeyCode::Down) {
    }

    if keyboard_input.pressed(KeyCode::V) {
        let hammer = HammerBundle::new(HAMMER_SIZE, HAMMER_SHAFT_SIZE);
        let shaft = commands.spawn(hammer.0).id();
        let head = commands.spawn(hammer.1).id();
        commands.entity(player).push_children(&[shaft]);
        commands.entity(shaft).push_children(&[head]);
    }

}

#[derive(Component)]
pub struct Jumper {
    pub is_jumping: bool,
    pub can_jump: bool,
    pub timer: Timer
}
pub fn tick_jump_times(mut query: Query<&mut Jumper>) {
    for mut jumper in query.iter_mut() {
        jumper.timer.tick (Duration::from_secs(PHYSICS_TIME_STEP as u64));
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
