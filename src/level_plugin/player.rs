use std::time::Duration;

use bevy::{prelude::*, transform::commands};

use super::{physics::*, weapon::*, walls::{WallCollider, WallSensor}};

#[derive(Component, Default)]
pub struct Player;
#[derive(Component, Default)]
pub struct Health(f32);

#[derive(Default)]
pub enum JumpStates {
    Jumping(Timer),
    Jumpable,
    #[default]
    Unjumpable
}

#[derive(Component, Default)]
pub struct Jumper {
    // TODO replace this with an enium for cleaner modeling?
    state: JumpStates
}
#[derive(Component, Default)]
pub struct DoubleJumper {
    // TODO replace this with an enium for cleaner modeling?
    state: JumpStates
}

pub fn tick_jump_times(mut commands: Commands, mut query: Query<&mut Jumper>) {
    println!("ticking jupm times system start");
    for mut jumper in query.iter_mut() {
        if let JumpStates::Jumping(ref mut timer) = jumper.state {
            timer.tick(Duration::from_secs_f32(PHYSICS_TIME_STEP));
        }
    }
    println!("ticking jupm times system end");
}

#[derive(Default)]
pub enum AttackStates {
    Attacking(Timer, Entity),
    #[default]
    CanAttack,
    NoAttack(Timer)
}

#[derive(Component, Default)]
pub struct Attacker {
    state: AttackStates
}

pub fn tick_attack_times(mut commands: Commands, mut query: Query<&mut Attacker>) {
    println!("tick attack times system start");
    for mut attacker in query.iter_mut() {
        match attacker.state {
            AttackStates::Attacking(ref mut timer, _) => {
                timer.tick(Duration::from_secs_f32(PHYSICS_TIME_STEP));
            }
            AttackStates::NoAttack(ref mut timer) => {
                timer.tick(Duration::from_secs_f32(PHYSICS_TIME_STEP));
            }
            _ => (),
        }
    }
    println!("tick attack times system end");
    
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
    pub attacker: Attacker,
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
    mut query: Query<(Entity, &WallSensor, &mut Jumper, &mut AdjustAcceleration, &mut OverrideVelocity, &mut Attacker), With<Player>>
) {
    println!("move player system start");
    let (player, wall_sensor, mut jumper, mut adjust_accel, mut over_vel, mut attacker) = query.single_mut();
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

    match &jumper.state {
        JumpStates::Jumpable => {
            if keyboard_input.just_pressed(KeyCode::Up) {
                over_vel.1 = Some(PLAYER_JUMP_VEL.y);
                jumper.state = JumpStates::Jumping(Timer::from_seconds(PLAYER_JUMP_TIME, TimerMode::Once))
                //adjust_accel.0 += PLAYER_JUMP_ACCEL;
            } else if !wall_sensor.down {
                jumper.state = JumpStates::Unjumpable
            }  
        }
        JumpStates::Jumping(timer) => {
            if keyboard_input.just_released(KeyCode::Up) || timer.finished() {
                over_vel.1 = None;
                jumper.state = JumpStates::Unjumpable;
                //adjust_accel.0 -= PLAYER_JUMP_ACCEL;
            }
        }
        JumpStates::Unjumpable => {
            if wall_sensor.down {
                jumper.state = JumpStates::Jumpable;
            }
        }
    }

    if keyboard_input.pressed(KeyCode::Down) {
    }

    match &attacker.state {
        AttackStates::CanAttack => {
            if keyboard_input.pressed(KeyCode::V) {
                let hammer = HammerBundle::new( 
                    Vec2 {x:0.0, y:0.4}, HAMMER_SIZE, HAMMER_SHAFT_SIZE,
                    0.0, HAMMER_SWING_TIME, 4.0);
                let shaft = commands.spawn(hammer.0).id();
                let head = commands.spawn(hammer.1).id();
                commands.entity(player).push_children(&[shaft]);
                commands.entity(shaft).push_children(&[head]);
                attacker.state = AttackStates::Attacking(Timer::from_seconds(HAMMER_SWING_TIME, TimerMode::Once), shaft);
             }
        }
        AttackStates::Attacking(timer, entity) => {
            if timer.finished() {
                commands.entity(*entity).despawn_recursive();
                attacker.state = AttackStates::NoAttack(Timer::from_seconds(HAMMER_SWING_TIME, TimerMode::Once));
            }
        }
        AttackStates::NoAttack(timer) => {
            if timer.finished() {
                attacker.state = AttackStates::CanAttack;
            }
        }
    }
    println!("move player system end");

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
