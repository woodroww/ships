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
    let height = 900.0;
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height,
            title: "Bevy Tutorial".to_string(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().
        insert_bundle(SpriteBundle {
        texture: asset_server.load("spaceship_red.png"),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {

    commands.spawn().insert_bundle(Camera2dBundle::default());

   /* let mut camera = OrthographicCameraBundle::new_2d();
    
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
    */
}

