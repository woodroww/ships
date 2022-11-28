use crate::{GameAssets, GameOverEvent, GameState, ScoreEvent, Ship};
use bevy::prelude::*;

#[derive(Component)]
struct ScoreBoard;

#[derive(Component)]
struct RedText;

#[derive(Component)]
struct YellowText;

#[derive(Component)]
struct WinnerText;

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(new_spawn_scoreboard)
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(reset_player))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(game_over)
                    .with_system(score_board),
            );
    }
}

fn game_over(
    mut game_over_events: EventReader<GameOverEvent>,
    mut game_state: ResMut<State<GameState>>,
    mut winner_text: Query<&mut Text, With<WinnerText>>,
    materials: Res<GameAssets>,
) {
    for event in game_over_events.iter() {
        game_state.set(GameState::MainMenu).unwrap();
        let mut winner = winner_text.single_mut();
        
        let win_color = match event.winner.as_str() {
            "Red" => Color::RED,
            "Yellow" => Color::YELLOW,
            _ => Color::GRAY,
        };
        let winner_text = TextSection::new(
            format!(" {} wins!", event.winner),
            TextStyle {
                font: materials.font.clone(),
                font_size: 45.0,
                color: win_color,
            }
        );

        let _text_alignment = TextAlignment::CENTER;

        let game_over_text = TextSection::new(
            "Game Over!",
            TextStyle {
                font: materials.font.clone(),
                font_size: 45.0,
                color: Color::WHITE,
            }
        );

        *winner = Text::from_sections([game_over_text, winner_text]);
    }
}

fn reset_player(
    mut ships: Query<(&mut Transform, &mut Ship)>,
    mut score_event: EventWriter<ScoreEvent>,
    mut winner_text: Query<&mut Text, With<WinnerText>>,
    materials: Res<GameAssets>,
) {
    for (mut ship_transform, mut ship) in &mut ships {
        if ship.color == "red" {
            ship_transform.translation = Vec3 {
                x: 250.0,
                y: 0.0,
                z: 1.0,
            };
        } else {
            ship_transform.translation = Vec3 {
                x: -250.0,
                y: 0.0,
                z: 1.0,
            };
        }
        ship.health = 10;
    }
    // we need to update the score board
    score_event.send(ScoreEvent {
        loser: "red".to_string(),
    });

    let mut winner = winner_text.single_mut();
    let text_style = TextStyle {
        font: materials.font.clone(),
        font_size: materials.font_size,
        color: materials.color,
    };
    let text_alignment = TextAlignment::CENTER;
    *winner = Text::from_section("".to_string(), text_style.clone())
            .with_alignment(text_alignment);
}

fn score_board(
    ships: Query<&Ship>,
    mut score_events: EventReader<ScoreEvent>,
    mut red_text: Query<&mut Text, (With<RedText>, Without<YellowText>)>,
    mut yellow_text: Query<&mut Text, (With<YellowText>, Without<RedText>)>,
    materials: Res<GameAssets>,
) {
    let text_style = TextStyle {
        font: materials.font.clone(),
        font_size: materials.font_size,
        color: materials.color,
    };
    let text_alignment = TextAlignment::CENTER;
    for event in score_events.iter() {
        for ship in ships.iter() {
            if event.loser == "red" && ship.color == "red" {
                if let Ok(mut text) = red_text.get_single_mut() {
                    *text = Text::from_section(ship.health.to_string(), text_style.clone())
                        .with_alignment(text_alignment);
                }
            } else {
                if let Ok(mut text) = yellow_text.get_single_mut() {
                    *text = Text::from_section(ship.health.to_string(), text_style.clone())
                        .with_alignment(text_alignment);
                }
            }
        }
    }
}

fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
) {
    let font = asset_server.load("fonts/SFNSMono.ttf");
    let text_style = TextStyle {
        font,
        font_size: 90.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::CENTER;

    let window = windows.primary();
    let width = window.width();
    let height = window.height();
    let x_padding = 100.0;
    let y_padding = x_padding * (height / width);
    let y_location = height / 2.0 - y_padding;

    let left_score = Transform {
        translation: Vec3 {
            x: -width / 2.0 + x_padding,
            y: y_location,
            z: 1.0,
        },
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("10", text_style.clone()).with_alignment(text_alignment),
            transform: left_score,
            ..default()
        },
        ScoreBoard,
        YellowText,
    ));

    let right_score = Transform {
        translation: Vec3 {
            x: width / 2.0 - x_padding,
            y: y_location,
            z: 1.0,
        },
        ..default()
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("10", text_style.clone()).with_alignment(text_alignment),
            transform: right_score,
            ..default()
        },
        ScoreBoard,
        RedText,
    ));
}

fn new_spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
) {
    let font = asset_server.load("fonts/SFNSMono.ttf");
    let text_style = TextStyle {
        font,
        font_size: 90.0,
        color: Color::WHITE,
    };
    let vertical = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        };

    let horizontal = NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                position_type: PositionType::Relative,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Row,
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        };

    commands
        .spawn(vertical)
        .with_children(|commands| {
            commands.spawn(horizontal)
                .with_children(|commands| {
                    commands.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::all(Val::Percent(3.0)),
                            ..default()
                        },
                        text: Text::from_section("10", text_style.clone()),
                        ..default()
                    })
                    .insert(ScoreBoard)
                    .insert(YellowText);

                    commands.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::all(Val::Percent(3.0)),
                            ..default()
                        },
                        text: Text::from_section("", text_style.clone()),
                        ..default()
                    })
                    .insert(WinnerText);

                    commands.spawn(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::all(Val::Percent(3.0)),
                            ..default()
                        },
                        text: Text::from_section("10", text_style.clone()),
                        ..default()
                    })
                    .insert(ScoreBoard)
                    .insert(RedText);
                });
        });
}
