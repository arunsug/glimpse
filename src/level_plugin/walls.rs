use bevy::prelude::*;
use super::physics::*;

#[derive(Component, Default)]
pub struct Wall;

#[derive(Component, Default)]
pub struct WallCollider;

#[derive(Component, Default)]
pub struct WallSensor {
    pub left: bool,
    pub right: bool,
    pub down: bool
}

#[derive(Component, Default)]
pub struct GroundSensor(bool);

const WALL_COLOR: Color = Color::rgb(0.8, 0.6, 0.0);

#[derive(Bundle, Default)]
pub struct WallBundle {
    pub wall: Wall,
    pub sprite_bundle: SpriteBundle,
    pub position: Position,
    pub shape: Shape,
}

impl WallBundle {
    pub fn new(pos: Vec2, size: Vec2) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle { 
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            shape: Shape::Rect(size),
            position: Position(pos),
            ..default()
        }
    }
}



// TODO Theorically you could move this into the physics system as a solid_immovable object or something
// but I dont' knwo if that lvel of abstraction is necessary.
pub fn handle_wall_collisions(
    wall_query: Query<(&Position, &Shape), (With<Wall>, Without<WallCollider>)>,
    mut wall_collider_query: Query<(&mut Position, &Shape, &mut Velocity, Option<&mut WallSensor>), 
        (With<WallCollider>, Without<Wall>)>
) {
    let zero_velocity = Velocity(Vec2::ZERO);
    for (mut col_pos, col_shape, mut col_vel, mut wall_sensor) in wall_collider_query.iter_mut() {
        if let Some(sensor) = wall_sensor.as_mut() {
            sensor.left = false;
            sensor.right = false;
            sensor.down = false;
        }
        for (wall_pos, wall_shape) in wall_query.iter() {
            match (&col_shape, &wall_shape) {
                (Shape::Rect(col_size), Shape::Rect(wall_size)) => {
                    // Calculat the upper left position for easier fucntion calcs
                    let col_upper_left = Vec2 {x: col_pos.0.x - col_size.x*0.5, y: col_pos.0.y + col_size.y*0.5};
                    let wall_upper_left = Vec2 {x: wall_pos.0.x - wall_size.x*0.5, y: wall_pos.0.y + wall_size.y*0.5};
                    let inter_angle = rectangles_casted_collision(
                        &col_upper_left, &col_size, &col_vel.0, 
                        &wall_upper_left, &wall_size, &zero_velocity.0);
                    // If the platform is moving then the position setting won't work
                    // TODO handle th cast casting better
                    if let Some(angle) = inter_angle {
                        let offset = (*col_size + *wall_size) * 0.5;
                        if (3.0*std::f32::consts::PI / 2.0) - MU <= angle {    
                            col_pos.0.y = wall_pos.0.y + offset.y +  MU;
                            if let Some(sensor) = wall_sensor.as_mut() {
                                sensor.down = true;
                            }
                        } else if std::f32::consts::PI - MU <= angle {
                            col_pos.0.x = wall_pos.0.x + offset.x + MU;
                            if let Some(sensor) = wall_sensor.as_mut() {
                                sensor.left = true;
                            }
                        } else if (std::f32::consts::PI / 2.0) - MU <= angle {
                            col_pos.0.y = wall_pos.0.y - offset.y - MU;
                        } else {
                            col_pos.0.x = wall_pos.0.x - offset.x - MU;
                            if let Some(sensor) = wall_sensor.as_mut() {
                                sensor.right = true;
                            }
                        }
                        if col_vel.0.length() > MU {
                            let mut angle_vec = Vec2::from_angle(angle);
                            angle_vec = angle_vec * col_vel.0.abs();
                            col_vel.0 -= angle_vec;
                        }
                    }
                }
                (_, _) => {
                    error!("Unhandled wall coollison");
                }
            };
        }
    } 
}