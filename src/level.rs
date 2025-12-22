use bevy::{prelude::*, window::PrimaryWindow};

use crate::{animation::AnimationTextureAtlasLayout, game::GameState, player::Player};

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

    commands.spawn((
        Player::new(asset_server, &mut layouts, &mut animation_layouts),
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
