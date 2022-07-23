use bevy::prelude::*;

const PLAYER_SPRITE: &str = "ahman.png";
const PLAYER_SIZE: (f32, f32) = (0.025, 0.025);

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_player)
            .add_system(move_player);
    }
}

fn create_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(self::generate_player_bundle(asset_server))
        .insert(Player);
}

fn generate_player_bundle(asset_server: Res<AssetServer>) -> bevy::prelude::SpriteBundle {
    SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            scale: Vec3::new(PLAYER_SIZE.0, PLAYER_SIZE.1, 1.),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    } else if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    } else if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    } else if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }

    let move_speed = 0.075;
    let move_delta = (direction * move_speed).extend(0.);

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta
    }
}
