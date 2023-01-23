use crate::firing::FiringPlugin;
use crate::scoring::ScoringPlugin;
use bevy::prelude::*;
//use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use crate::menu::MainMenuPlugin;
use crate::player::PlayerPlugin;

pub mod menu;
pub mod firing;
pub mod scoring;
pub mod player;

// Components are the data associated with entities.
// Component: just a normal Rust data type. generally scoped to a single piece of functionality
//     Examples: position, velocity, health, color, name

#[derive(Resource)]
pub struct GameAssets {
    red_laser: Handle<Image>,
    red_space_ship: Handle<Image>,
    yellow_laser: Handle<Image>,
    yellow_space_ship: Handle<Image>,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
    fire_sound: Handle<AudioSource>,
    hit_sound: Handle<AudioSource>,
    ufo_green: Handle<Image>,
    ufo_red: Handle<Image>,
}

struct GameOverEvent {
    winner: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    Gameplay,
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
struct ScoreEvent {
    loser: String,
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

fn main() {
    let width = 900.0;
    let height = 500.0;
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "SpaceShips".to_string(),
                width,
                height,
                ..default()
            },
            ..default()
        }))
        .add_plugin(AudioPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MainMenuPlugin)
        .add_plugin(FiringPlugin)
        .add_plugin(ScoringPlugin)
        .add_plugin(PlayerPlugin)
        .add_state(GameState::MainMenu)
        .insert_resource(ClearColor(CLEAR))
        .register_type::<Ship>()
        .add_startup_system_to_stage(StartupStage::PreStartup, load_resources)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_update(GameState::Gameplay).with_system(keyboard_input_system))
        .add_event::<GameOverEvent>()
        .add_event::<ScoreEvent>()
        .run();
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

fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fire_sound = asset_server.load("Gun+Silencer.mp3");
    let hit_sound = asset_server.load("Grenade+1.mp3");

    commands.insert_resource(GameAssets {
        red_laser: asset_server.load("red_laser.png"),
        red_space_ship: asset_server.load("spaceship_red.png"),
        yellow_laser: asset_server.load("yellow_laser.png"),
        yellow_space_ship: asset_server.load("spaceship_yellow.png"),
        font: asset_server.load("fonts/Roboto-Regular.ttf"),
        font_size: 90.0,
        color: Color::SILVER,
        fire_sound,
        hit_sound,
        ufo_green: asset_server.load("ufoGreen.png"),
        ufo_red: asset_server.load("ufoRed.png"),
    });
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

