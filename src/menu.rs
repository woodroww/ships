use crate::{GameState, GameAssets};
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_main_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(start_button_clicked)
                    .with_system(quit_button_clicked),
            );
    }
}

fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            game_state.set(GameState::Gameplay).unwrap();
        }
    }
}

fn quit_button_clicked(
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            exit.send(AppExit);
        }
    }
}

fn spawn_main_menu(mut commands: Commands, my_assets: Res<GameAssets>) {
    let start_button = spawn_button(&mut commands, &my_assets, "Start", my_assets.ufo_green.clone());
    commands.entity(start_button).insert(StartButton).insert(Name::new("StartButton"));

    let quit_button = spawn_button(&mut commands, &my_assets, "Quit", my_assets.ufo_red.clone());
    commands.entity(quit_button).insert(QuitButton).insert(Name::new("QuitButton"));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuUIRoot)
        .insert(Name::new("Menu"))
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "SpaceShips",
                    TextStyle {
                        font: my_assets.font.clone(),
                        font_size: 70.0,
                        color: my_assets.color,
                    },
                ),
                ..default()
            })
            .insert(Name::new("GameTitle"));
        })
        .add_child(start_button)
        .add_child(quit_button);
}

fn spawn_button(
    commands: &mut Commands,
    my_assets: &Res<GameAssets>,
    text: &str,
    img: Handle<Image>,
) -> Entity {
    let mut button_margin = UiRect::all(Val::Percent(2.0));
    button_margin.left = Val::Percent(-10.0);

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(91.0), Val::Px(91.0)),
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                margin: button_margin,
                ..default()
            },
            image: img.into(),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexStart,
                    margin: UiRect::all(Val::Percent(3.0)),
                    position: UiRect::new(Val::Px(0.0), Val::Px(-110.0), Val::Px(20.0), Val::Px(0.0)),
                    ..default()
                },
                text: Text::from_section(
                    text.to_string(),
                    TextStyle {
                        font: my_assets.font.clone(),
                        font_size: 44.0,
                        color: my_assets.color,
                    },
                ),
                ..default()
            });
        })
        .id()
}
