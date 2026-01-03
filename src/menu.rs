use crate::game::GameState;
use bevy::{
    input_focus::InputDispatchPlugin,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Activate, Button, UiWidgetsPlugins, observe},
};

pub(crate) const MENU_BG_COLOR: Color = Color::srgb_u8(43, 44, 47);

pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiWidgetsPlugins, InputDispatchPlugin))
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(
                Update,
                (button_hovered, handle_enter).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), teardown_menu);
    }
}

#[derive(Component)]
pub struct MenuRoot;

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
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
            BackgroundColor(MENU_BG_COLOR),
        ))
        .with_children(|commands| {
            commands.spawn((
                Text::new("DOWNFALL"),
                TextFont {
                    font_size: 64.,
                    ..default()
                },
            ));

            commands.spawn((
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
                Hovered::default(),
                BackgroundColor(Color::srgba(0., 0., 0., 0.)),
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(12)),
                Button,
                children![(Text::new("New Game"),)],
                observe(new_game),
            ));

            #[cfg(not(target_arch = "wasm32"))]
            commands.spawn((
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
                Hovered::default(),
                BackgroundColor(Color::srgba(0., 0., 0., 0.)),
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(12)),
                Button,
                children![(Text::new("Exit"),)],
                observe(exit),
            ));
        });
}

fn new_game(_: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

#[allow(unused)]
fn exit(_: On<Activate>, mut commands: Commands) {
    commands.write_message(AppExit::Success);
}

fn button_hovered(mut buttons: Query<(&mut BackgroundColor, &Hovered), With<Button>>) {
    for (mut bg, hovered) in buttons.iter_mut() {
        if hovered.get() {
            bg.0 = Color::WHITE.with_alpha(0.3);
        } else {
            bg.0 = Color::srgba(0., 0., 0., 0.);
        }
    }
}

fn handle_enter(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if !input.just_pressed(KeyCode::Enter) {
        return;
    }

    next_state.set(GameState::InGame);
}

fn teardown_menu(mut commands: Commands, menu: Query<Entity, With<MenuRoot>>) {
    let Ok(menu) = menu.single() else {
        return;
    };

    commands.entity(menu).despawn();
}
