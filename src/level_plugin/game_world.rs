use bevy::prelude::*;
use crate::prelude::DEFAULT_PIXELES_PER_SCREEN_BOTTOM;

pub const METERS_PER_SCREEN_BOTTOM: f32 = 40.0;

#[derive(Component, Default)]
pub struct GameWorld;

#[derive(Bundle, Default)]
pub struct GameWorldInfo {
    pub world: GameWorld,
    pub space: SpatialBundle,
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
