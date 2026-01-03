use crate::{
    animation::AnimationPlugin, debris::DebrisPlugin, level::LevelPlugin, menu::MenuPlugin,
    player::PlayerPlugin,
};
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized, WindowResolution},
};

pub(crate) const DEFAULT_WIDTH: f32 = 1280.0;
pub(crate) const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Downfall".to_string(),
                        canvas: Some("#game".to_string()),
                        resolution: WindowResolution::new(
                            DEFAULT_WIDTH as u32,
                            (DEFAULT_WIDTH / ASPECT_RATIO) as u32,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set({
                    #[cfg(target_arch = "wasm32")]
                    {
                        AssetPlugin {
                            file_path: "downfall-assets".to_string(),
                            meta_check: bevy::asset::AssetMetaCheck::Never,
                            ..default()
                        }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        AssetPlugin::default()
                    }
                }),
            MenuPlugin,
            LevelPlugin,
            AnimationPlugin,
            PlayerPlugin,
            DebrisPlugin,
        ))
        .init_resource::<ScreenConstraints>()
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, handle_screen_resize);
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

#[derive(Resource, Copy, Clone)]
pub(crate) struct ScreenConstraints {
    pub(crate) scale: f32,
    pub(crate) min_x: f32,
    pub(crate) max_x: f32,
    pub(crate) max_y: f32,
    pub(crate) min_y: f32,
}

impl ScreenConstraints {
    pub(crate) fn from_screen_width(width: f32) -> Self {
        let scale = width / DEFAULT_WIDTH;
        let height = width / ASPECT_RATIO;

        ScreenConstraints {
            scale,
            min_x: -width / 2.0,
            max_x: width / 2.0,
            max_y: height / 2.0,
            min_y: -height / 2.0,
        }
    }
}

impl Default for ScreenConstraints {
    fn default() -> Self {
        Self::from_screen_width(DEFAULT_WIDTH)
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn handle_screen_resize(
    mut events: MessageReader<WindowResized>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut constraints: ResMut<ScreenConstraints>,
    mut ui_scale: ResMut<UiScale>,
) {
    let Ok(mut window) = window.single_mut() else {
        return;
    };

    for event in events.read() {
        window
            .resolution
            .set(event.width, event.width / ASPECT_RATIO);

        *constraints = ScreenConstraints::from_screen_width(event.width);

        ui_scale.0 = constraints.scale;
    }
}
