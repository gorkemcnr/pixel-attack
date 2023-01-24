use bevy_parallax::ParallaxCameraComponent;
use rand::{distributions::Uniform, prelude::Distribution};

use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, Entity, EventWriter, Query, Res, ResMut,
        Transform, Vec2, Vec3, With, Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
};

use crate::{
    animation::{animate, AnimationIndices, AnimationTimer},
    vehicle::{Vehicle, VehicleHitEvent},
};

#[derive(Component)]
pub struct Enemy {
    pub expected_position_y: f32,
}

pub fn animate_enemy(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &AnimationIndices,
        ),
        With<Enemy>,
    >,
) {
    for (mut timer, mut sprite, animation_indices) in query.iter_mut() {
        animate(&time, &mut timer, &mut sprite, animation_indices);
    }
}

pub fn move_enemy(
    mut commands: Commands,
    mut vehicle_hit_event_writer: EventWriter<VehicleHitEvent>,
    mut vehicle_query: Query<&Transform, (With<Vehicle>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut Enemy), (With<Enemy>, Without<Vehicle>)>,
) {
    let vehicle_transform = vehicle_query.single_mut();

    for (enemy_entity, mut enemy_transform, mut enemy) in enemy_query.iter_mut() {
        if enemy_transform.translation.x + 300.0 < vehicle_transform.translation.x {
            commands.entity(enemy_entity).despawn();
            return;
        }

        if is_collide_with_vehicle(vehicle_transform.translation, enemy_transform.translation) {
            vehicle_hit_event_writer.send(VehicleHitEvent);
            commands.entity(enemy_entity).despawn();
            return;
        }

        if enemy_transform.translation.y == enemy.expected_position_y {
            let mut rng = rand::thread_rng();
            let y_range = Uniform::from(-300..300);
            let y = y_range.sample(&mut rng);
            enemy.expected_position_y = f32::from((y - (y % 2)) as i16);
        }

        let direction_y = if enemy.expected_position_y > enemy_transform.translation.y {
            2.0
        } else {
            -2.0
        };

        let vehicle_position_y = enemy_transform.translation.y + direction_y;

        enemy_transform.translation.y = vehicle_position_y.clamp(-300.0, 300.0);
    }
}

fn is_collide_with_vehicle(vehicle_position: Vec3, enemy_position: Vec3) -> bool {
    if vehicle_position.x >= enemy_position.x - 130.0
        && vehicle_position.x <= enemy_position.x + 130.0
        && (vehicle_position.y >= enemy_position.y - 80.0
            && vehicle_position.y <= enemy_position.y + 80.0)
    {
        return true;
    }

    return false;
}

pub fn create_enemy_bundle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    background_query: Query<&Transform, With<ParallaxCameraComponent>>,
    enemy_query: Query<&Enemy, With<Enemy>>,
) {
    if !enemy_query.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..50);
    let value = range.sample(&mut rng);
    if ! value % 3 == 0 {
        return;
    }

    let background_transform = background_query.single();
    let position_x = background_transform.translation.x + 800.0;

    let enemy_texture_handle = asset_server.load("enemy.png");
    let enemy_texture_atlas = TextureAtlas::from_grid(
        enemy_texture_handle,
        Vec2::new(329.0, 160.0),
        3,
        1,
        None,
        None,
    );
    let enemy_texture_atlas_handle = texture_atlases.add(enemy_texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: enemy_texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                flip_x: true,
                custom_size: Some(Vec2::new(176.0, 96.0)),
                ..TextureAtlasSprite::default()
            },
            transform: Transform::from_xyz(position_x, -30.0, 4.0),
            ..SpriteSheetBundle::default()
        },
        AnimationIndices { first: 0, last: 2 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Enemy {
            expected_position_y: -30.0,
        },
    ));
}
