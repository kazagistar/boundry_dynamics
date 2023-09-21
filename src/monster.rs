use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

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
            .add_systems(Update, (spawn_monster, chase, juice));
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
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
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
        Monster,
        SpriteSheetBundle {
            texture_atlas: spawner.handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        RigidBody::Dynamic,
        Position::from(spawn_pos.truncate()),
        Collider::ball(15.0),
        //Restitution::from(0.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn chase(
    mut monsters: Query<(&Transform, &mut LinearVelocity), (With<Monster>, Without<Character>)>,
    character: Query<&Transform, (With<Character>, Without<Monster>)>,
    song: Res<SongPlayback>
) {
    let character = character.single().translation;
    // Travel 4 pixels per tick (pixels per meter is set to 1.0)
    let target_speed = song.bpm_timer.percent_left() * 4.0 * 60.0;
    for (transform, mut velocity) in monsters.iter_mut() {
        let target_velocity = (character - transform.translation).clamp_length(0.0,  target_speed);
        // Fix velocity
        velocity.0 = target_velocity.truncate();
        // println!("{}", ext_imp.impulse);
    }
}

fn juice(
    mut monsters: Query<&mut Transform, With<Monster>>,
    song: Res<SongPlayback>,
) {
    let size = 0.8 + song.bpm_timer.percent_left().min(song.bpm_timer.percent()) * 0.4;
    for mut transform in monsters.iter_mut() {
        transform.scale = Vec3::new(size, size, 1.0);
    }
}
