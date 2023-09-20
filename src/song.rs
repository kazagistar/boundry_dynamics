use bevy::prelude::*;

pub struct SongPlugin;
impl Plugin for SongPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, load_music)
            .add_systems(Update, tick_song);
    }
}

#[derive(Resource)]
pub struct SongPlayback {
    pub bpm_timer: Timer,
}

fn load_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("music/Wish You Were Here.mp3"),
        ..default()
    });
    commands.insert_resource(SongPlayback {
        bpm_timer: Timer::from_seconds(0.48, TimerMode::Repeating),
    });
}

fn tick_song(
    mut song: ResMut<SongPlayback>,
    time: Res<Time>
) {
    song.bpm_timer.tick(time.delta());
}
