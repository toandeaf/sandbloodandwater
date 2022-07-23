use bevy::prelude::*;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StarterPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .run()
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);

    let window = windows.get_primary_mut().unwrap();

    let (win_w, win_h) = (window.width(), window.height());
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    query
        .iter_mut()
        .for_each(|(mut timer, mut sprite, texture_atlas_handle)| {
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        });
}

struct StarterPlugin;

impl Plugin for StarterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
            .insert_resource(WindowDescriptor {
                title: "Jake's Wee Game".to_string(),
                width: 600.0,
                height: 700.0,
                ..Default::default()
            });
    }
}
