mod player;
mod game_world;
mod physics;
mod walls;
mod enemy;
mod weapon;

use bevy::ecs::schedule::{LogLevel, ScheduleBuildSettings};
use bevy::prelude::*;

use crate::prelude::*;
use self::player::*;
use self::enemy::*;
use self::game_world::*;
use self::physics::*;
use self::walls::*;
use self::weapon::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_level.in_schedule(OnEnter(GlimpseState::GameRunning)))
            .add_system(move_player.in_set(OnUpdate(GlimpseState::GameRunning)))
            
            //.add_event::<AccelerateEntityEvent>()
            //.add_event::<VelocitateEntityEvent>()

            .insert_resource(FixedTime::new_from_secs(PHYSICS_TIME_STEP)) // set the time step for the CorSchedulei
            // TODO i think we can avoid chaining everythign some stuff can be in parallel
            .add_systems((apply_acceleration_adjustments, apply_gravity, apply_resistance, apply_friction).in_set(PhysicsSet::ApplyForces)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_system(apply_acceleration_override.in_set(PhysicsSet::OverrideAcceleration).in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((apply_accel, apply_velocity_adjustments).in_set(PhysicsSet::ApplyAcceleration)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((apply_velocity_override, apply_angular_velocity_override).in_set(PhysicsSet::OverrideVelocity)
                    .in_schedule(CoreSchedule::FixedUpdate))
            .add_system(handle_wall_collisions.in_set(PhysicsSet::CastedCollisionDetection)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((apply_velocity, apply_angular_velocity).in_set(PhysicsSet::ApplyVelocity)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_systems((apply_position_to_transform,apply_rotation_to_transform, propagate_transform).in_set(PhysicsSet::ModifyTransform)
                .in_schedule(CoreSchedule::FixedUpdate))
            .add_system(narrow_phase.in_set(PhysicsSet::CollisionDetection))
            .configure_set(PhysicsSet::ApplyForces.before(PhysicsSet::OverrideAcceleration))
            .configure_set(PhysicsSet::OverrideAcceleration.before(PhysicsSet::ApplyAcceleration))
            .configure_set(PhysicsSet::ApplyAcceleration.before(PhysicsSet::OverrideVelocity))
            .configure_set(PhysicsSet::OverrideVelocity.before(PhysicsSet::CastedCollisionDetection))
            .configure_set(PhysicsSet::CastedCollisionDetection.before(PhysicsSet::ApplyVelocity))
            .configure_set(PhysicsSet::ApplyVelocity.before(PhysicsSet::ModifyTransform))
            .configure_set(PhysicsSet::ModifyTransform.before(PhysicsSet::CollisionDetection))
            .edit_schedule(CoreSchedule::FixedUpdate, |schedule| {
                schedule.set_build_settings(ScheduleBuildSettings {
                    ambiguity_detection: LogLevel::Warn,
                    ..default()
                });
            })
            /* .configure_sets((PhysicsSet::ApplyForces, PhysicsSet::OverrideAcceleration, PhysicsSet::ApplyAcceleration, 
                PhysicsSet::OverrideVelocity, PhysicsSet::CastedCollisionDetection, PhysicsSet::ApplyVelocity, 
                PhysicsSet::ModifyTransform, PhysicsSet::CollisionDetection).chain())*/

            // we should be using the above code to schedule our physics system but it doesn't work
            // the acceleartion sometimes happens after casting which breaks teh whole thing
            /* .add_systems((apply_acceleration_adjustments, apply_gravity, apply_resistance, apply_friction, apply_acceleration_override, // apply forces (aka set aceeleration)
                apply_velocity_adjustments, apply_accel, apply_velocity_override, apply_angular_velocity_override,// apply acceleration (aka set velocity)
                handle_wall_collisions, // hanlde casting for collisions
                apply_velocity, apply_angular_velocity, //apply velocity (aka set positions) 
                apply_position_to_transform, apply_rotation_to_transform, 
                propagate_transform, // modify transforms
                narrow_phase).chain().in_set(PhysicsSet::CollisionDection).in_schedule(CoreSchedule::FixedUpdate))*/
                // TODO the issue is that there are just to many things in the chain i need to break it up apparently
            .add_systems((tick_jump_times, tick_attack_times).in_set(PhysicsSet::CollisionDetection))
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
    println!("setting up level start");
    let world = commands.spawn(GameWorldInfo::new(Vec2{x:0.0,y:0.0}, 1.0)).id();
    let player = commands.spawn(PlayerBundle::new(Vec2 {x:0.0, y:5.0}, Vec2 {x:1.0, y:1.7})).id();
    let enemy = commands.spawn(EnemyBundle::new(Vec2 {x:-1.0, y:5.0}, Vec2 {x:1.0, y:1.7})).id();
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
    let wall = commands.spawn(WallBundle::new(Vec2 {x: -3.0, y:-8.0}, Vec2 {x:20.0 , y:0.5})).id();
    let wall1 = commands.spawn(WallBundle::new(Vec2 {x: -6.0, y:1.0}, Vec2 {x:0.5 , y:8.5})).id();
    let wall2 = commands.spawn(WallBundle::new(Vec2 {x: 3.0, y:0.0}, Vec2 {x:0.5 , y:7.5})).id();
    let wall3 = commands.spawn(WallBundle::new(Vec2 {x: 7.0, y:-3.0}, Vec2 {x:4.0 , y:0.5})).id();
    let wall4 = commands.spawn(WallBundle::new(Vec2 {x: -11.0, y:-4.0}, Vec2 {x:5.0 , y:0.5})).id();
    let wall5 = commands.spawn(WallBundle::new(Vec2 {x: -14.0, y:0.0}, Vec2 {x:5.0 , y:0.5})).id();
    let wall6 = commands.spawn(WallBundle::new(Vec2 {x: -14.0, y:2.0}, Vec2 {x:1.0 , y:0.5})).id();
    let wall7 = commands.spawn(WallBundle::new(Vec2 {x: -16.0, y:4.0}, Vec2 {x:1.0 , y:0.5})).id();
    let wall8 = commands.spawn(WallBundle::new(Vec2 {x: -18.0, y:6.0}, Vec2 {x:1.0 , y:0.5})).id();
    let wall9 = commands.spawn(WallBundle::new(Vec2 {x: -20.0, y:8.0}, Vec2 {x:1.0 , y:0.5})).id();

    let window = query.get_single().unwrap();
    commands.entity(window).push_children(&[world]);
    commands.entity(player).push_children(&[baby]);
    commands.entity(world).push_children(&[player]);
    commands.entity(world).push_children(&[enemy]);
    commands.entity(world).push_children(&[wall, wall1, wall2, wall3, wall4, wall5, wall6, wall7, wall8, wall9]);
    println!("setting up level end");
}

fn cleanup_level(
    mut commands: Commands,
    query: Query<Entity, With<GameWorld>>,
) {
    let world = query.single();
    commands.entity(world).despawn_recursive();
}

