use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_kira_audio::prelude::*;

use crate::{
    GameAssets, GameOverEvent, GameState, RedLaser, ScoreEvent, Ship, YellowLaser, LASER_SIZE,
    player::SHIP_SIZE,
};

pub struct FiringPlugin;

impl Plugin for FiringPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_exit(GameState::Gameplay)
                    .with_system(reset_lasers)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(laser_movement)
                    .with_system(check_laser_delay)
                    .with_system(laser_collisions)
            );
    }
}

fn reset_lasers(
    mut commands: Commands,
    mut red_lasers: Query<(Entity, &RedLaser, &mut Transform), Without<YellowLaser>>,
    mut yellow_lasers: Query<(Entity, &YellowLaser, &mut Transform), Without<RedLaser>>)
{
    for (entity, _laser, _transform) in &mut yellow_lasers {
        commands.entity(entity).despawn_recursive();
    }
    for (entity, _laser, _transform) in &mut red_lasers {
        commands.entity(entity).despawn_recursive();
    }
}

fn laser_movement(
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

fn check_laser_delay(mut ships: Query<(&Transform, &mut Ship)>, time: Res<Time>) {
    for (_t, mut ship) in &mut ships {
        ship.laser_timer.tick(time.delta());
        if ship.laser_timer.just_finished() {
            ship.fire_delay_passed = true;
        }
    }
}

fn laser_collisions(
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
                    winner: "Yellow".to_string(),
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
                    winner: "Red".to_string(),
                });
                break;
            }
        }
    }
}
