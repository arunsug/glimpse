
use bevy::prelude::*;

use super::physics::*;
use super::walls::*;
use super::player::*;

#[derive(Component, Default)]
pub struct Enemy;

const ENEMY_COLOR: Color = Color::rgb(0.8, 0.3, 0.2);
const ENEMY_RESIST: Vec2 = Vec2 { x:1.0, y:0.05 };
const ENEMY_FRICTION: Vec2 = Vec2 { x:7.0, y:0.0 };

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub sprite: SpriteBundle,
    pub physics_controller: PhysicsControllerBundle,
    pub physics: BasePhysicsBundle,
    pub gravity: Gravity,
    pub wall_collider: WallCollider,
    pub wall_sensor: WallSensor,
    pub collider: Collider
}

impl EnemyBundle {
    pub fn new(position: Vec2, size: Vec2) -> EnemyBundle {
        EnemyBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3::ONE,
                    ..default()
                },
                sprite: Sprite {
                    color: ENEMY_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },
            physics: BasePhysicsBundle {
                resistance: Resistance(ENEMY_RESIST),
                friction: Friction(ENEMY_FRICTION),
                body: Body {
                    position: Position(position),
                    shape: Shape::Rect(size),
                    ..default() 
                },
                ..default()
            },
            ..default()
        }
    }
}


pub fn enemy_attack(query: Query<Entity, With<Enemy>>) {
    
}