use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::WorldInspectorPlugin;

// Components are the data associated with entities.
// Component: just a normal Rust data type. generally scoped to a single piece of functionality
//     Examples: position, velocity, health, color, name

pub struct Materials {
    red_laser: Handle<Image>,
    red_space_ship: Handle<Image>,
    yellow_laser: Handle<Image>,
    yellow_space_ship: Handle<Image>,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
}

struct GameOverEvent {
    winner: String,
}

enum GameState {
    GameOver,
    Playing,
    //   Reset,
}

struct ShipGame {
    state: GameState,
}

// reflect things for bevy_inspector_egui
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Ship {
    color: String,
    health: u8,
    laser_timer: Timer,
    fire_delay_passed: bool,
}

#[derive(Component)]
struct ScoreBoard;

#[derive(Component)]
struct ScoreEvent {
    loser: String,
}

#[derive(Component)]
struct RedText;
#[derive(Component)]
struct YellowText;

#[derive(Component)]
struct YellowLaser;

#[derive(Component)]
struct RedLaser;

// Entity: a collection of components with a unique id
//     Examples: Entity1 { Name("Alice"), Position(0, 0) },

// System: runs logic on entities, components, and resources
//     Examples: move system, damage system

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const MAX_LASERS: usize = 5;
pub const LASER_SIZE: Vec2 = Vec2 { x: 55.0, y: 17.0 };
pub const SHIP_SCALE: f32 = 0.1;
// original image 500 × 413
pub const SHIP_SIZE: Vec2 = Vec2 {
    x: 500.0 * SHIP_SCALE,
    y: 413.0 * SHIP_SCALE,
};


fn main() {
    let width = 900.0;
    let height = 500.0;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ShipGame {
            state: GameState::Playing,
        })
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width,
            height,
            title: "Bevy Tutorial".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        })
        .register_type::<Ship>()
        //       .register_inspectable::<ShipGame>()
        .add_startup_system(load_resources)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_scoreboard)
        .add_system(keyboard_input_system)
        .add_system(laser_system)
        .add_system(player_fire)
        .add_system(check_laser_time)
        .add_system(check_for_collisions)
        .add_system(reset_player)
        .add_system(game_over)
        .add_system(score_board)
        .add_event::<GameOverEvent>()
        .add_event::<ScoreEvent>()
        .run();
}

fn game_over(mut game_over_events: EventReader<GameOverEvent>) {
    for event in game_over_events.iter() {
        info!("Game Over! {} wins!", event.winner);
    }
}

fn check_for_collisions(
    mut commands: Commands,
    yellow_lasers: Query<(Entity, &Transform, &YellowLaser)>,
    red_lasers: Query<(Entity, &Transform, &RedLaser)>,
    mut ships: Query<(&Transform, &mut Ship)>,
    mut game_over_event: EventWriter<GameOverEvent>,
    mut score_event: EventWriter<ScoreEvent>,
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

fn player_fire(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    red_lasers: Query<&RedLaser>,
    yellow_lasers: Query<&YellowLaser>,
    mut ships: Query<(&Transform, &mut Ship)>,
    game: Res<ShipGame>,
    materials: Res<Materials>,
) {
    if let GameState::GameOver = game.state {
        return;
    }
    let ship_width = 500.0 * 0.1;

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
                        ship_transform.translation.x - ship_width
                    } else {
                        ship_transform.translation.x + ship_width
                    };
                    let y = ship_transform.translation.y;
                    if color == "red" {
                        commands
                            .spawn()
                            .insert_bundle(SpriteBundle {
                                texture: materials.red_laser.clone(),
                                transform: Transform {
                                    translation: Vec3 { x, y, z: 1.0 },
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(RedLaser {});
                    } else {
                        commands
                            .spawn()
                            .insert_bundle(SpriteBundle {
                                texture: materials.yellow_laser.clone(),
                                transform: Transform {
                                    translation: Vec3 { x, y, z: 1.0 },
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(YellowLaser {});
                    }
                    ship.fire_delay_passed = false;
                }
            }
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut ships: Query<(&mut Ship, &mut Transform)>,
    windows: ResMut<Windows>,
    time: Res<Time>,
) {
    let velocity = 200.0 * time.delta_seconds();

    let window = windows.primary();
    let width = window.width();
    let height = window.height();
    //info!("window w: {}, h: {}", width, height);

    for (ship, mut transform) in &mut ships {
        // original image 500 × 413
        let ship_width = 500.0 * transform.scale.x;
        let ship_height = 413.0 * transform.scale.y;

        if ship.color == "yellow" {
            // up
            if keyboard_input.pressed(KeyCode::E)
                && transform.translation.y + velocity + ship_height / 2.0 < height / 2.0
            {
                transform.translation.y += velocity;
            }
            // down
            if keyboard_input.pressed(KeyCode::D)
                && transform.translation.y - velocity - ship_height / 2.0 > -height / 2.0
            {
                transform.translation.y -= velocity;
            }
            // right
            if keyboard_input.pressed(KeyCode::F)
                && transform.translation.x + velocity + (ship_width / 2.0) < 0.0
            {
                transform.translation.x += velocity;
            }
            // left
            if keyboard_input.pressed(KeyCode::S)
                && transform.translation.x - velocity - (ship_width / 2.0) > -width / 2.0
            {
                transform.translation.x -= velocity;
            }
        }

        if ship.color == "red" {
            // up
            if keyboard_input.pressed(KeyCode::I)
                && transform.translation.y + velocity + ship_height / 2.0 < height / 2.0
            {
                transform.translation.y += velocity;
            }
            // down
            if keyboard_input.pressed(KeyCode::K)
                && transform.translation.y - velocity - ship_height / 2.0 > -height / 2.0
            {
                transform.translation.y -= velocity;
            }
            // right
            if keyboard_input.pressed(KeyCode::L)
                && transform.translation.x + velocity + ship_width / 2.0 < width / 2.0
            {
                transform.translation.x += velocity;
            }
            // left
            if keyboard_input.pressed(KeyCode::J)
                && transform.translation.x - velocity - (ship_width / 2.0) > 0.0
            {
                transform.translation.x -= velocity;
            }
        }
    }
}

fn reset_player(
    mut ships: Query<(&mut Transform, &mut Ship)>,
    mut game_over: EventReader<GameOverEvent>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    let event = game_over.iter().nth(0);
    if event.is_some() {
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
        // this "red" shouldn't matter as we are not actually updating the health
        // we just want the score board updated
        score_event.send(ScoreEvent {
            loser: "red".to_string(),
        });
    }
}

fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Materials {
        red_laser: asset_server.load("red_laser.png"),
        red_space_ship: asset_server.load("spaceship_red.png"),
        yellow_laser: asset_server.load("yellow_laser.png"),
        yellow_space_ship: asset_server.load("spaceship_yellow.png"),
        font: asset_server.load("fonts/SFNSMono.ttf"),
        font_size: 90.0,
        color: Color::WHITE,
    });
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let spaceship_size = (55, 40);
    // original image 500 × 413

    // red = pygame.Rect(x, y, width, height)
    // red = pygame.Rect(700, 300, SPACESHIP_SIZE[0], SPACESHIP_SIZE[1])
    // yellow = pygame.Rect(100, 300, SPACESHIP_SIZE[0], SPACESHIP_SIZE[1])

    // pygame window WIDTH, HEIGHT = 900, 500

    let red_transform = Transform {
        rotation: Quat::from_rotation_z(-90.0 * core::f32::consts::PI / 180.0),
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
        laser_timer: Timer::from_seconds(0.2, true),
        fire_delay_passed: true,
    };

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("spaceship_red.png"),
            transform: red_transform,
            ..default()
        })
        .insert(ship_component)
        .insert(Name::new("red ship"));

    let yellow_transform = Transform {
        rotation: Quat::from_rotation_z(90.0 * core::f32::consts::PI / 180.0),
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
        laser_timer: Timer::from_seconds(0.2, true),
        fire_delay_passed: true,
    };

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("spaceship_yellow.png"),
            transform: yellow_transform,
            ..default()
        })
        .insert(ship_component)
        .insert(Name::new("yellow ship"));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
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

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("10", text_style.clone()).with_alignment(text_alignment),
            transform: left_score,
            ..default()
        })
        .insert(ScoreBoard)
        .insert(YellowText);

    let right_score = Transform {
        translation: Vec3 {
            x: width / 2.0 - x_padding,
            y: y_location,
            z: 1.0,
        },
        ..default()
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("10", text_style.clone()).with_alignment(text_alignment),
            transform: right_score,
            ..default()
        })
        .insert(ScoreBoard)
        .insert(RedText);
}

fn score_board(
    ships: Query<&Ship>,
    mut score_events: EventReader<ScoreEvent>,
    mut red_text: Query<&mut Text, (With<RedText>, Without<YellowText>)>,
    mut yellow_text: Query<&mut Text, (With<YellowText>, Without<RedText>)>,
    materials: Res<Materials>,
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
