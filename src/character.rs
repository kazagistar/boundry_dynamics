use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, load_character)
            .add_systems(FixedUpdate, move_character);
    }
}

#[derive(Component)]
pub struct Character;

fn load_character(
    asset_server: Res<AssetServer>,
    mut commands: Commands,    
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle: Handle<Image> = asset_server.load(r"sprites\free-rgw-sprites\16x16\Character_001.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: 1.},
                rotation: default(),
                scale: Vec3::new(1.5, 1.5, 1.),
            },
            ..default()
        },
        RigidBody::KinematicPositionBased,
        GravityScale(0.0),
        Collider::ball(5.0),
        Character,
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