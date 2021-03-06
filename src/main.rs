use std::env;

extern crate log;

// bevy
use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use board_plugin::resources::{BoardAssets, SpriteMaterial};
use board_plugin::BoardPlugin;

#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;

// [cfg(debug_assertions)]
// use bevy_inspector_egui::WorldInspectorParams;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

// Logging Setting
#[cfg(target_arch = "wasm32")]
use web_log::println;
// use web_log::{Console, ConsoleType, println};

fn init() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

fn main() {
    if let Ok(profile) = env::var("PROFILE") {
        println!("cargo:rustc-cfg=build={:?}", profile);
    }

    init();

    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Default Plugins
    .add_plugins(DefaultPlugins);

    // for DEBUG Plugins
    #[cfg(debug_assertions)]
    {
        // app.add_plugin(EditorPlugin)
        // app.insert_resource(WorldInspectorParams {
        //     despawnable_entities: true,
        //     highlight_changes: true,
        //     ..Default::default()
        // })
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin);
    }

    // Board Plugin
    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        safe_start: true,
        ..Default::default()
    })
    // .add_state(AppState::InGame)
    .add_state(AppState::Out)
    .add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
    .add_startup_system(setup_board)
    .add_system(state_handler);

    // Startup Camera
    app.add_startup_system(camera_setup);
    // Run App
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        }
    }
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Default::default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        // bomb_counter_font: asset_server.load("fonts/j.ttf"),
        bomb_counter_font: asset_server.load("fonts/NotoSansJP-Regular.otf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    });
    // Plugin activation
    state.set(AppState::InGame).unwrap();
}
