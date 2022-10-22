use bevy::prelude::*;

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_ships);
    }
}

fn add_ships(mut commands: Commands) {
    commands.spawn().insert(ShipHealth { hp: 5 });
}

// Components are the data associated with entities.
// Component: just a normal Rust data type. generally scoped to a single piece of functionality
//     Examples: position, velocity, health, color, name

#[derive(Component)]
struct ShipHealth {
    hp: u8,
}

#[derive(Component)]
struct Ship {
    color: String,
}

// Entity: a collection of components with a unique id
//     Examples: Entity1 { Name("Alice"), Position(0, 0) },

// System: runs logic on entities, components, and resources
//     Examples: move system, damage system

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    // pygame WIDTH, HEIGHT = 900, 500
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 900.0,
            height: 500.0,
            title: "Bevy Tutorial".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(keyboard_input_system)
        .run();
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut ships: Query<(&mut Ship, &mut Transform)>) {
    /*info!("ships: {}", ships.iter().collect::<Vec<&Ship>>().len());
    for ship in ships.iter() {
        info!(ship.color);
    }*/

    let velocity = 5.0;
    for (ship, mut transform) in &mut ships {
        if ship.color == "yellow" {
            // up
            if keyboard_input.pressed(KeyCode::E) {
                transform.translation.y += velocity;
            }
            // down
            if keyboard_input.pressed(KeyCode::D) {
                transform.translation.y -= velocity;
            }
            // right
            if keyboard_input.pressed(KeyCode::F) {
                transform.translation.x += velocity;
            }
            // left
            if keyboard_input.pressed(KeyCode::S) {
                transform.translation.x -= velocity;
            }
        }
        if ship.color == "red" {
            // up
            if keyboard_input.pressed(KeyCode::I) {
                transform.translation.y += velocity;
            }
            // down
            if keyboard_input.pressed(KeyCode::K) {
                transform.translation.y -= velocity;
            }
            // right
            if keyboard_input.pressed(KeyCode::L) {
                transform.translation.x += velocity;
            }
            // left
            if keyboard_input.pressed(KeyCode::J) {
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
    };

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("spaceship_red.png"),
            transform: red_transform,
            ..default()
        })
        .insert(ship_component);

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
    };

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("spaceship_yellow.png"),
            transform: yellow_transform,
            ..default()
        })
        .insert(ship_component);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn().insert_bundle(Camera2dBundle::default());
}
