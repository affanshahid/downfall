use crate::menu::MenuPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins, MenuPlugin))
            .init_state::<GameState>()
            .add_systems(Startup, setup_camera);
    }
}

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    #[default]
    Menu,
    InGame,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
