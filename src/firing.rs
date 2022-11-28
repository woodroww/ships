pub struct FiringPlugin;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_kira_audio::prelude::*;

use crate::{
    GameAssets, GameOverEvent, GameState, RedLaser, ScoreEvent, Ship, YellowLaser, LASER_SIZE,
    SHIP_SIZE,
};

impl Plugin for FiringPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_something))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(laser_system)
                    .with_system(check_laser_time)
                    .with_system(check_for_collisions),
            );
    }
}

fn spawn_something() {}

fn laser_system(
    windows: ResMut<Windows>,
    mut commands: Commands,
    mut red_lasers: Query<(Entity, &RedLaser, &mut Transform), Without<YellowLaser>>,
    mut yellow_lasers: Query<(Entity, &YellowLaser, &mut Transform), Without<RedLaser>>,
    time: Res<Time>,
) {
    //info!("lasers: {}", lasers.iter().map(|(laser, _, _)| laser).collect::<Vec<&Laser>>().len());

    let window = windows.primary();
    let width = window.width();
    let laser_velocity = 600.0 * time.delta_seconds();

    for (entity, _laser, mut transform) in &mut yellow_lasers {
        transform.translation.x += laser_velocity;
        if transform.translation.x > width / 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }

    for (entity, _laser, mut transform) in &mut red_lasers {
        transform.translation.x -= laser_velocity;
        if transform.translation.x < -width / 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_laser_time(mut ships: Query<(&Transform, &mut Ship)>, time: Res<Time>) {
    for (_t, mut ship) in &mut ships {
        ship.laser_timer.tick(time.delta());
        if ship.laser_timer.just_finished() {
            ship.fire_delay_passed = true;
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    yellow_lasers: Query<(Entity, &Transform, &YellowLaser)>,
    red_lasers: Query<(Entity, &Transform, &RedLaser)>,
    mut ships: Query<(&Transform, &mut Ship)>,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut score_event: EventWriter<ScoreEvent>,
    audio: Res<Audio>,
    materials: Res<GameAssets>,
) {
    for (ship_transform, mut ship) in &mut ships {
        if ship.color == "red" {
            for (laser_entity, laser_transform, _laser) in &yellow_lasers {
                let collision = collide(
                    laser_transform.translation,
                    LASER_SIZE,
                    ship_transform.translation,
                    SHIP_SIZE,
                );
                if let Some(_collision) = collision {
                    ship.health -= 1;
                    commands.entity(laser_entity).despawn_recursive();
                    score_event.send(ScoreEvent {
                        loser: "red".to_string(),
                    });
                    audio.play(materials.hit_sound.clone());
                }
            }
            if ship.health == 0 {
                game_over_event.send(GameOverEvent {
                    winner: "yellow".to_string(),
                });
                break;
            }
        } else {
            for (laser_entity, laser_transform, _laser) in &red_lasers {
                let collision = collide(
                    laser_transform.translation,
                    LASER_SIZE,
                    ship_transform.translation,
                    SHIP_SIZE,
                );
                if let Some(_collision) = collision {
                    ship.health -= 1;
                    commands.entity(laser_entity).despawn_recursive();
                    score_event.send(ScoreEvent {
                        loser: "yellow".to_string(),
                    });
                    audio.play(materials.hit_sound.clone());
                }
            }
            if ship.health == 0 {
                game_over_event.send(GameOverEvent {
                    winner: "red".to_string(),
                });
                break;
            }
        }
    }
}
