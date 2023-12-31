mod character;
mod monster;
mod song;
mod animation;

use bevy::{prelude::*, diagnostic::*};
use bevy_rapier2d::prelude::*;
use character::CharacterPlugin;
use monster::MonsterPlugin;
use song::SongPlugin;
use animation::AnimationPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((SongPlugin, CharacterPlugin, MonsterPlugin, AnimationPlugin))
        .add_systems(Startup, start_camera)
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .run();
}

fn start_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
