mod arrows;
mod background;
mod consts;
mod score;
mod types;
mod ui;

use arrows::ArrowsPlugin;

use crate::background::BackgroundPlugin;
use crate::score::ScoreResource;
use crate::ui::UIPlugin;
use bevy::prelude::*;
use bevy::window::{close_on_esc, PresentMode};

pub const CLEAR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const WINDOW_WIDTH: f32 = 1280.;
pub const WINDOW_HEIGHT: f32 = 800.;

fn main() {
    let config = types::load_config();

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Rust Rhythm Game".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                fit_canvas_to_parent: true,
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .insert_resource(config)
        .insert_resource(ScoreResource::default())
        .add_system(close_on_esc)
        .add_plugin(UIPlugin)
        .add_plugin(ArrowsPlugin)
        .add_plugin(BackgroundPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
