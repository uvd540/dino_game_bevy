use bevy::prelude::*;

pub struct DinoPlugin;

impl Plugin for DinoPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, load_dino_assets)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_dino)
            .add_system(handle_dino_movement)
            ;
    }
}

#[derive(Component)]
struct Dino {
    dino_state: DinoState,
    vel_y: f32
}

#[derive(PartialEq)]
enum DinoState {
    // Standing,
    Running,
    Jumping,
    // Collided,
}

#[derive(Clone)]
struct DinoRunTextureAtlas(Handle<TextureAtlas>);

fn load_dino_assets (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet_texture_handle = asset_server.load("dino_spritesheet.png");
    let mut dino_run_texture_atlas = TextureAtlas::new_empty(spritesheet_texture_handle, Vec2::new(1233., 68.));
    // Standing Sprite
    // texture_atlas.add_texture(bevy::sprite::Rect {
    //     min: Vec2::new(850., 46.),
    //     max: Vec2::new(889., 4.)
    // });
    // Running Sprites
    dino_run_texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(938., 46.),
        max: Vec2::new(977., 4.)
    });
    dino_run_texture_atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(982., 46.),
        max: Vec2::new(1021., 4.)
    });

    let texture_atlas_handle = texture_atlases.add(dino_run_texture_atlas);
    commands.insert_resource(DinoRunTextureAtlas(texture_atlas_handle));

}

fn spawn_dino(
    mut commands: Commands,
    dino_run_texture_atlas_handle: Res<DinoRunTextureAtlas>
) {
    let dino = Dino {
        dino_state: DinoState::Running,
        vel_y: 0.
    };
    commands
        .spawn()
        .insert(dino)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: dino_run_texture_atlas_handle.0.clone(),
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_translation(Vec3::new(crate::DINO_X_LOCATION, crate::DINO_Y_LOCATION, 0.)),
            ..Default::default()
        });
}

// fn handle_dino_animations
// Query (TextureAtlasSprite, Dino)
// change Sprite properties based on Dino.dino_state

fn handle_dino_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Dino, &mut Transform)>
) {
    for (mut dino, mut transform) in query.iter_mut() {
        match dino.dino_state {
            DinoState::Running => {
                if keyboard_input.just_pressed(KeyCode::Space) {
                    dino.vel_y = crate::JUMP_VEL;
                    dino.dino_state = DinoState::Jumping;
                }
            },
            DinoState::Jumping => {
                if transform.translation.y == crate::DINO_Y_LOCATION {
                    dino.dino_state = DinoState::Running;
                }
            },
        }
        dino.vel_y -= crate::GRAVITY;
        transform.translation.y += dino.vel_y;
        transform.translation.y = transform.translation.y.clamp(crate::DINO_Y_LOCATION, crate::WINDOW_HEIGHT / 2.);
    }
}