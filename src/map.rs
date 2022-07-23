use bevy::prelude::*;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map);
    }
}

const TILE_SIZE: (f32, f32) = (0.005, 0.005);
const TILE_WIDTH: f32 = 1.0;
const TILE_HEIGHT: f32 = 1.0;
const INTIAL_TILE_WIDTH: f32 = TILE_WIDTH * -13.;
const INTIAL_TILE_HEIGHT: f32 = TILE_HEIGHT * -7.;

fn create_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("tiles/desert/Tile_5.png");

    let mut x_pos = INTIAL_TILE_WIDTH.clone();
    let mut y_pos = INTIAL_TILE_HEIGHT.clone();

    for _x_increment in 0..30 {
        commands.spawn_bundle(SpriteBundle {
            texture: texture.clone(),
            transform: Transform {
                translation: Vec3::new(x_pos, y_pos, 0.),
                scale: Vec3::new(TILE_SIZE.0, TILE_SIZE.1, 1.),
                ..Default::default()
            },
            ..Default::default()
        });
        for _y_increment in 0..20 {
            commands.spawn_bundle(SpriteBundle {
                texture: texture.clone(),
                transform: Transform {
                    translation: Vec3::new(x_pos, y_pos, 0.),
                    scale: Vec3::new(TILE_SIZE.0, TILE_SIZE.1, 1.),
                    ..Default::default()
                },
                ..Default::default()
            });
            y_pos += TILE_HEIGHT;
        }

        x_pos += TILE_WIDTH;
        y_pos = INTIAL_TILE_HEIGHT;
    }
}
