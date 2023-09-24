use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
            .add_systems(Update, (spawn_monster, BeatChase::system, BeatScale::system));
    }
}

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
        BeatChase { tween: Tween { ttype: TweenType::Sawtooth, a: 60.0 * 4.0, b: 60.0 * -1.0 }},
        SpatialBundle::from(Transform::from_translation(spawn_pos)),
        RigidBody::Dynamic,
        Collider::ball(15.0),
        Restitution::coefficient(0.0),
        Velocity::default(),
        AdditionalMassProperties::Mass(1.0),
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
    )).with_children(|builder| {
        builder.spawn((
            BeatChase { tween: Tween { ttype: TweenType::Triangle, a: 0.8, b: 1.2 }},
            SpriteSheetBundle {
                texture_atlas: spawner.handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
        ));
    });
}


#[derive(Component)]
struct BeatChase {
    tween: Tween
}
impl BeatChase {
    fn system(
        mut monsters: Query<(&Transform, &mut Velocity, &BeatChase)>,
        character: Query<&Transform, With<Character>>,
        song: Res<SongPlayback>,
    ) {
        let character = character.single().translation;
        let p = song.bpm_timer.percent();
        // Travel 4 pixels per tick (pixels per meter is set to 1.0)
        for (transform, mut velocity, config) in monsters.iter_mut() {
            let target_speed = config.tween.tween(p);
            let target_velocity = (character - transform.translation).clamp_length(0.0,  target_speed);
            // Fix velocity
            velocity.linvel = target_velocity.truncate();
        }
    }
}


#[derive(Component)]
struct BeatScale {
    tween: Tween,
}
impl BeatScale {
    fn system(
        mut monsters: Query<(&mut Transform, &BeatScale)>,
        song: Res<SongPlayback>,
    ) {
        let p = song.bpm_timer.percent();
        for (mut transform, config) in monsters.iter_mut() {
            let size = config.tween.tween(p);
            transform.scale = Vec3::new(size, size, 1.0);
        }
    }
}

pub struct Tween {
    ttype: TweenType,
    a: f32,
    b: f32,
}

#[derive(Copy, Clone)]
pub enum TweenType {
    Sawtooth,
    Triangle,
}

impl Tween {
    fn tween(&self, p: f32) -> f32 {
        match self.ttype {
            TweenType::Sawtooth => lerp(self.a, self.b, p),
            TweenType::Triangle => lerp(self.a, self.b, p.min(1.0 - p) * 2.0),
        }
    }
}

fn lerp(s: f32, e: f32, p: f32) -> f32 {
    s + p * (e - s)
}
