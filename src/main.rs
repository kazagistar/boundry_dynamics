mod character;
mod monster;
mod song;

use bevy::{prelude::*, diagnostic::*};
use bevy_xpbd_2d::prelude::*;
use character::CharacterPlugin;
use monster::MonsterPlugin;
use song::SongPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        .add_plugins((SongPlugin, CharacterPlugin, MonsterPlugin))
        .add_systems(Startup, start_camera)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
