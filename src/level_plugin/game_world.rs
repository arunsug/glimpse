use bevy::prelude::*;
use super::physics::*;
use crate::prelude::DEFAULT_PIXELES_PER_SCREEN_BOTTOM;

pub const METERS_PER_SCREEN_BOTTOM: f32 = 40.0;

#[derive(Component, Default)]
pub struct GameWorld;

#[derive(Bundle, Default)]
pub struct GameWorldInfo {
    pub world: GameWorld,
    pub space: SpatialBundle,
    pub position: Position,
    pub rotation: Rotation,
    pub transform: TwoDimTrans,
    pub global_position: GlobalPosition,
    pub global_rotations: GlobalRotation,
}

impl GameWorldInfo {
    pub fn new(position: Vec2, zoom: f32) -> GameWorldInfo {
        GameWorldInfo {
            world: GameWorld,
            space: SpatialBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3 { 
                        x:zoom * DEFAULT_PIXELES_PER_SCREEN_BOTTOM/METERS_PER_SCREEN_BOTTOM, 
                        y:zoom * DEFAULT_PIXELES_PER_SCREEN_BOTTOM/METERS_PER_SCREEN_BOTTOM,
                        z:1.0
                    },
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            ..default()
        }
    }
}

// update our 2d tranforms starting from the world transform
pub fn propagate_transform(
    world_query: Query<Entity, With<GameWorld>>, 
    child_query: Query<&Children, With<TwoDimTrans>>,
    mut transform_query: Query<(&mut TwoDimTrans, &mut GlobalRotation, &mut GlobalPosition, &Position, &Rotation)>
) {
    // stack to use for DFS down the transform tree
    let mut stack = vec![world_query.single()];
    while stack.len() > 0 {

        // get the top element of the stack extract the value we need from it
        let cur = stack.pop().unwrap();
        if let Ok(children) = child_query.get(cur) {
            let (trans, rot, _, _, _) = transform_query.get(cur).unwrap();
            let trans = trans.clone();
            let rot = rot.0;

            for child in children.iter() {
                // get the child transform values
                if let Ok((mut child_trans, mut child_rot, 
                    mut child_pos, local_pos, local_rot)) 
                    = transform_query.get_mut(*child) {
                    // generate the local transform
                    child_trans.0 = Mat3::from_scale_angle_translation(Vec2::ONE, local_rot.0, local_pos.0);
                    // multiply the teh parent tranfrom
                    child_trans.0 = trans.0*child_trans.0;
                    // tranform the local point by the child point
                    child_pos.0 = trans.0.transform_point2(local_pos.0);
                    // transform the local rotation
                    child_rot.0 = rot+local_rot.0;
                    // add the child to the stack
                    stack.push(*child);
                    
                }
            }
        }
    }
}

/*
heres is our orginization
file for eahc enitity named after the entiyt contains components and entity specifc systems

_plugin.rs files for plugins

each level will be a plugin and have a system set of all systems for the level and seperate state

Collider plugin

Phisics will be handled on a enity basis

Level plugin
    all systmes
    GameWorld Entity
        all game bits as child entitys for quick despawn
        HUD a child enitity?

Inventory State

Menu State

Settings State





 */
