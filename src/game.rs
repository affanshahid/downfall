use crate::{
    animation::AnimationPlugin, level::LevelPlugin, menu::MenuPlugin, player::PlayerPlugin,
};
use bevy::{prelude::*, window::WindowResolution};

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;
pub const MIN_X: f32 = -WIDTH / 2.0;
pub const MAX_X: f32 = WIDTH / 2.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WIDTH as u32, HEIGHT as u32),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            MenuPlugin,
            LevelPlugin,
            AnimationPlugin,
            PlayerPlugin,
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
