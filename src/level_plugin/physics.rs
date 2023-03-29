// TODO break this into a plugin
use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSet {
    ApplyForces,
    ApplyAcceleration,
    CastedCollisionDetection,
    ApplyVelocity,
    CollisionDection
}

pub const PHYSICS_TIME_STEP: f32 = 1.0 / 240.0;
pub const MU: f32 = 0.0000003;

#[derive(Component, Default)]
pub struct Gravity;

#[derive(Component, Default, Debug)]
pub struct Resistance(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Friction(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Acceleration(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct Position(pub Vec2);


#[derive(Component)]
pub enum Shape {
    Rect(Vec2),
    Circle(u32),
    Tri(Vec2, Vec2)
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Rect(Vec2::ONE)
    }
}

#[derive(Bundle, Default)]
pub struct Body {
    pub position: Position,
    pub shape: Shape
}


#[derive(Bundle, Default)]
pub struct BasePhysicsBundle {
    pub body: Body,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub resistance: Resistance,
    pub friction: Friction,
}

#[derive(Component, Default, Debug)]
pub struct OverrideVelocity(pub bool, pub Vec2);

#[derive(Component, Default, Debug)]
pub struct OverrideAcceleration(pub bool, pub Vec2);

#[derive(Component, Default, Debug)]
pub struct AdjustVelocity(pub Vec2);

#[derive(Component, Default, Debug)]
pub struct AdjustAcceleration(pub Vec2);


// TODO we could do this with events instead I think
#[derive(Bundle, Default)]
pub struct PhysicsControllerBundle {
    pub over_vel: OverrideVelocity,
    pub over_accel: OverrideAcceleration,
    pub adj_vel: AdjustVelocity,
    pub adj_acce: AdjustAcceleration
}

#[derive(Component)]
pub struct Jumper {
    pub is_jumping: bool,
    pub can_jump: bool,
    pub timer: Timer
}

pub fn apply_acceleration_adjustments(mut query: Query<(&mut Acceleration, &AdjustAcceleration)>) {
    for (mut accel, adjust) in query.iter_mut() {
        accel.0 += adjust.0;
    }
}

pub fn apply_velocity_adjustments(mut query: Query<(&mut Velocity, &AdjustVelocity)>) {
    for (mut vel, adjust) in query.iter_mut() {
        vel.0 += adjust.0;
    }
}

pub fn apply_acceleration_override(mut query: Query<(&mut Acceleration, &OverrideAcceleration)>) {
    for (mut accel, over) in query.iter_mut() {
        if over.0 {
            accel.0 = over.1;
        }
    }
}

pub fn apply_velocity_override(mut query: Query<(&mut Velocity, &OverrideVelocity)>) {
    for (mut vel, over) in query.iter_mut() {
        if over.0 {
            vel.0 = over.1;
        }
    }
}

const GRAVITY_VECTOR: Vec2 = Vec2 { x:0.0, y:-9.8 };
pub fn apply_gravity(mut query: Query<&mut Acceleration, With<Gravity>>) {
    for mut accel in query.iter_mut() {
        accel.0 += GRAVITY_VECTOR;
    }
}

// we are tyring to fix the drifting left bug
pub fn apply_resistance(mut query: Query<(&mut Acceleration, &Velocity, &Resistance)>) {
    for (mut accel, vel, resist) in query.iter_mut() {
        if vel.0.x.abs() > MU {
            accel.0.x -= vel.0.x * vel.0.x * resist.0.x * vel.0.signum().x;
        }
        if vel.0.y.abs() > MU {
            accel.0.y -= vel.0.y * vel.0.y * resist.0.y * vel.0.signum().y;
        }
    }
}

// TODO fix the drifitn gleft bug
pub fn apply_friction(mut query: Query<(&mut Acceleration, &Velocity, &Friction)>) {
    for (mut accel, vel, friction) in query.iter_mut() {
        if vel.0.x.abs() > MU {
            accel.0.x -= vel.0.signum().x * friction.0.x;
        }
        if vel.0.y.abs() > MU {
            accel.0.y -= vel.0.signum().y * friction.0.y;
        }
    }
}

pub fn apply_accel(mut query: Query<(&mut Acceleration, &mut Velocity)>){
    for (mut accel, mut vel) in query.iter_mut() {
        vel.0 += accel.0*PHYSICS_TIME_STEP;
        debug!("apply accel vel and accel {:?}, {:?}", vel, accel);
        accel.0 = Vec2::ZERO;
    } 
}

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Position)>) {
    for (vel, mut pos) in query.iter_mut() {
        pos.0 += vel.0*PHYSICS_TIME_STEP;
        debug!("apply velocity vel and pos {:?}, {:?}\n", vel, pos);
    }
}

pub fn apply_position_to_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut trans) in query.iter_mut() {
        trans.translation = pos.0.extend(0.0);
    }
}

pub fn tick_jump_times(mut query: Query<&mut Jumper>) {
    for mut jumper in query.iter_mut() {
        jumper.timer.tick (Duration::from_secs(PHYSICS_TIME_STEP as u64));
    }
}

pub fn rectangles_casted_collision(
    pos1: &Vec2, size1: &Vec2, vel1: &Vec2, 
    pos2: &Vec2, size2: &Vec2, vel2: &Vec2
) -> Option<f32> {
    let cast1 = Vec2 { x:pos1.x + vel1.x * PHYSICS_TIME_STEP, y:pos1.y + vel1.y * PHYSICS_TIME_STEP };
    let cast2 = Vec2 { x:pos2.x + vel2.x * PHYSICS_TIME_STEP, y:pos2.y + vel2.y * PHYSICS_TIME_STEP };
    if cast1.x <= cast2.x + size2.x && 
        cast1.x + size1.x >= cast2.x &&
        cast1.y >= cast2.y - size2.y && 
        cast1.y - size1.y <= cast2.y 
    {
        let horizontal = if pos1.x >= pos2.x + size2.x {
            Some((std::f32::consts::PI, (pos2.x + size2.x - pos1.x) / (vel1.x - vel2.x)))            
        } else if pos1.x + size1.x <= pos2.x {
            Some((0.0, (pos2.x - (pos1.x + size1.x)) / (vel1.x - vel2.x)))
        } else {
            None
        };

        let vertical = if pos1.y <= pos2.y - size2.y {
            Some((std::f32::consts::PI/2.0, (pos2.y - size2.y - pos1.y) / (vel1.y - vel2.y)))
        } else if pos1.y - size1.y >= pos2.y {
            Some((std::f32::consts::PI * 3.0 / 2.0, (pos2.y - (pos1.y - size1.y)) / (vel1.x - vel2.x)))
        } else {
            None
        };

        match (horizontal, vertical) {
            (Some((h_ang, tx)), Some((v_ang, ty))) => {
                if tx < ty {
                    Some(h_ang)
                } else {
                    Some(v_ang)
                }
            }
            (None, Some((v_ang, ty))) => Some(v_ang),
            (Some((h_ang, tx)), None) => Some(h_ang),
            (None, None) => panic!("we collided but can't find the angel") 
        }
        
    } else {
        None
    }
}

// lets check if they are on screen for now
// so we can ignore things that aren't
fn in_active_range(bodies: &Vec<(&Position, &Shape)>) {

}

// the concept here is to decte paris to check
fn broad_phase() {
    
}

// here is where we check the pairs
fn narow_phase() {

}

