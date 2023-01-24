use bevy::{
    prelude::{
        default, AssetServer, Assets, Commands, Component, Entity, Input, KeyCode, Query, Res,
        ResMut, Transform, Vec2, Vec3, With, Without, Audio, PlaybackSettings,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};

use crate::{
    animation::{animate, AnimationIndices, AnimationTimer},
    enemy::Enemy,
    vehicle::Vehicle,
};

#[derive(Component)]
pub struct Plasma {
    pub starting_point_x: f32,
}

pub fn animate_plasma(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &AnimationIndices,
        ),
        With<Plasma>,
    >,
) {
    for (mut timer, mut sprite, animation_indices) in query.iter_mut() {
        animate(&time, &mut timer, &mut sprite, animation_indices);
    }
}

pub fn move_plasma(
    mut commands: Commands,
    mut plasma_query: Query<(&mut Transform, &Plasma, Entity), (With<Plasma>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Plasma>)>,
) {
    if plasma_query.is_empty() {
        return;
    }

    let (mut plasma_transform, plasma, plasma_entity) = plasma_query.single_mut();
    
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        if is_collide_with_enemy(plasma_transform.translation, enemy_transform.translation) {
            commands.entity(enemy_entity).despawn();
            commands.entity(plasma_entity).despawn();
            return;
        }
    }

    plasma_transform.translation.x += 20.0;

    if plasma_transform.translation.x >= plasma.starting_point_x + 1150.0 {
        commands.entity(plasma_entity).despawn();
    }
}

pub fn fire_plasma(
    mut commands: Commands,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    keyboard_input: Res<Input<KeyCode>>,
    plasma_query: Query<&Transform, With<Plasma>>,
    mut vehicle_query: Query<&Transform, With<Vehicle>>,
) {
    let transform = vehicle_query.single_mut();

    if !plasma_query.is_empty() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn(create_plasma_bundle(
            asset_server.clone(),
            texture_atlases,
            transform.translation.x + 80.0,
            transform.translation.y,
        ));
        let plasma_sound = asset_server.load("plasma.ogg");

        audio.play_with_settings(plasma_sound, PlaybackSettings::ONCE.with_volume(2.00));
    }
}

pub fn create_plasma_bundle(
    asset_server: AssetServer,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    x: f32,
    y: f32,
) -> (
    bevy::prelude::SpriteSheetBundle,
    AnimationIndices,
    AnimationTimer,
    Plasma,
) {
    let plasma_texture_handle = asset_server.load("plasma.png");
    let plasma_texture_atlas = TextureAtlas::from_grid(
        plasma_texture_handle,
        Vec2::new(48.0, 48.0),
        3,
        1,
        None,
        None,
    );
    let plasma_texture_atlas_handle = texture_atlases.add(plasma_texture_atlas);

    return (
        SpriteSheetBundle {
            texture_atlas: plasma_texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(x, y, 4.0),
            ..default()
        },
        AnimationIndices { first: 0, last: 2 },
        AnimationTimer(Timer::from_seconds(100.0, TimerMode::Once)),
        Plasma {
            starting_point_x: x,
        },
    );
}

fn is_collide_with_enemy(plasma_position: Vec3, enemy_position: Vec3) -> bool {
    if plasma_position.x >= enemy_position.x - 130.0
        && plasma_position.x <= enemy_position.x + 130.0
        && (plasma_position.y >= enemy_position.y - 50.0
            && plasma_position.y <= enemy_position.y + 50.0)
    {
        return true;
    }

    return false;
}
