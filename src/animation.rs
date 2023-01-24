use bevy::{time::{Time, Timer}, sprite::TextureAtlasSprite, prelude::{Res, Component, Deref, DerefMut}};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

pub fn animate(
    time: &Res<Time>,
    timer: &mut AnimationTimer,
    sprite: &mut TextureAtlasSprite,
    animation_indices: &AnimationIndices,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        sprite.index = (sprite.index + 1) % (animation_indices.last + 1);
    }
}