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

#[derive(Component, Default, Debug)]
pub struct Rotation(pub f32);

#[derive(Component)]
pub enum Shape {
    Rect(Vec2),
    Circle(f32),
    Poly(Vec<Vec2>)
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Rect(Vec2::ONE)
    }
}

#[derive(Bundle, Default)]
pub struct Body {
    pub position: Position,
    pub rotation: Rotation,
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
pub struct OverrideVelocity(pub Option<f32>, pub Option<f32>);

#[derive(Component, Default, Debug)]
pub struct OverrideAcceleration(pub Option<f32>, pub Option<f32>);

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
        if let Some(x) = over.0 {
            accel.0.x = x;
        }
        if let Some(y) = over.1 {
            accel.0.y = y;
        }
    }
}

pub fn apply_velocity_override(mut query: Query<(&mut Velocity, &OverrideVelocity)>) {
    for (mut vel, over) in query.iter_mut() {
        if let Some(x) = over.0 {
            vel.0.x = x;
        }
        if let Some(y) = over.1 {
            vel.0.y = y;
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

pub fn apply_rotation_to_transform(mut query: Query<(&Rotation, &mut Transform)>) {
    for (rot, mut trans) in query.iter_mut() {
        trans.rotation = Quat::from_rotation_z(rot.0);
    }
}


//pos1 and pos2 are teh upper left corner of the rect
pub fn rectangles_casted_collision(
    pos1: &Vec2, size1: &Vec2, vel1: &Vec2, 
    pos2: &Vec2, size2: &Vec2, vel2: &Vec2
) -> Option<f32> {
    let cast1 = Vec2 { x:pos1.x + vel1.x * PHYSICS_TIME_STEP, y:pos1.y + vel1.y * PHYSICS_TIME_STEP };
    let cast2 = Vec2 { x:pos2.x + vel2.x * PHYSICS_TIME_STEP, y:pos2.y + vel2.y * PHYSICS_TIME_STEP };
    if aabb_collision(&cast1, size1, &cast2, size2) {
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

fn detect_collision_pair(
    pos1: &Vec2, shape1: &Shape, angle1: f32,
    pos2: &Vec2, shape2: &Shape, angle2: f32) -> bool 
{
    match (shape1, shape2) {
        (Shape::Rect(size1), Shape::Rect(size2)) => {
            if angle1.abs() < MU && angle2.abs() < MU {
                aabb_collision(pos1, size1, pos2, size2)
            } else {
                let points1 = generate_rectangle_points(pos1, size1, angle1);
                let points2 = generate_rectangle_points(pos2, size2, angle2);
                sat_collision(points1, points2)
            }
        }
        (Shape::Circle(radius1), Shape::Circle(radius2)) => {
            circles_collision(pos1, radius1, pos2, radius2)
        }
        (Shape::Circle(radius), Shape::Rect(size)) => {
            if angle2.abs() < MU {
                aabb_circle_collision(pos2, size, pos1, *radius)
            } else {
                let points = generate_rectangle_points(pos2, size, angle2);
                sat_circle_collision(points, pos1, *radius)
            }
        }
        (Shape::Rect(size), Shape::Circle(radius)) => {
            if angle1.abs() < MU {
                aabb_circle_collision(pos1, size, pos2, *radius)
            } else {
                let points = generate_rectangle_points(pos1, size, angle1);
                sat_circle_collision(points, pos2, *radius)
            }
        }
        (_,_) => {
            panic!("Unhandled collision");
        }
    }
}

// TODO Shoudl I inline these fucntions
// Check for a collision between two rectangles
fn circles_collision(pos1: &Vec2, radius1: &f32, pos2: &Vec2, radius2: &f32) -> bool {
    pos1.distance(*pos2) < radius1 + radius2
}

// Check for a collision between axis aligned bounding boxes (two rectangles with no rotation)
// takes in the upper left corner of the rect
fn aabb_collision(pos1: &Vec2, size1: &Vec2, pos2: &Vec2, size2: &Vec2) -> bool {
    pos1.x <= pos2.x + size2.x && 
        pos1.x + size1.x >= pos2.x &&
        pos1.y >= pos2.y - size2.y && 
        pos1.y - size1.y <= pos2.y
}

// Check for a collision between axis aligned bounding box and a circle
fn aabb_circle_collision(rect_pos: &Vec2, size: &Vec2, circ_pos: &Vec2, radius: f32) -> bool {
    let x_loc = if rect_pos.x > circ_pos.x {
        Some(rect_pos.x)
    } else if rect_pos.x + size.x < circ_pos.x {
        Some(rect_pos.x + size.x)
    } else {
        None
    };
    let y_loc = if rect_pos.y < circ_pos.y {
        Some(rect_pos.y)
    } else if rect_pos.y - size.y > circ_pos.y {
        Some(rect_pos.y - size.y)
    } else {
        None
    };
    match (x_loc, y_loc) {
        (Some(x), Some(y)) => Vec2::new(x, y).distance(*circ_pos) < radius,
        (Some(x), None) => (x - circ_pos.x).abs() < radius,
        (None, Some(y)) => (y - circ_pos.y).abs() < radius,
        (None, None) => true,
    }
}
// TODO
fn sat_collision(points1: Vec<Vec2>, points2: Vec<Vec2>) -> bool {
    false
}
// TODO
fn sat_circle_collision(points: Vec<Vec2>, circ_pos: &Vec2, radius: f32) -> bool {
    false
}

fn generate_rectangle_points(pos: &Vec2, size: &Vec2, angle: f32) -> Vec<Vec2> {
    let hori_offset = (size.x / 2.0) * Vec2::from_angle(angle).perp();
    let vert_offset = (size.y / 2.0) * Vec2::from_angle(angle).perp();
    vec![*pos - hori_offset + vert_offset,
        *pos + hori_offset + vert_offset,
        *pos + hori_offset - vert_offset,
        *pos - hori_offset - vert_offset] 
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

