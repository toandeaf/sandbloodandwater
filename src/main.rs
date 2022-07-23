use bevy::prelude::*;
use bevy::window::WindowMode;

mod map;
mod player;

fn main() {
    App::new()
        .add_plugin(StarterPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(map::TileMapPlugin)
        .add_startup_system(setup)
        .run()
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}

struct StarterPlugin;

impl Plugin for StarterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Sand, Blood and Water".to_string(),
            height: 700.,
            width: 500.,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        });
    }
}
