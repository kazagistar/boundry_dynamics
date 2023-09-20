use std::f32::{INFINITY, NAN};

use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(FixedUpdate, (collide, apply_pushback, reset).chain());
    }
}
#[derive(Bundle, Default)]
pub struct ColliderBunde {
    pub collider: CircleCollider,
    pub collision: CollisionDetector,
}

#[derive(Component, Default)]
pub struct CircleCollider {
    // configuration
    pub radius: f32,
    pub mass: f32,
    pub pushback: bool,
    pub detect: bool,
}

#[derive(Component, Default)]
pub struct CollisionDetector {
    // per-tick output
    offset: Vec2,
    count: u32,
    pub entities: Vec<Entity>,
}

pub fn collide(
    mut e1: Query<(Entity, &Transform, &CircleCollider, &mut CollisionDetector)>,
    e2: Query<(Entity, &Transform, &CircleCollider)>,
) {
    for (entity1, transform1, collider1, mut detector1) in &mut e1 {
        for (entity2, transform2, collider2) in &e2 {
            // dont compare to yourself
            if entity1 == entity2 {
                continue;
            }
            let range = collider1.radius + collider2.radius;
            let range_squared = range * range;
            let p1 = transform1.translation.truncate();
            let p2 = transform2.translation.truncate();
            let distance_squared = p1.distance_squared(p2);
            // skip if no collision
            if distance_squared >= range_squared {
                continue;
            }

            if collider1.detect {
                detector1.entities.push(entity2);
            }
            if collider1.pushback && collider2.pushback {
                let direction = if distance_squared != 0.0 {
                    p1 - p2
                } else if entity1 < entity2 {
                    Vec2::new(range, 0.0)
                } else {
                    Vec2::new(-range, 0.0)
                };
                let distance = distance_squared.sqrt();
                let overlap_ratio = (range - distance) / range;
                let m1 = collider1.mass;
                let m2 = collider2.mass;
                let mut scale = m2 / m1 + m2;
                if scale == NAN {
                    scale = if m2 == INFINITY && m1 != INFINITY { 1.0 } else { 0.5 }
                }
                let pushback = scale * direction * overlap_ratio;
                detector1.offset += pushback;
            }
            detector1.count += 1;
        }
    }
}

pub fn apply_pushback (
    mut query: Query<(&mut Transform, &CollisionDetector)>
) {
    for (mut transform, detector) in &mut query {
        if detector.offset.length_squared() != 0.0 {
            transform.translation += (detector.offset / detector.count as f32).extend(0.0);
        }
    }
}

fn reset(mut query: Query<&mut CollisionDetector>) {
    for mut collision in query.iter_mut() {
        //if collision.count == 0 {
        //    return
        //}
        collision.offset = Vec2::ZERO;
        collision.count = 0;
        collision.entities.clear();
    }
}