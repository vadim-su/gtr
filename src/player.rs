use bevy::{input::ButtonInput, prelude::*, time::Time};

use crate::{
    game::GameState,
    unit::{move_unit, UnitBundle, UnitType},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0));
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        app.add_systems(
            FixedUpdate,
            roach_movement_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_player);
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let unit_type = UnitType {
        name: "Roach".to_string(),
        texture_path: "textures/roach.png".to_string(),
        base_speed: 1000.0,
        base_rotation_speed: f32::to_radians(360.0),
        collider_radius: 100.0,
    };

    commands.spawn((
        UnitBundle::new(
            unit_type,
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            &asset_server,
        ),
        Player,
    ));
}

pub fn roach_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &UnitType, &mut Transform)>,
) {
    let (_, unit_type, transform) = query.single_mut();
    let mut movement_factor = 0.0;
    let mut rotation_factor = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement_factor -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.0;
    }

    move_unit(
        transform,
        movement_factor,
        rotation_factor,
        time.delta_seconds(),
        unit_type,
    );
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    let player_entity = query.single();
    commands.entity(player_entity).despawn();
}
