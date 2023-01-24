use std::time::Duration;

use bevy::{
    prelude::{Component, Input, KeyCode, Query, Res, Transform, With, Without},
    sprite::TextureAtlasSprite,
    time::Time,
};
use bevy_parallax::ParallaxCameraComponent;

use crate::{animation::{animate, AnimationIndices, AnimationTimer}};

#[derive(Component)]
pub struct Vehicle;

pub struct VehicleHitEvent;

pub fn animate_vehicle(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &AnimationIndices,
        ),
        With<Vehicle>,
    >,
) {
    for (mut timer, mut sprite, animation_indices) in query.iter_mut() {
        animate(&time, &mut timer, &mut sprite, animation_indices);
    }
}

pub fn move_vehicle(
    keyboard_input: Res<Input<KeyCode>>,
    background_query: Query<&Transform, (With<ParallaxCameraComponent>,  Without<Vehicle>)>,
    mut vehicle_query: Query<(&mut Transform, &mut AnimationTimer), (With<Vehicle>, Without<ParallaxCameraComponent>)>,
) {

    let background_transform = background_query.single();
    
    for (mut transform, mut animation_timer) in vehicle_query.iter_mut() {
        let mut direction_y = 0.0;

        transform.translation.x = background_transform.translation.x-450.0;

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 5.0;
        } else if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 5.0;
        }
        else {
            return;
        }

        animation_timer.tick(Duration::from_secs_f32(1.5));

        if animation_timer.just_finished() {
            let vehicle_position_y = transform.translation.y + direction_y;

            transform.translation.y = vehicle_position_y.clamp(-300.0, 300.0);
        }
    }
}
