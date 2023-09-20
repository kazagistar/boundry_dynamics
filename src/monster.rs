use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{character::Character, song::SongPlayback};
use rand::prelude::*;

#[derive(Component)]
struct MonsterSpawner {
    timer: Timer,
    handle: Handle<TextureAtlas>,
}

pub struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, load_monster_spawner)
            .add_systems(Update, spawn_monster)
            .add_systems(FixedUpdate, (chase, juice));
    }
}

#[derive(Component)]
struct Monster;

fn load_monster_spawner(
    asset_server: Res<AssetServer>,
    mut commands: Commands,    
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load(r"sprites/swishs-monster-pack/30px by 30px/swish_skele_abomination.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(30.0, 30.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        MonsterSpawner {
            handle: texture_atlas_handle,
            timer: Timer::from_seconds(0.002, TimerMode::Repeating),
        }
    );
}

fn spawn_monster(
    mut commands: Commands,
    mut spawner: Query<&mut MonsterSpawner>,
    time: Res<Time>,
    character: Query<&Transform, With<Character>>,
) {
    let mut spawner = spawner.single_mut();
    spawner.timer.tick(time.delta());
    if !spawner.timer.just_finished() {
        return
    }

    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..2.0*PI,);
    let distance = rng.gen_range(800.0..1000.0,);
    let offset = Vec2::from_angle(angle) * distance;
    let character_pos = character.single().translation;
    let spawn_pos = character_pos + offset.extend(0.0);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: spawner.handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: spawn_pos,
                ..default()
            },
            ..default()
        },
        Monster,
    ));
}

fn chase(
    mut monsters: Query<(&mut Transform, &Monster)>,
    character: Query<&Transform, (With<Character>, Without<Monster>)>,
    song: Res<SongPlayback>
) {
    let character = character.single().translation;
    let speed = song.bpm_timer.percent_left();
    for (mut transform, _monster) in monsters.iter_mut() {
        transform.translation = transform.translation + (character - transform.translation).clamp_length(0.0, 4.0 * speed);
    }
}

fn juice(
    mut monsters: Query<(&mut Transform, &Monster)>,
    song: Res<SongPlayback>,
) {
    let size = 1.0 + song.bpm_timer.percent_left().min(song.bpm_timer.percent()) * 0.4;
    for (mut transform, _monster) in monsters.iter_mut() {
        transform.scale = Vec3::new(size, size, 1.0);
    }
}
