use bevy::prelude::*;

use crate::{
    game::{GameState, Score, BOUNDS},
    player::Player,
    unit::{move_unit, UnitBundle, UnitType},
};
use rand;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(
            FixedUpdate,
            (
                enemy_ai_system,
                enemy_spawn_system,
                collision_detection_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_enemies);
    }
}

#[derive(Component)]
struct Enemy;

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, position: Vec3) {
    let unit_type = UnitType {
        name: "Spitter".to_string(),
        texture_path: "textures/spitter.png".to_string(),
        base_speed: 500.0,
        base_rotation_speed: f32::to_radians(360.0),
        collider_radius: 20.0,
    };

    let transform = Transform {
        translation: position,
        scale: Vec3::splat(0.5),
        ..Default::default()
    };

    commands.spawn((UnitBundle::new(unit_type, transform, &asset_server), Enemy));
    println!("Enemy spawned at {:?}", position);
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

fn enemy_spawn_system(
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        let spawn_bounds = BOUNDS / 4.0;
        let x = rand::random::<f32>() * spawn_bounds.x - spawn_bounds.x / 2.0;
        let y = rand::random::<f32>() * spawn_bounds.y - spawn_bounds.y / 2.0;
        spawn_enemy(commands, asset_server, Vec3::new(x, y, 0.0));
    }
}

fn enemy_ai_system(time: Res<Time>, mut query: Query<(&Enemy, &UnitType, &mut Transform)>) {
    for (_, unit_type, mut transform) in query.iter_mut() {
        if transform.translation.x.abs() >= BOUNDS.x / 2.0
            || transform.translation.y.abs() >= BOUNDS.y / 2.0
        {
            transform.rotate_z(std::f32::consts::PI);
        }

        let movement_factor = 1.0;
        let rotation_factor = rand::random::<f32>() * 2.0 - 1.0;

        move_unit(
            transform,
            movement_factor,
            rotation_factor,
            time.delta_seconds(),
            unit_type,
        );
    }
}

fn despawn_enemies(mut commands: Commands, query: Query<(Entity, &Enemy)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn collision_detection_system(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_query: Query<(&Transform, &UnitType), With<Player>>,
    enemies_query: Query<(Entity, &Transform, &UnitType), With<Enemy>>,
) {
    let (player_transform, player_unit_type) = player_query.single();
    for (enemy_entity, enemy_transform, enemy_unit_type) in enemies_query.iter() {
        if collide(
            player_unit_type.collider_radius as u32,
            enemy_unit_type.collider_radius as u32,
            player_transform.translation,
            enemy_transform.translation,
        ) {
            commands.entity(enemy_entity).despawn();
            score.0 += 1;
        }
    }
}

fn collide(collider1: u32, collider2: u32, pos1: Vec3, pos2: Vec3) -> bool {
    let collision_x = (pos1.x - pos2.x).abs() < collider1 as f32 + collider2 as f32;
    let collision_y = (pos1.y - pos2.y).abs() < collider1 as f32 + collider2 as f32;
    return collision_x && collision_y;
}
