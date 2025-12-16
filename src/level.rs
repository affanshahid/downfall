use bevy::prelude::*;

use crate::game::GameState;

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

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100.),
            height: percent(100.),
            ..default()
        },
        ImageNode::new(asset_server.load("background.png")),
        LevelEntity,
    ));
}

fn handle_escape(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    };

    next_state.set(GameState::Menu);
}

fn teardown_level(mut commands: Commands, background: Query<Entity, With<LevelEntity>>) {
    let Ok(backgroud) = background.single() else {
        return;
    };

    commands.entity(backgroud).despawn();
}
