use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use crate::{
    GameAssets, GameState, RedLaser, Ship, YellowLaser,
    MAX_LASERS,
};

pub const SHIP_SCALE: f32 = 0.5;
// original image 500 × 413
pub const SHIP_SIZE: Vec2 = Vec2 {
    x: 98.0 * SHIP_SCALE,
    y: 75.0 * SHIP_SCALE,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_players)
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(player_fire)
            );
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    red_lasers: Query<&RedLaser>,
    yellow_lasers: Query<&YellowLaser>,
    mut ships: Query<(&Transform, &mut Ship)>,
    materials: Res<GameAssets>,
    audio: Res<Audio>,
) {
    for (ship_transform, mut ship) in &mut ships {
        let color = ship.color.to_owned();
        if (color == "red" && keyboard.pressed(KeyCode::Space))
            || (color == "yellow" && keyboard.pressed(KeyCode::Back))
        {
            let laser_count = if color == "red" {
                red_lasers.iter().len()
            } else {
                yellow_lasers.iter().len()
            };

            if laser_count < MAX_LASERS {
                if ship.fire_delay_passed {
                    let x = if color == "red" {
                        ship_transform.translation.x - SHIP_SIZE.x
                    } else {
                        ship_transform.translation.x + SHIP_SIZE.x
                    };
                    let y = ship_transform.translation.y;
                    if color == "red" {
                        commands.spawn((
                            SpriteBundle {
                                texture: materials.red_laser.clone(),
                                transform: Transform {
                                    translation: Vec3 { x, y, z: 1.0 },
                                    ..default()
                                },
                                ..default()
                            },
                            RedLaser {},
                        ));
                    } else {
                        commands.spawn((
                            SpriteBundle {
                                texture: materials.yellow_laser.clone(),
                                transform: Transform {
                                    translation: Vec3 { x, y, z: 1.0 },
                                    ..default()
                                },
                                ..default()
                            },
                            YellowLaser {},
                        ));
                    }
                    audio.play(materials.fire_sound.clone());
                    ship.fire_delay_passed = false;
                }
            }
        }
    }
}

fn spawn_players(mut commands: Commands, materials: Res<GameAssets>) {
    let red_transform = Transform {
        rotation: Quat::from_rotation_z(90.0 * core::f32::consts::PI / 180.0),
        scale: Vec3 {
            x: SHIP_SCALE,
            y: SHIP_SCALE,
            z: 1.0,
        },
        translation: Vec3 {
            x: 250.0,
            y: 0.0,
            z: 1.0,
        },
    };

    let ship_component = Ship {
        color: "red".to_string(),
        health: 10,
        laser_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        fire_delay_passed: true,
    };

    commands.spawn((
        SpriteBundle {
            texture: materials.red_space_ship.clone(),
            transform: red_transform,
            ..default()
        },
        ship_component,
        Name::new("red ship"),
    ));

    let yellow_transform = Transform {
        rotation: Quat::from_rotation_z(-90.0 * core::f32::consts::PI / 180.0),
        scale: Vec3 {
            x: SHIP_SCALE,
            y: SHIP_SCALE,
            z: 1.0,
        },
        translation: Vec3 {
            x: -250.0,
            y: 0.0,
            z: 1.0,
        },
    };
    let ship_component = Ship {
        color: "yellow".to_string(),
        health: 10,
        laser_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        fire_delay_passed: true,
    };

    commands.spawn((
        SpriteBundle {
            texture: materials.yellow_space_ship.clone(),
            transform: yellow_transform,
            ..default()
        },
        ship_component,
        Name::new("yellow ship"),
    ));
}
