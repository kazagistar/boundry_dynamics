use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;
use enum_map::enum_map;

use crate::{animation::{SimpleAnimation, SimpleWalkingAnimation, Direction}, song::SongPlayback};

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, (init_asset_hack, apply_deferred, load_character).chain())
            .add_systems(FixedUpdate, (move_character, energy_ball_attack, move_urb));
    }
}

#[derive(Default, Resource)]
struct AssetHack {
    texture_atlas: HashMap<String, Handle<TextureAtlas>>,
}

fn init_asset_hack(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut assets = AssetHack {
        texture_atlas: HashMap::default(),
    };
    
    assets.texture_atlas.insert("red-effects".into(), texture_atlases.add(
        TextureAtlas::from_grid(
            asset_server.load(r"sprites\effect-bullet-impact-explosion\Red Effect Bullet Impact Explosion 32x32.png"),
            Vec2::new(32.0, 32.0),
            20,
            16,
            None,
            None
    )));
    assets.texture_atlas.insert("character-1".into(), texture_atlases.add(
        TextureAtlas::from_grid(
            asset_server.load(r"sprites\free-rgw-sprites\16x16\Character_001.png"),
            Vec2::new(24.0, 24.0),
            4,
            4,
            None,
            None
    )));

    commands.insert_resource(assets);
}

#[derive(Component)]
pub struct Character;

fn load_character(
    assets: Res<AssetHack>,
    mut commands: Commands,
) {

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.texture_atlas["character-1"].clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1.},
                rotation: default(),
                scale: Vec3::new(1.5, 1.5, 1.),
            },
            ..default()
        },
        SimpleAnimation {
            range: 0..4,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        SimpleWalkingAnimation {
            animations: enum_map![
                Direction::Down => 0..4,
                Direction::Left => 4..8,
                Direction::Right => 8..12,
                Direction::Up => 12..16,
                Direction::Default => 0..1,
            ],
            current: Direction::Default,
        },
        RigidBody::KinematicPositionBased,
        GravityScale(0.0),
        Collider::ball(5.0),
        Character,
        Velocity::default(),
        EnergyBallAttack,
    ));
}

fn move_character(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Character>>,
) {
    let mut direction = Vec3::default();
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.;
    }
    if let Some(mut direction) = direction.try_normalize() {
        direction *= 4.0;
        for mut transform in &mut query {
            transform.translation += direction;
        }
    }
}

#[derive(Debug, Component)]
struct EnergyBallAttack;

fn energy_ball_attack(
    mut commands: Commands,
    query: Query<(&Transform, &SimpleWalkingAnimation), With<EnergyBallAttack>>,
    song: Res<SongPlayback>,
    assets: Res<AssetHack>,
) {
    if !song.bpm_timer.just_finished() {
        return;
    }
    for (transform, animation) in &query {
        let dir = if animation.current == Direction::Default { Direction::Down } else { animation.current };

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: assets.texture_atlas["red-effects"].clone(),
                sprite: TextureAtlasSprite::new(111),
                transform: Transform {
                    translation: transform.translation + (dir.to_vec() * 12.).extend(0.),
                    rotation: Direction::Left.rotate_to(&dir),
                    scale: Vec3::new(2., 2., 1.),
                },
                ..default()
            },
            SimpleAnimation {
                range: 111..115,
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            Collider::ball(16.0),
            Sensor,
            Velocity {
                linvel: dir.to_vec() * 8.,
                ..default()
            },
            UnRigidBody,
        ));
    }
}

#[derive(Component)]
struct UnRigidBody;

fn move_urb(
    mut query: Query<(&mut Transform, &Velocity), With<UnRigidBody>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.linvel.extend(0.);
    }
}