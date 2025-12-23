use bevy::prelude::*;

use crate::{
    animation::{AnimatedSprite, AnimationTextureAtlasLayout},
    game::{InGameState, MAX_X, MIN_X, MIN_Y},
};

const VELOCITY_X: f32 = 300.0;
const SCALE: f32 = 0.25;
const PLAYER_Y: f32 = MIN_Y + 100.0;
pub const COLL_WIDTH: f32 = 80.0;
pub const COLL_HEIGHT: f32 = 150.0;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, movement, flip_sprite).run_if(in_state(InGameState::Running)),
        );
    }
}

#[derive(Default, PartialEq, Eq)]
pub(crate) enum Direction {
    #[default]
    Right,
    Left,
}

#[derive(Component)]
#[require(Transform)]
pub(crate) struct Player {
    velocity: f32,
    direction: Direction,
}

impl Player {
    pub(crate) fn new(
        asset_server: &AssetServer,
        layouts: &mut Assets<TextureAtlasLayout>,
        animation_layouts: &mut Assets<AnimationTextureAtlasLayout>,
    ) -> impl Bundle {
        let animation_layout = AnimationTextureAtlasLayout::from_json(
            include_str!("../assets/character_spritesheet.json"),
            layouts,
        )
        .expect("expected to load spritesheet layout");

        let animation_layout_handle = animation_layouts.add(animation_layout.clone());

        (
            Player {
                velocity: 0.0,
                direction: Direction::default(),
            },
            AnimatedSprite::new(
                asset_server.load("character_spritesheet.png"),
                animation_layout_handle,
                &animation_layout,
                "idle",
                24,
            ),
            Transform::from_translation(Vec3::new(0., PLAYER_Y, 10.))
                .with_scale(Vec3::splat(SCALE)),
            // Uncomment to visualize collision rectangles
            // children![(
            //     Sprite::from_color(Color::WHITE, Vec2::new(COLL_WIDTH, COLL_HEIGHT)),
            //     Transform::default().with_scale(Vec3::splat(1.0 / SCALE))
            // )],
        )
    }
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut player: Query<&mut Player>) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    let mut velocity = 0.0;

    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        velocity -= VELOCITY_X;
    }

    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        velocity += VELOCITY_X;
    }

    player.velocity = velocity.clamp(-VELOCITY_X, VELOCITY_X);
}

fn movement(
    time: ResMut<Time>,
    mut query: Query<(&mut Transform, &mut Player, &mut AnimatedSprite)>,
) {
    let Ok((mut transform, mut player, mut animation)) = query.single_mut() else {
        return;
    };

    let translation_x = player.velocity * time.delta_secs();
    transform.translation.x = (transform.translation.x + translation_x).clamp(MIN_X, MAX_X);

    if translation_x < 0.0 {
        player.direction = Direction::Left;
    } else if translation_x > 0.0 {
        player.direction = Direction::Right;
    }

    if translation_x == 0.0 {
        animation.animation_name = "idle".to_string();
    } else {
        animation.animation_name = "walk".to_string();
    }
}

fn flip_sprite(mut query: Query<(&mut Sprite, &Player)>) {
    let Ok((mut sprite, player)) = query.single_mut() else {
        return;
    };

    sprite.flip_x = player.direction != Direction::default();
}
