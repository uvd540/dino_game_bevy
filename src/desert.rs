use bevy::prelude::*;

pub struct DesertPlugin;

impl Plugin for DesertPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_desert_assets)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_desert)
            .add_system(handle_track_movement);
    }
}

#[derive(Component)]
struct Ground;

// #[derive(Component)]
// struct Obstacle {
//     level: ObstacleLevel,
// }

// enum ObstacleLevel {
//     ONE,
//     TWO,
//     THREE
// }

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
        min: Vec2::new(2., 65.),
        max: Vec2::new(1201., 54.),
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