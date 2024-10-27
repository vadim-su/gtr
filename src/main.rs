mod enemies;
mod game;
mod player;
mod unit;

use bevy::prelude::*;
use bevy::{
    app::{App, Startup},
    DefaultPlugins,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((game::GamePlugin, player::PlayerPlugin, enemies::EnemyPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
