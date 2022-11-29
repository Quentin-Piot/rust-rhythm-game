mod arrows;
mod consts;
mod types;

use arrows::ArrowsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode};
use bevy_editor_pls::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let config = types::load_config();

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Rhythm Game".to_string(),
                width: 1280.,
                height: 720.,
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_system(close_on_esc)
        .add_plugin(EditorPlugin)
        .add_plugin(ArrowsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_camera)
        .insert_resource(config)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
