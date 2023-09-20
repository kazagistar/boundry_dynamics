mod character;
mod monster;
mod song;

use bevy::prelude::*;
use character::CharacterPlugin;
use monster::MonsterPlugin;
use song::SongPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins((SongPlugin, CharacterPlugin, MonsterPlugin))
        .add_systems(Startup, start_camera)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
