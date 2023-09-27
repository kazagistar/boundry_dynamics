use std::f32::consts::PI;

use bevy::{prelude::*, ecs::system::EntityCommands};
use bevy_rapier2d::prelude::*;

use crate::{character::Character, song::SongPlayback};
use rand::prelude::*;

pub struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, load_monster_spawner)
            .add_systems(Update, (spawn_monster, BeatChase::system, BeatScale::system, BeatSpin::system, BeatLineDash::system));
    }
}

type EntitySpawnFn = Box<dyn FnMut(&mut EntityCommands) + Send + Sync>;

#[derive(Component)]
struct MonsterSpawner {
    timer: Timer,
    monsters: Vec<EntitySpawnFn>,
}

fn load_monster_spawner(
    asset_server: Res<AssetServer>,
    mut commands: Commands,    
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(
        MonsterSpawner {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            monsters: vec![
                skeleton_spawner(&asset_server, &mut texture_atlases),
                bird_spawner(&asset_server, &mut texture_atlases),
                lizard_spawner(&asset_server, &mut texture_atlases),
            ],
        }
    );
}

fn skeleton_spawner(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) -> EntitySpawnFn {
    let texture_handle: Handle<Image> = asset_server.load(r"sprites/swishs-monster-pack/30px by 30px/swish_skele_abomination.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(30.0, 30.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    Box::new(move |commands| {
        commands.insert((
            BeatChase { tween: Tween { ttype: TweenType::Square, a: 60.0 * 3.0, b: 60.0 * 0.0, ..default()}, on_beat: 0, freq: 1},
            RigidBody::Dynamic,
            Collider::ball(15.0),
            Restitution::coefficient(0.0),
            Velocity::default(),
            ColliderMassProperties::Mass(5.0),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
        )).with_children(|builder| {
            builder.spawn((
                BeatSpin { freq: 4, on_beat: 1, tween: Tween { ttype: TweenType::Sawtooth, a: 0.0, b: 2.0 * PI, start: 0.5, ..default()}},
                BeatScale { tween: Tween { ttype: TweenType::Square, a: 1.2, b: 1.0, ..default()}, on_beat: 0, freq: 1},
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    ..default()
                },
            ));
        });
    })
}

fn bird_spawner(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) -> EntitySpawnFn {
    let texture_handle: Handle<Image> = asset_server.load(r"sprites/swishs-monster-pack/30px by 30px/swish_terrordactyl.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(30.0, 30.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    Box::new(move |commands| {
        commands.insert((
            BeatChase { tween: Tween { ttype: TweenType::Sawtooth, a: 60.0 * 4.0, b: 60.0 * 2.0, ..default()}, on_beat: 0, freq: 1},
            RigidBody::Dynamic,
            Collider::ball(15.0),
            Restitution::coefficient(0.0),
            Velocity::default(),
            ColliderMassProperties::Mass(0.1),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
        )).with_children(|builder| {
            builder.spawn((
                BeatScale { tween: Tween { ttype: TweenType::Triangle, a: 1.0, b: 1.1, mid: 0.2, ..default()}, on_beat: 0, freq: 1},
                BeatSpin { freq: 4, on_beat: 3, tween: Tween { ttype: TweenType::Sawtooth, a: 2.0 * PI, b: 0.0, ..default()}},
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    ..default()
                },
            ));
        });
    })
}

fn lizard_spawner(
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) -> EntitySpawnFn {
    let texture_handle: Handle<Image> = asset_server.load(r"sprites/swishs-monster-pack/24px by 24px/swish_crested_lizard.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    Box::new(move |commands| {
        commands.insert((
            BeatLineDash { tween: Tween { ttype: TweenType::Triangle, a: 60.0 * 6.0, b: 12.0 * 16.0, ..default()}, on_beat: 0, freq: 2},
            BeatScale { tween: Tween { ttype: TweenType::Sawtooth, a: 2.5, b: 1.0, ..default()}, on_beat: 1, freq: 2},
            RigidBody::Dynamic,
            Collider::ball(15.0),
            Restitution::coefficient(0.0),
            Velocity::default(),
            ColliderMassProperties::Mass(1.0),
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
        )).with_children(|builder| {
            builder.spawn((
                BeatSpin { freq: 2, on_beat: 1, tween: Tween { ttype: TweenType::Sawtooth, a: 2.0 * PI, b: 0.0, ..default()}},
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    ..default()
                },
            ));
        });
    })
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

    let mut entity_commands = commands.spawn((
        SpatialBundle::from(Transform::from_translation(spawn_pos)),
    ));
    if let Some(monster) = spawner.monsters.choose_mut(&mut rng) {
        monster(&mut entity_commands);
    } else {
        println!("Error, no monsters found to spawn")
    }
}

#[derive(Component, Clone)]
struct BeatLineDash {
    tween: Tween,
    freq: usize,
    on_beat: usize,
}
#[derive(Component, Clone)]
#[component(storage = "SparseSet")]
struct DirectionLocked {
    dir: Vec2,
}
impl BeatLineDash {
    fn system(
        mut commands: Commands,
        mut monsters: Query<(Entity, &Transform, &mut Velocity, &BeatLineDash, Option<&mut DirectionLocked>)>,
        character: Query<&Transform, With<Character>>,
        song: Res<SongPlayback>,
    ) {
        let character = character.single().translation;
        let p = song.bpm_timer.percent();
        // Travel 4 pixels per tick (pixels per meter is set to 1.0)
        for (entity, transform, mut velocity, config, lock) in monsters.iter_mut() {
            if song.beat_count % config.freq == config.on_beat {
                let target_speed = config.tween.tween(p);
                let flat_direction = lock.map_or_else(|| {
                    let target_direction = character - transform.translation;
                    let new_lock = if target_direction.x.abs() > target_direction.y.abs() {
                        Vec2::new(target_direction.x, 0.0)
                    } else {
                        Vec2::new(0.0, target_direction.y)
                    };
                    commands.entity(entity).insert(DirectionLocked { dir: new_lock });
                    new_lock
                }, |l| l.dir);
                
                velocity.linvel = flat_direction.clamp_length(target_speed, target_speed)
            } else {
                if lock.is_some() {
                    commands.entity(entity).remove::<DirectionLocked>();
                }
                velocity.linvel = Vec2::ZERO;
            }
        }
    }
}

#[derive(Component, Clone)]
struct BeatChase {
    tween: Tween,
    freq: usize,
    on_beat: usize,
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
            if song.beat_count % config.freq == config.on_beat {
                let target_speed = config.tween.tween(p);
                let target_velocity = (character - transform.translation).clamp_length(target_speed, target_speed);
                // Fix velocity
                velocity.linvel = target_velocity.truncate();
            } else {
                velocity.linvel = Vec2::ZERO;
            }
        }
    }
}

#[derive(Component, Clone)]
struct BeatScale {
    tween: Tween,
    freq: usize,
    on_beat: usize,
}
impl BeatScale {
    fn system(
        mut monsters: Query<(&mut Transform, &BeatScale)>,
        song: Res<SongPlayback>,
    ) {
        let p = song.bpm_timer.percent();
        for (mut transform, config) in monsters.iter_mut() {
            if song.beat_count % config.freq == config.on_beat {
                let size = config.tween.tween(p);
                transform.scale = Vec3::new(size, size, 1.0);
            } else {
                transform.scale = Vec3::ONE;
            }
        }
    }
}


#[derive(Component, Clone)]
struct BeatSpin {
    tween: Tween,
    freq: usize,
    on_beat: usize,
}
impl BeatSpin {
    fn system(
        mut monsters: Query<(&mut Transform, &BeatSpin)>,
        song: Res<SongPlayback>,
    ) {
        let p = song.bpm_timer.percent();
        for (mut transform, config) in monsters.iter_mut() {
            if song.beat_count % config.freq == config.on_beat {
                let radians = config.tween.tween(p);
                transform.rotation = Quat::from_rotation_z(radians);
            } else {
                transform.rotation = Quat::from_rotation_z(0.0);
            }
        }
    }
}

#[derive(Clone)]
pub struct Tween {
    ttype: TweenType,
    a: f32,
    b: f32,
    start: f32,
    mid: f32,
    end: f32,
}

impl Default for Tween {
    fn default() -> Self {
        Self { ttype: TweenType::Sawtooth, a: 1.0, b: 1.0, start: 0.0, mid: 0.5, end: 1.0 }
    }
}

#[derive(Copy, Clone)]
pub enum TweenType {
    Sawtooth,
    Triangle,
    Square,
}

impl Tween {
    fn tween(&self, p: f32) -> f32 {
        if p < self.start { return self.a }
        if p > self.end {
            match self.ttype {
                TweenType::Triangle => return self.a,
                _ => return self.b,
            }
        }
        match self.ttype {
            TweenType::Sawtooth =>
                lerp(self.a, self.b, p),
            TweenType::Triangle =>
                if p < self.mid { 
                    lerp(self.a, self.b, p / self.mid)
                } else {
                    lerp(self.a, self.b, p / (1.0 - self.mid))
                }
            TweenType::Square => 
                if p < self.mid { self.a } else { self.b },
        }
    }
}

fn lerp(s: f32, e: f32, p: f32) -> f32 {
    s + p * (e - s)
}
