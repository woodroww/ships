use bevy::prelude::*;
use crate::{GameState, GameOverEvent, GameAssets, YellowText, RedText, ScoreEvent, Ship};

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(reset_player))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(game_over)
                    .with_system(score_board)
            );
    }
}

fn game_over(
    mut game_over_events: EventReader<GameOverEvent>,
    mut game_state: ResMut<State<GameState>>
) {
    for event in game_over_events.iter() {
        info!("Game Over! {} wins!", event.winner);
        game_state.set(GameState::MainMenu).unwrap();
    }
}

fn reset_player(
    mut ships: Query<(&mut Transform, &mut Ship)>,
    mut score_event: EventWriter<ScoreEvent>,
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
