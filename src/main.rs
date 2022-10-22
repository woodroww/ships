use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

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
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct Laser {
    color: String,
}

// Entity: a collection of components with a unique id
//     Examples: Entity1 { Name("Alice"), Position(0, 0) },

// System: runs logic on entities, components, and resources
//     Examples: move system, damage system

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

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
        .register_type::<Laser>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(keyboard_input_system)
        .add_system(toggle_override)
        .add_system(change_scale_factor)
        .add_system(laser_system)
        .add_system(player_fire)
        .run();
}

fn laser_system(
    windows: ResMut<Windows>,
    mut commands: Commands,
    mut lasers: Query<(Entity, &mut Laser, &mut Transform)>,
) {
    //info!("lasers: {}", lasers.iter().map(|(laser, _, _)| laser).collect::<Vec<&Laser>>().len());

    let window = windows.primary();
    let width = window.width();
    let laser_velocity = 7.0;

    for (entity, laser, mut transform) in &mut lasers {
        if laser.color == "yellow" {
            transform.translation.x += laser_velocity;
            if transform.translation.x > width / 2.0 {
                commands.entity(entity).despawn_recursive();
            }
        }

        if laser.color == "red" {
            transform.translation.x -= laser_velocity;
            if transform.translation.x < -width / 2.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}


fn fire_laser(
    color: &str,
    lasers: Query<(&Transform, &mut Laser)>,
    ships: Query<(&Transform, &Ship)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ship_width = 500.0 * 0.1;
    let max_lasers = 3;

    let transform_laser =
        lasers
            .iter()
            .filter_map(|(t, l)| if l.color == color { Some((t, l)) } else { None });
    let laser_count = transform_laser.collect::<Vec<(&Transform, &Laser)>>().len();
    if laser_count < max_lasers {
        let mut idk = ships
            .iter()
            .filter(|(_t, l)| if l.color == color { true } else { false })
            .into_iter();
        if let Some((ship_transform, _ship)) = idk.next() {

            let x = if color == "red" {
                ship_transform.translation.x - ship_width
            } else {
                ship_transform.translation.x + ship_width
            };
            let y = ship_transform.translation.y;
            let laser_asset_name = color.to_owned() + "_laser.png";
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
                .insert(Laser {
                    color: color.to_string(),
                });
        } else {
            info!("where is the red ship?");
        }
    }
}

fn player_fire(
    commands: Commands,
    kb: Res<Input<KeyCode>>,
    query: Query<(&Transform, &mut Laser)>,
    ships: Query<(&Transform, &Ship)>,
    asset_server: Res<AssetServer>,
) {
    if kb.pressed(KeyCode::Space) {
        fire_laser("red", query, ships, commands, asset_server);
    } else if kb.pressed(KeyCode::Back) {
        fire_laser("yellow", query, ships, commands, asset_server);
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut ships: Query<(&mut Ship, &mut Transform)>,
    windows: ResMut<Windows>,
) {
    let velocity = 5.0;

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
            x: 0.1,
            y: 0.1,
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
            x: 0.1,
            y: 0.1,
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

/// This system toggles scale factor overrides when enter is pressed
fn toggle_override(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Return) {
        window.set_scale_factor_override(window.scale_factor_override().xor(Some(1.)));
    }
}

/// This system changes the scale factor override when up or down is pressed
fn change_scale_factor(input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    if input.just_pressed(KeyCode::Up) {
        window.set_scale_factor_override(window.scale_factor_override().map(|n| n + 1.));
    } else if input.just_pressed(KeyCode::Down) {
        window.set_scale_factor_override(window.scale_factor_override().map(|n| (n - 1.).max(1.)));
    }
}
