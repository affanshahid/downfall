use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch, window::PrimaryWindow};

const STARTING_DEBRIS_TIMER_SECS: u64 = 1;

use crate::{
    animation::AnimationTextureAtlasLayout,
    debris::{Debris, DebrisData},
    game::{GameState, InGameState},
    player::{COLL_HEIGHT, COLL_WIDTH, Player},
};

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_level)
            .add_systems(
                Update,
                (spawn_debris, check_collision, update_score)
                    .run_if(in_state(InGameState::Running)),
            )
            .add_systems(Update, handle_escape)
            .add_systems(OnExit(GameState::InGame), teardown_level)
            .init_resource::<ScoreStopwatch>()
            .insert_resource(DebrisTimer(Timer::new(
                Duration::from_secs(STARTING_DEBRIS_TIMER_SECS),
                TimerMode::Repeating,
            )));
    }
}

#[derive(Component)]
struct LevelEntity;

#[derive(Component)]
struct ScoreText;

#[derive(Resource, Deref, DerefMut, Default)]
struct ScoreStopwatch(Stopwatch);

#[derive(Resource, Deref, DerefMut)]
struct DebrisTimer(Timer);

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animation_layouts: ResMut<Assets<AnimationTextureAtlasLayout>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut score_stopwatch: ResMut<ScoreStopwatch>,
) {
    let Ok(window) = window.single() else { return };
    let mut bg = Sprite::from_image(asset_server.load("background.png"));
    bg.custom_size = Some(Vec2::new(window.width(), window.height()));

    commands.spawn((bg, LevelEntity));

    commands.spawn((
        Player::new(&asset_server, &mut layouts, &mut animation_layouts),
        LevelEntity,
    ));

    commands.spawn((
        Node {
            margin: UiRect::axes(px(110), px(10)),
            ..default()
        },
        Text::new("Score: 0"),
        TextColor(Color::BLACK),
        ScoreText,
        LevelEntity,
    ));
    score_stopwatch.reset();
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

fn check_collision(
    debris_data: Res<DebrisData>,
    player: Query<&Transform, With<Player>>,
    debris: Query<(&Transform, &Debris)>,
    mut next_state: ResMut<NextState<InGameState>>,
    mut score_stopwatch: ResMut<ScoreStopwatch>,
) {
    let Ok(transform) = player.single() else {
        return;
    };

    let player_rect = Rect::from_center_size(
        transform.translation.truncate(),
        Vec2::new(COLL_WIDTH, COLL_HEIGHT),
    );

    for (transform, debris) in debris.iter() {
        let definition = &debris_data.definitions[debris.definition_idx];
        let debris_rect = Rect::from_center_size(
            transform.translation.truncate(),
            Vec2::new(definition.coll_width, definition.coll_height),
        );

        if !player_rect.intersect(debris_rect).is_empty() {
            next_state.set(InGameState::GameOver);
            score_stopwatch.pause();
            return;
        }
    }
}

fn update_score(
    mut score: Query<&mut Text, With<ScoreText>>,
    mut score_stopwatch: ResMut<ScoreStopwatch>,
    time: Res<Time>,
) {
    let Ok(mut text) = score.single_mut() else {
        return;
    };

    text.0 = format!(
        "Score: {}",
        score_stopwatch.tick(time.delta()).elapsed_secs().floor()
    );
}
