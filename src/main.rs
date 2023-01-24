mod animation;
mod enemy;
mod life;
mod plasma;
mod vehicle;

use animation::{AnimationIndices, AnimationTimer};
use bevy::{audio::AudioSink, prelude::*};
use bevy_parallax::{
    LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxResource,
};
use enemy::{animate_enemy, create_enemy_bundle, move_enemy};
use life::{create_life, move_life, vehicle_hit_handle};
use plasma::{animate_plasma, fire_plasma, move_plasma};
use vehicle::{animate_vehicle, move_vehicle, Vehicle, VehicleHitEvent};

#[derive(Resource)]
pub struct LevelMusicController(pub Handle<AudioSink>);

fn main() {
    // Define window
    let window = WindowDescriptor {
        title: "Pixel Attack".to_string(),
        width: 1280.0,
        height: 720.0,
        resizable: false,
        ..Default::default()
    };

    App::new()
        // Add parallax resource with layer data
        .insert_resource(ParallaxResource {
            layer_data: vec![
                LayerData {
                    speed: 0.9,
                    path: "back.png".to_string(),
                    tile_size: Vec2::new(96.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 0.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.6,
                    path: "middle.png".to_string(),
                    tile_size: Vec2::new(144.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 1.0,
                    ..Default::default()
                },
                LayerData {
                    speed: 0.1,
                    path: "front.png".to_string(),
                    tile_size: Vec2::new(272.0, 160.0),
                    cols: 1,
                    rows: 1,
                    scale: 4.5,
                    z: 2.0,
                    ..Default::default()
                },
            ],
            ..Default::default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window,
                    ..default()
                })
                // Use nearest filtering so our pixel art renders clear
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(ParallaxPlugin)
        .add_startup_system(setup)
        .add_system(move_camera_system)
        .add_system(animate_vehicle)
        .add_system(animate_enemy)
        .add_system(animate_plasma)
        .add_system(move_vehicle)
        .add_system(move_plasma)
        .add_system(move_enemy)
        .add_system(move_life)
        .add_system(fire_plasma)
        .add_system(vehicle_hit_handle)
        .add_system(create_enemy_bundle)
        .add_event::<VehicleHitEvent>()
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("music.ogg");

    let handle = audio_sinks
        .get_handle(audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.80)));
    commands.insert_resource(LevelMusicController(handle));

    let vehicle_texture_handle = asset_server.load("vehicle.png");
    let vehicle_texture_atlas = TextureAtlas::from_grid(
        vehicle_texture_handle,
        Vec2::new(176.0, 96.0),
        3,
        1,
        None,
        None,
    );
    let vehicle_texture_atlas_handle = texture_atlases.add(vehicle_texture_atlas);

    commands
        .spawn(Camera2dBundle::default())
        .insert(ParallaxCameraComponent);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: vehicle_texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(-450.0, -29.5, 4.0),
            ..default()
        },
        AnimationIndices { first: 0, last: 2 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Vehicle,
    ));

    let lifes = create_life(asset_server, texture_atlases);

    for (sprite_sheet, life) in lifes.iter() {
        commands.spawn((sprite_sheet.clone(), life.clone()));
    }
}

fn move_camera_system(mut move_event_writer: EventWriter<ParallaxMoveEvent>) {
    move_event_writer.send(ParallaxMoveEvent {
        camera_move_speed: 3.0,
    });
}
