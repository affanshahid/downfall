use bevy::prelude::*;

use crate::{
    animation::{AnimatedSprite, AnimationTextureAtlasLayout},
    game::{InGameState, ScreenConstraints},
};

const VELOCITY_X: f32 = 300.0;
const SCALE: f32 = 0.25;
const PLAYER_Y_DELTA: f32 = 100.0;
pub(crate) const COLL_WIDTH: f32 = 80.0;
pub(crate) const COLL_HEIGHT: f32 = 150.0;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_input, movement, flip_sprite).run_if(in_state(InGameState::Running)),
        )
        .add_systems(
            Update,
            handle_resize
                .run_if(in_state(InGameState::Running).and(resource_changed::<ScreenConstraints>)),
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
        constraints: ScreenConstraints,
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
            Transform::from_translation(Vec3::new(
                0.,
                constraints.min_y + (PLAYER_Y_DELTA * constraints.scale),
                10.,
            ))
            .with_scale(Vec3::splat(SCALE * constraints.scale)),
            // Uncomment to visualize collision rectangles
            // children![(
            //     Sprite::from_color(
            //         Color::srgba(1.0, 1.0, 1.0, 0.3),
            //         Vec2::new(COLL_WIDTH, COLL_HEIGHT)
            //     ),
            //     Transform::default().with_scale(Vec3::splat(1.0 / SCALE))
            // )],
        )
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Player>,
    constraints: Res<ScreenConstraints>,
) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    let scaled_velocity = VELOCITY_X * constraints.scale;
    let mut velocity = 0.0;

    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
        velocity -= scaled_velocity;
    }

    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
        velocity += scaled_velocity;
    }

    player.velocity = velocity.clamp(-scaled_velocity, scaled_velocity);
}

fn movement(
    time: ResMut<Time>,
    mut query: Query<(&mut Transform, &mut Player, &mut AnimatedSprite)>,
    constraints: Res<ScreenConstraints>,
) {
    let Ok((mut transform, mut player, mut animation)) = query.single_mut() else {
        return;
    };

    let translation_x = player.velocity * time.delta_secs();
    transform.translation.x =
        (transform.translation.x + translation_x).clamp(constraints.min_x, constraints.max_x);

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

fn handle_resize(
    constraints: Res<ScreenConstraints>,
    mut player: Query<&mut Transform, With<Player>>,
    mut previous_scale: Local<Option<f32>>,
) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    let scale_ratio = match *previous_scale {
        Some(prev) => constraints.scale / prev,
        None => constraints.scale,
    };

    player.scale = Vec3::splat(SCALE * constraints.scale);
    player.translation.x *= scale_ratio;
    player.translation.y = constraints.min_y + (PLAYER_Y_DELTA * constraints.scale);
    *previous_scale = Some(constraints.scale);
}
