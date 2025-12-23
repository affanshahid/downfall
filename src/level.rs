use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

const STARTING_DEBRIS_TIMER_SECS: u64 = 3;

use crate::{
    animation::AnimationTextureAtlasLayout,
    debris::{Debris, DebrisData},
    game::GameState,
    player::Player,
};

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_level)
            .add_systems(
                Update,
                (handle_escape, spawn_debris).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), teardown_level)
            .insert_resource(DebrisTimer(Timer::new(
                Duration::from_secs(STARTING_DEBRIS_TIMER_SECS),
                TimerMode::Repeating,
            )));
    }
}

#[derive(Component)]
struct LevelEntity;

#[derive(Resource, Deref, DerefMut)]
struct DebrisTimer(Timer);

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
        Player::new(&asset_server, &mut layouts, &mut animation_layouts),
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

fn spawn_debris(
    mut commands: Commands,
    data: Res<DebrisData>,
    asset_server: Res<AssetServer>,
    mut debris_timer: ResMut<DebrisTimer>,
    time: Res<Time>,
) {
    if !debris_timer.tick(time.delta()).just_finished() {
        return;
    }

    commands.spawn((LevelEntity, Debris::new_random(&data, &asset_server)));
}
