use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    animation::{AnimatedSprite, AnimationTextureAtlasLayout},
    game::GameState,
};

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_level)
            .add_systems(Update, handle_escape.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), teardown_level);
    }
}

#[derive(Component)]
pub(crate) struct LevelEntity;

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animation_layouts: ResMut<Assets<AnimationTextureAtlasLayout>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.single() else { return };
    let mut bg = Sprite::from_image(asset_server.load("background.png"));
    bg.custom_size = Some(Vec2::new(window.width(), window.height()));

    commands.spawn((bg, LevelEntity));

    let animation_layout = AnimationTextureAtlasLayout::from_json(
        include_str!("../assets/character_spritesheet.json"),
        &mut layouts,
    )
    .expect("expected to load spritesheet layout");

    let animation_layout_handle = animation_layouts.add(animation_layout.clone());

    commands.spawn((
        AnimatedSprite::new(
            asset_server.load("character_spritesheet.png"),
            animation_layout_handle,
            &animation_layout,
            "idle",
            24,
        ),
        Transform::from_translation(Vec3::new(0., -250., 10.)).with_scale(Vec3::splat(0.25)),
        LevelEntity,
    ));
}

fn handle_escape(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    };

    next_state.set(GameState::Menu);
}

fn teardown_level(mut commands: Commands, entities: Query<Entity, With<LevelEntity>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}
