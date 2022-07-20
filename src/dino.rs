use bevy::prelude::*;

// Texture Atlas indices:
// 0 - Standing
// 1, 2 - Running

pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_dino_assets)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_dino)
            .add_system(handle_dino_animations)
            .add_system(handle_dino_movement);
    }
}

#[derive(Component)]
struct Dino {
    dino_state: DinoState,
    vel_y: f32,
}

#[derive(PartialEq)]
enum DinoState {
    // Standing,
    Running,
    Jumping,
    // Collided,
}

#[derive(Clone)]
struct DinoTextureAtlas(Handle<TextureAtlas>);

fn load_dino_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet_texture_handle = asset_server.load("dino_spritesheet.png");
    let mut dino_texture_atlas =
        TextureAtlas::new_empty(spritesheet_texture_handle, Vec2::new(1233., 68.));
    // Standing Sprite
    dino_texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(850., 46.),
        max: Vec2::new(889., 4.),
    });
    // Running Sprites
    dino_texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(938., 46.),
        max: Vec2::new(977., 4.),
    });
    dino_texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(982., 46.),
        max: Vec2::new(1021., 4.),
    });

    let texture_atlas_handle = texture_atlases.add(dino_texture_atlas);
    commands.insert_resource(DinoTextureAtlas(texture_atlas_handle));
}

fn spawn_dino(mut commands: Commands, dino_texture_atlas_handle: Res<DinoTextureAtlas>) {
    let dino = Dino {
        dino_state: DinoState::Running,
        vel_y: 0.,
    };
    commands
        .spawn()
        .insert(dino)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: dino_texture_atlas_handle.0.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(
                crate::DINO_X_LOCATION,
                crate::DINO_Y_LOCATION,
                0.,
            )),
            ..Default::default()
        })
        .insert(AnimationTimer(Timer::from_seconds(
            1. / (f32::from(crate::DINO_FPS)),
            true,
        )));
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn handle_dino_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Dino)>,
) {
    for (mut timer, mut sprite, dino) in query.iter_mut() {
        match dino.dino_state {
            DinoState::Jumping => {
                sprite.index = 0;
            }
            DinoState::Running => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    match sprite.index {
                        1 => sprite.index = 2,
                        _ => sprite.index = 1,
                    }
                }
            }
        }
    }
}

fn handle_dino_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Dino, &mut Transform)>,
) {
    for (mut dino, mut transform) in query.iter_mut() {
        match dino.dino_state {
            DinoState::Running => {
                if keyboard_input.just_pressed(KeyCode::Space) {
                    dino.vel_y = crate::JUMP_VEL;
                    dino.dino_state = DinoState::Jumping;
                }
            }
            DinoState::Jumping => {
                if transform.translation.y == crate::DINO_Y_LOCATION {
                    dino.dino_state = DinoState::Running;
                }
            }
        }
        dino.vel_y -= crate::GRAVITY;
        transform.translation.y += dino.vel_y;
        transform.translation.y = transform
            .translation
            .y
            .clamp(crate::DINO_Y_LOCATION, crate::WINDOW_HEIGHT / 2.);
    }
}
