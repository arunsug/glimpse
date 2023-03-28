mod player;
mod game_world;
mod physics;
mod walls;

use bevy::prelude::*;

use crate::prelude::*;
use self::player::*;
use self::game_world::*;
use self::physics::*;
use self::walls::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_level.in_schedule(OnEnter(GlimpseState::GameRunning)))
            .add_system(move_player.in_set(OnUpdate(GlimpseState::GameRunning)))
            .add_event::<AccelerateEntityEvent>()
            .add_event::<VelocitateEntityEvent>()
            .insert_resource(FixedTime::new_from_secs(PHYSICS_TIME_STEP)) 
            // TODO i think we can avoid chaining everythign some stuff can be in parallel
            /* .add_systems((handle_acceleration_events, apply_gravity, apply_resistance, apply_friction).chain().in_set(PhysicsSet::ApplyForces)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((apply_accel, handle_velocity_events).chain().in_set(PhysicsSet::ApplyAcceleration)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_system(.in_set(PhysicsSet::CastedCollisionDetection)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((apply_velocity, apply_position_to_transform).chain().in_set(PhysicsSet::ApplyVelocity)
                .in_schedule(CoreSchedule::FixedUpdate))
            //.add_system(apply.in_set(PhysicsSet::CollisionDetection))
            .configure_sets((PhysicsSet::ApplyForces, PhysicsSet::ApplyAcceleration, 
                PhysicsSet::CastedCollisionDetection, PhysicsSet::ApplyVelocity, 
                PhysicsSet::CollisionDection).chain())*/

            // we should be using the above code to schedule our physics system but it doesn't work
            // the acceleartion sometimes happens after casting which breaks teh whole thing
            .add_systems(
                (handle_acceleration_events, apply_gravity, apply_resistance, apply_friction, // apply forces (aka set aceeleration)
                apply_accel, handle_velocity_events, // apply acceleration (aka set velocity)
                handle_wall_collisions, // hanlde casting for collisions
                apply_velocity, apply_position_to_transform, //apply velocity (aka set positions) 
                ).chain().in_set(PhysicsSet::CollisionDection).in_schedule(CoreSchedule::FixedUpdate))
            //.configure_sets(Physics::ApplyForces)
            .add_system(cleanup_level.in_schedule(OnExit(GlimpseState::GameRunning)));

        #[cfg(debug_assertions)]
        app.add_system(debug_player.in_set(OnUpdate(GlimpseState::GameRunning)));

    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum LevelState {
    #[default]
    NoLevel,
    Level1,
}

fn setup_level(mut commands: Commands, query: Query<Entity, With<GlimpseWindow>>) {
    let world = commands.spawn(GameWorldInfo::new(Vec2{x:0.0,y:0.0}, 1.0)).id();
    let player = commands.spawn(PlayerBundle::new(Vec2 {x:0.0, y:5.0}, Vec2 {x:0.4, y:0.7})).id();
    let baby = commands.spawn(
            SpriteBundle {
            transform: Transform {
                translation: Vec3 { x: 1.0, y: 1.0, z: 0.0 },
                scale: Vec3 { x: 1.0, y: 1.0, z: 1.0 },
                ..default()
            },
            sprite: Sprite {
                color: DEBUG_COLOR,
                custom_size: Some(Vec2::new(0.3, 1.0)),
                ..default()
            },
            ..default()}
        ).id();
    let wall = commands.spawn(WallBundle::new(Vec2 {x: -3.0, y:-4.0}, Vec2 {x:10.0 , y:0.5})).id();
    let wall1 = commands.spawn(WallBundle::new(Vec2 {x: -6.0, y:1.0}, Vec2 {x:0.5 , y:8.5})).id();
    let wall2 = commands.spawn(WallBundle::new(Vec2 {x: 3.0, y:0.0}, Vec2 {x:0.5 , y:7.5})).id();

    let window = query.get_single().unwrap();
    commands.entity(window).push_children(&[world]);
    commands.entity(player).push_children(&[baby]);
    commands.entity(world).push_children(&[player]);
    commands.entity(world).push_children(&[wall, wall1, wall2]);
}

fn cleanup_level(
    mut commands: Commands,
    query: Query<Entity, With<GameWorld>>,
) {
    let world = query.single();
    commands.entity(world).despawn_recursive();
}
