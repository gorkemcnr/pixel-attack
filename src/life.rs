use bevy::{prelude::{Component, Query, With, Commands, EventReader, Entity, AssetServer, Res, ResMut, Assets, Vec2, Transform, default, Without}, sprite::{TextureAtlas, SpriteSheetBundle, TextureAtlasSprite}};
use bevy_parallax::ParallaxCameraComponent;

use crate::vehicle::VehicleHitEvent;

#[derive(Component, Clone, Copy)]
pub struct Life;

pub fn vehicle_hit_handle(
    mut commands: Commands,
    vehicle_hit_event_reader: EventReader<VehicleHitEvent>,
    life_query: Query<Entity, With<Life>>,
) {
    if vehicle_hit_event_reader.is_empty() || life_query.is_empty() {
        return;
    }

    if let Some(entity) = life_query.iter().next() {
        vehicle_hit_event_reader.clear();
        commands.entity(entity).despawn();
    }
}

pub fn move_life(
    background_query: Query<&Transform, (With<ParallaxCameraComponent>,  Without<Life>)>,
    mut life_query: Query<&mut Transform, (With<Life>, Without<ParallaxCameraComponent>)>,
) {
    
    let background_transform = background_query.single();
    let mut previous_x_position = background_transform.translation.x+250.0;

    for mut transform in life_query.iter_mut() {
        transform.translation.x = previous_x_position + 90.0;
        previous_x_position = transform.translation.x;
    }
}

pub fn create_life(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Vec<(bevy::prelude::SpriteSheetBundle, Life)> {
    let plasma_texture_handle = asset_server.load("vehicle.png");
    let plasma_texture_atlas = TextureAtlas::from_grid(
        plasma_texture_handle,
        Vec2::new(176.0, 96.0),
        1,
        1,
        None,
        None,
    );
    let plasma_texture_atlas_handle = texture_atlases.add(plasma_texture_atlas);

    let mut bundles = Vec::<(SpriteSheetBundle, Life)>::new();

    bundles.push (
        (bevy::prelude::SpriteSheetBundle {
            texture_atlas: plasma_texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(88.0, 48.0)),
                ..TextureAtlasSprite::default()
            },
            transform: Transform::from_xyz(400.0, 300.0, 3.0),
            ..default()
        },
        Life)
    );

    bundles.push (
        (bevy::prelude::SpriteSheetBundle {
            texture_atlas: plasma_texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(88.0, 48.0)),
                ..TextureAtlasSprite::default()
            },
            transform: Transform::from_xyz(500.0, 300.0, 3.0),
            ..default()
        },
        Life)
    );

    bundles.push (
        (bevy::prelude::SpriteSheetBundle {
            texture_atlas: plasma_texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(88.0, 48.0)),
                ..TextureAtlasSprite::default()
            },
            transform: Transform::from_xyz(600.0, 300.0, 3.0),
            ..default()
        },
        Life)
    );

    return bundles;

}

