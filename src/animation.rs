use std::{ops::Range, f32::consts::PI};

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use enum_map::{Enum, EnumMap};

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Update, animate)
            .add_systems(Update, animate_walking);
    }
}

#[derive(Component, Debug, Default)]
pub struct SimpleAnimation {
    pub range: Range<usize>,
    pub timer: Timer,
}

pub fn animate(time: Res<Time>, mut query: Query<(&mut TextureAtlasSprite, &mut SimpleAnimation)>) {
    for (mut sprite, mut schedule) in query.iter_mut() {
        schedule.timer.tick(time.delta());
        if !schedule.timer.just_finished() {
            continue;
        }

        sprite.index += 1;
        if sprite.index >= schedule.range.end {
            sprite.index = schedule.range.start;
        }
    }
}

#[derive(Debug, Enum, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Default,
}

impl Direction {
    pub fn rotate_to(&self, other: &Self) -> Quat {
        if *self == Direction::Default || *other == Direction::Default {
            Quat::IDENTITY
        } else {
            let difference = (*other as u8).wrapping_sub(*self as u8) % 4;
            Quat::from_rotation_z(difference as f32 * PI / -2.)
        }
    }
    
    pub fn to_vec(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::Y,
            Direction::Right => Vec2::X,
            Direction::Down => Vec2::NEG_Y,
            Direction::Left => Vec2::NEG_X,
            Direction::Default => Vec2::ZERO,
        }
    }
}

impl From<Vec2> for Direction {
    fn from(v: Vec2) -> Self {
        if v == Vec2::ZERO {
            Direction::Default
        } else if v.x * v.x >= v.y * v.y {
            if v.x > 0. { Direction::Right } else { Direction::Left }
        } else {
            if v.y > 0. { Direction::Up } else { Direction::Down }
        }
    }
}

#[derive(Component, Debug)]
pub struct SimpleWalkingAnimation {
    pub animations: EnumMap<Direction, Range<usize>>,
    pub current: Direction
}

pub fn animate_walking(mut query: Query<(&mut SimpleAnimation, &mut SimpleWalkingAnimation, &mut TextureAtlasSprite, &Velocity), Changed<Velocity>>) {
    for (mut animation, mut walking, mut sprite, velocity) in query.iter_mut() {
        let direction = Direction::from(velocity.linvel);
        println!("velocity changed, dir: {:?}", direction);
        if walking.current == direction {
            return;
        }
        walking.current = direction;

        let relative_current = sprite.index - animation.range.start;
        animation.range = walking.animations[direction].clone();
        sprite.index = animation.range.start + relative_current;
    }
}

