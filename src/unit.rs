use bevy::{
    asset::AssetServer,
    math::Vec3,
    prelude::{Bundle, Component, Mut, Res, Transform},
    sprite::SpriteBundle,
};

use crate::game::BOUNDS;

#[derive(Component)]
pub struct UnitType {
    pub name: String,
    pub texture_path: String,
    pub base_speed: f32,
    pub base_rotation_speed: f32,
    pub collider_radius: f32,
}

#[derive(Component)]
pub struct Health(i32);

#[derive(Bundle)]
pub struct UnitBundle {
    unit_type: UnitType,
    sprite: SpriteBundle,
}

impl UnitBundle {
    pub fn new(
        unit_type: UnitType,
        transform: Transform,
        asset_server: &Res<AssetServer>,
    ) -> UnitBundle {
        let texture_handle = asset_server.load(&unit_type.texture_path);

        let sprite = SpriteBundle {
            texture: texture_handle,
            transform,
            ..Default::default()
        };

        UnitBundle { unit_type, sprite }
    }
}

pub fn move_unit(
    mut transform: Mut<Transform>,
    movement_factor: f32,
    rotation_factor: f32,
    delta_time: f32,
    unit_type: &UnitType,
) {
    let rotation = rotation_factor * unit_type.base_rotation_speed * delta_time;
    let direction = transform.rotation * Vec3::Y;

    transform.rotate_z(rotation);

    let distance = movement_factor * unit_type.base_speed * delta_time;
    transform.translation += direction * distance;

    let bounds = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.clamp(-bounds, bounds);
}
