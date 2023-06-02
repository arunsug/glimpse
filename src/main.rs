pub mod level_plugin;
pub mod prelude;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;

use crate::prelude::*;
use crate::level_plugin::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

fn main() {
    App::new()
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920.0, 1080.0),
            medium: Vec2::new(800.0, 600.0),
            small: Vec2::new(640.0, 360.0),
        })
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::ERROR,
            filter: "glimpse=debug".into(),
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state::<GlimpseState>()
        .add_plugin(LevelPlugin)
        .add_startup_system(setup)
        .add_system(toggle_resolution)
        .add_system(handle_window_resize)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let window = commands.spawn(WindowBundle::new()).id();
    let wall = commands.spawn(
        SpriteBundle {
            transform: Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                ..default()
            },
            sprite: Sprite {
                color: BACKGROUND_COLOR,
                custom_size: Some(Vec2::new(1000.0, 500.0)),
                ..default()
            },
        ..default()
    }).id();
    commands.entity(window).push_children(&[wall]);
}

#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}
