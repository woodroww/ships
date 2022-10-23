use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy::sprite::collide_aabb::collide;

// Components are the data associated with entities.
// Component: just a normal Rust data type. generally scoped to a single piece of functionality
//     Examples: position, velocity, health, color, name

/*
pub struct Materials {
    player: Handle<ColorMaterial>,
    laser: Handle<ColorMaterial>,
}
    */

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
pub const SHIP_SIZE: Vec2 = Vec2 { x: 500.0 * SHIP_SCALE, y: 413.0 * SHIP_SCALE };

fn main() {
    let width = 900.0;
    let height = 500.0;
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
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
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(keyboard_input_system)
        .add_system(laser_system)
        .add_system(player_fire)
        .add_system(check_laser_time)
        .add_system(check_for_collisions)
        .add_system(chack_for_game_over)
        .run();
}

fn chack_for_game_over(
    mut ships: Query<&mut Ship>,
) {
    for ship in &mut ships {
        if ship.health == 0 {
            info!("game over");
        }
    }
}

fn check_for_collisions(
    mut commands: Commands,
    yellow_lasers: Query<(Entity, &Transform, &YellowLaser)>,
    red_lasers: Query<(Entity, &Transform, &RedLaser)>,
    mut ships: Query<(&Transform, &mut Ship)>,
) {
    for (ship_transform, mut ship) in &mut ships {
        if ship.color == "red" {
            for (entity, laser_transform, _laser) in &yellow_lasers {
                let collision = collide(
                    laser_transform.translation,
                    LASER_SIZE,
                    ship_transform.translation,
                    SHIP_SIZE,
                );
                if let Some(_collision) = collision {
                    info!("collision!");
                    ship.health -= 1;
                    commands.entity(entity).despawn_recursive();
                }
            }
        } else {
            for (entity, laser_transform, _laser) in &red_lasers {
                let collision = collide(
                    laser_transform.translation,
                    LASER_SIZE,
                    ship_transform.translation,
                    SHIP_SIZE,
                );
                if let Some(_collision) = collision {
                    info!("collision!");
                    ship.health -= 1;
                    commands.entity(entity).despawn_recursive();
                }
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
    kb: Res<Input<KeyCode>>,
    red_lasers: Query<(&Transform, &RedLaser)>,
    yellow_lasers: Query<(&Transform, &YellowLaser)>,
    mut ships: Query<(&Transform, &mut Ship)>,
    asset_server: Res<AssetServer>,
) {
    let ship_width = 500.0 * 0.1;

    for (ship_transform, mut ship) in &mut ships {
        let color = ship.color.to_owned();
        if (color == "red" && kb.pressed(KeyCode::Space))
            || (color == "yellow" && kb.pressed(KeyCode::Back))
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
                    let laser_asset_name = color.to_owned() + "_laser.png";
                    if color == "red" {
                        commands
                            .spawn()
                            .insert_bundle(SpriteBundle {
                                texture: asset_server.load(&laser_asset_name),
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
                                texture: asset_server.load(&laser_asset_name),
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

