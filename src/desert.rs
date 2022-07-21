use bevy::prelude::*;
use rand::Rng;

pub struct DesertPlugin;

impl Plugin for DesertPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_desert_assets)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_desert)
            .add_system(handle_track_movement)
            .add_system(add_environment_objects)
            .add_system(handle_animations)
            .add_system(handle_desert_object_movement);
    }
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct DesertObject;

#[derive(Component, Deref)]
struct Obstacle(u8);

#[derive(Component)]
struct Animated;

#[derive(Clone)]
struct DesertTextureAtlas(Handle<TextureAtlas>);

fn load_desert_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet_texture_handle = asset_server.load("dino_spritesheet.png");
    let mut texture_atlas =
        TextureAtlas::new_empty(spritesheet_texture_handle, Vec2::new(1233., 68.));
    // Ground Sprite
    texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(2., 66.),
        max: Vec2::new(1201., 53.),
    });
    // Cloud Sprite
    texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(85., 15.),
        max: Vec2::new(132., 1.),
    });
    // Bird Sprite 1
    texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(135., 3.),
        max: Vec2::new(178., 40.),
    });
    // Bird Sprite 2
    texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(181., 3.),
        max: Vec2::new(224., 40.),
    });

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(DesertTextureAtlas(texture_atlas_handle));
}

fn spawn_desert(mut commands: Commands, texture_atlas: Res<DesertTextureAtlas>) {
    commands
        .spawn()
        .insert(Ground {})
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas.0.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(
                599. - (crate::WINDOW_WIDTH / 2.),
                crate::DINO_Y_LOCATION - 16.,
                0.,
            )),
            ..Default::default()
        });
    commands
        .spawn()
        .insert(DesertObjectSpawnTimer(Timer::from_seconds(1., true)));
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn handle_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            match sprite.index {
                2 => sprite.index = 3,
                _ => sprite.index = 2,
            }
        }
    }
}

fn handle_track_movement(
    mut commands: Commands,
    texture_atlas: Res<DesertTextureAtlas>,
    mut query: Query<(Entity, &mut Transform), With<Ground>>,
) {
    let num_tracks = query.iter().count();
    for (ground, mut transform) in query.iter_mut() {
        transform.translation.x -= crate::TRACK_SPEED;
        if (transform.translation.x < 0.) && (num_tracks < 2) {
            commands
                .spawn()
                .insert(Ground {})
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas.0.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x + 1198.,
                        crate::DINO_Y_LOCATION - 16.,
                        0.,
                    )),
                    ..Default::default()
                });
        } else if transform.translation.x < -(599. + (crate::WINDOW_WIDTH / 2.)) {
            commands.entity(ground).despawn();
        }
    }
}

#[derive(Component, Deref, DerefMut)]
struct DesertObjectSpawnTimer(Timer);

fn add_environment_objects(mut commands: Commands, texture_atlas: Res<DesertTextureAtlas>) {
    if rand::thread_rng().gen_range(0..=400) % 200 == 0 {
        match rand::thread_rng().gen_range(0..=1) {
            0 => { // Bird
                commands
                    .spawn()
                    .insert(DesertObject)
                    .insert(AnimationTimer(Timer::from_seconds(
                        1. / (f32::from(crate::DINO_FPS)),
                        true,
                    )))
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas.0.clone(),
                        sprite: TextureAtlasSprite::new(2),
                        transform: Transform::from_translation(Vec3::new(
                            crate::WINDOW_WIDTH / 2.,
                            crate::WINDOW_HEIGHT / 2. * rand::thread_rng().gen_range(0.2..=0.8),
                            0.,
                        )),
                        ..Default::default()
                    });
            }
            _ => { // Cloud
                commands
                    .spawn()
                    .insert(DesertObject)
                    .insert_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas.0.clone(),
                        sprite: TextureAtlasSprite::new(1),
                        transform: Transform::from_translation(Vec3::new(
                            crate::WINDOW_WIDTH / 2.,
                            crate::WINDOW_HEIGHT / 2. * rand::thread_rng().gen_range(0.2..=0.8),
                            0.,
                        )),
                        ..Default::default()
                    });
            }
        }
    }
}

fn handle_desert_object_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<DesertObject>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= crate::TRACK_SPEED;
        if transform.translation.x < -(crate::WINDOW_WIDTH / 2. + 50.) {
            commands.entity(entity).despawn();
        }
    }
}
