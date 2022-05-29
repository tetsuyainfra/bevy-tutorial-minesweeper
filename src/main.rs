use std::env;

extern crate log;

// bevy
use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;

// #[cfg(debug_assertions)]
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

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
    .add_plugin(BoardPlugin);

    // Startup Camera
    app.add_startup_system(camera_setup);
    // Run App
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
