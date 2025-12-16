use crate::{level::LevelPlugin, menu::MenuPlugin};
use bevy::{prelude::*, window::WindowResolution};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1280, 720),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            MenuPlugin,
            LevelPlugin,
        ))
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
