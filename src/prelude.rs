use bevy::{prelude::*, window::WindowResized};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GlimpseState {
    MainMenu,
    #[default]
    GameRunning,
    Paused,
    Inventory,
    PickFile,
}

#[derive(Component, Default)]
pub struct GlimpseWindow;

#[derive(Bundle, Default)]
pub struct WindowBundle {
    pub window: GlimpseWindow,
    pub space: SpatialBundle,
}

pub const WINDOW_RATIO: f32 = 2.0;
pub const DEFAULT_PIXELES_PER_SCREEN_BOTTOM: f32 = 1000.0; 

impl WindowBundle {
    pub fn new() -> WindowBundle {
        WindowBundle {
            window: GlimpseWindow,
            space: SpatialBundle {
                transform: Transform {
                    translation: Vec3 {x:0.0 , y:0.0, z:0.0},
                    scale: Vec3 {x:1.0 , y:1.0, z:1.0},
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            ..default()
        }
    }
}

pub fn handle_window_resize(
    mut ev_window_resize: EventReader<WindowResized>, 
    mut query: Query<&mut Transform, With<GlimpseWindow>>
) {
    let mut x_pixels: f32 = 0.0; 
    for ev in ev_window_resize.iter() {
        x_pixels = if WINDOW_RATIO*ev.height < ev.width {
            WINDOW_RATIO*ev.height
        } else {
            ev.width
        };
    }
    if x_pixels > 0.001 {
        let window_scale = x_pixels / DEFAULT_PIXELES_PER_SCREEN_BOTTOM;
        let mut window_transform = query.get_single_mut().unwrap();
        window_transform.scale = Vec3 {x:window_scale , y:window_scale, z:1.0};
    }
}

pub const DEBUG_COLOR: Color = Color::rgb(0.9, 0.1, 0.1);
