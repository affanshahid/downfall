use crate::{
    animation::AnimationPlugin, debris::DebrisPlugin, level::LevelPlugin, menu::MenuPlugin,
    player::PlayerPlugin,
};
use bevy::{asset::AssetMetaCheck, prelude::*, window::WindowResolution};

#[cfg(target_arch = "wasm32")]
const ASSET_PATH: &str = "downfall-assets";

#[cfg(not(target_arch = "wasm32"))]
const ASSET_PATH: &str = "assets";

pub(crate) const WIDTH: f32 = 1280.0;
pub(crate) const HEIGHT: f32 = 720.0;
pub(crate) const MIN_X: f32 = -WIDTH / 2.0;
pub(crate) const MAX_X: f32 = WIDTH / 2.0;
pub(crate) const MAX_Y: f32 = HEIGHT / 2.0;
pub(crate) const MIN_Y: f32 = -HEIGHT / 2.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Downfall".to_string(),
                        canvas: Some("#game".to_string()),
                        resolution: WindowResolution::new(WIDTH as u32, HEIGHT as u32),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: ASSET_PATH.to_string(),
                    meta_check: {
                        #[cfg(target_arch = "wasm32")]
                        {
                            AssetMetaCheck::Never
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            AssetMetaCheck::default()
                        }
                    },
                    ..default()
                }),
            MenuPlugin,
            LevelPlugin,
            AnimationPlugin,
            PlayerPlugin,
            DebrisPlugin,
        ))
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .add_systems(Startup, setup_camera);
    }
}

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum GameState {
    #[default]
    Menu,
    InGame,
}

#[derive(Default, SubStates, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GameState = GameState::InGame)]
pub(crate) enum InGameState {
    #[default]
    Running,
    GameOver,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
