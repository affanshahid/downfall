use bevy::{
    input_focus::InputDispatchPlugin,
    prelude::*,
    ui_widgets::{Activate, Button, UiWidgetsPlugins, observe},
};

use crate::game::GameState;

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiWidgetsPlugins, InputDispatchPlugin))
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), teardown_menu);
    }
}

#[derive(Component)]
pub struct MenuRoot;

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        GlobalTransform::default(),
        MenuRoot,
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::top(px(64)),
            ..default()
        },
        children![
            (
                Text::new("DOWNFALL"),
                TextFont {
                    font_size: 64.,
                    ..default()
                }
            ),
            (
                GlobalTransform::default(),
                Node {
                    width: px(200),
                    padding: UiRect::axes(px(16), px(8)),
                    margin: UiRect::top(px(180)),
                    border: UiRect::all(px(1)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(12)),
                Button,
                children![(Text::new("New Game"),)],
                observe(new_game),
            ),
            (
                GlobalTransform::default(),
                Node {
                    width: px(200),
                    padding: UiRect::axes(px(16), px(8)),
                    margin: UiRect::top(px(16)),
                    border: UiRect::all(px(1)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(12)),
                Button,
                children![(Text::new("Exit"),)],
                observe(exit),
            )
        ],
    ));
}

fn new_game(_: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

fn exit(_: On<Activate>, mut commands: Commands) {
    commands.write_message(AppExit::Success);
}

fn teardown_menu(mut commands: Commands, menu: Query<(&MenuRoot, Entity)>) {
    let Ok((_, menu)) = menu.single() else {
        return;
    };

    commands.entity(menu).despawn();
}
