use crate::consts::{BASE_SPEED, DISTANCE};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::prelude::*;
use core::f32::consts::PI;
use serde_derive::{Deserialize, Serialize};
use std::io::Read;

const HARDCODED_CONFIG: &str = "name = \"Test song\"\r\nfilename = \"territory.ogg\"\r\n\r\narrows = [\r\n    { click_time = 2.30, speed = \"Slow\", direction = \"Up\" },\r\n    { click_time = 3.30, speed = \"Fast\", direction = \"Down\" },\r\n    { click_time = 4.30, speed = \"Slow\", direction = \"Right\" },\r\n    { click_time = 5.30, speed = \"Slow\", direction = \"Left\" },\r\n    { click_time = 6.30, speed = \"Medium\", direction = \"Up\" },\r\n    { click_time = 7.30, speed = \"Slow\", direction = \"Down\" },\r\n    { click_time = 8.30, speed = \"Fast\", direction = \"Right\" },\r\n    { click_time = 9.30, speed = \"Slow\", direction = \"Left\" },\r\n    { click_time = 10.30, speed = \"Slow\", direction = \"Up\" },\r\n    { click_time = 11.30, speed = \"Slow\", direction = \"Down\" },\r\n    { click_time = 12.30, speed = \"Medium\", direction = \"Right\" },\r\n    { click_time = 13.30, speed = \"Slow\", direction = \"Left\" },\r\n    { click_time = 14.30, speed = \"Slow\", direction = \"Up\" },\r\n    { click_time = 15.30, speed = \"Fast\", direction = \"Down\" },\r\n    { click_time = 16.30, speed = \"Slow\", direction = \"Right\" },\r\n    { click_time = 17.30, speed = \"Slow\", direction = \"Left\" },\r\n    { click_time = 17.90, speed = \"Medium\", direction = \"Up\" },\r\n    { click_time = 18.90, speed = \"Slow\", direction = \"Up\" },\r\n    { click_time = 19.90, speed = \"Medium\", direction = \"Down\" },\r\n    { click_time = 20.30, speed = \"Fast\", direction = \"Down\" },\r\n    { click_time = 20.90, speed = \"Medium\", direction = \"Up\" },\r\n    { click_time = 21.90, speed = \"Slow\", direction = \"Up\" },\r\n    { click_time = 22.90, speed = \"Fast\", direction = \"Up\" },\r\n    { click_time = 23.90, speed = \"Medium\", direction = \"Down\" },\r\n    { click_time = 24.30, speed = \"Fast\", direction = \"Down\" },\r\n    { click_time = 24.90, speed = \"Medium\", direction = \"Up\" },\r\n    { click_time = 25.90, speed = \"Slow\", direction = \"Left\" },\r\n    { click_time = 26.90, speed = \"Medium\", direction = \"Left\" },\r\n    { click_time = 27.90, speed = \"Fast\", direction = \"Right\" },\r\n    { click_time = 28.30, speed = \"Medium\", direction = \"Right\" },\r\n    { click_time = 28.90, speed = \"Medium\", direction = \"Left\" },\r\n    { click_time = 29.90, speed = \"Fast\", direction = \"Up\" },\r\n    { click_time = 30.90, speed = \"Medium\", direction = \"Up\" },\r\n    { click_time = 31.90, speed = \"Slow\", direction = \"Down\" },\r\n    { click_time = 32.30, speed = \"Medium\", direction = \"Down\" },\r\n    { click_time = 32.90, speed = \"Slow\", direction = \"Up\" },\r\n]\r\n";
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Left => [KeyCode::Left, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }

    /// Returns the correct y coordinate for an arrow with this direction
    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => 150.,
            Directions::Down => 50.,
            Directions::Left => -50.,
            Directions::Right => -150.,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    /// Returns actual speed at which the arrow should move
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }
    /// Speed multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Clone, Copy)]
/// Keeps track of when each arrow should spawn and it's speed and direction
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}
impl ArrowTime {
    fn new(arrow: &ArrowTimeToml) -> Self {
        let speed_value = arrow.speed.value();
        Self {
            spawn_time: arrow.click_time - (DISTANCE / speed_value) as f64,
            speed: arrow.speed,
            direction: arrow.direction,
        }
    }
}

#[derive(Resource)]

pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}

pub fn load_config(_path: &str, asset_server: &Res<AssetServer>) -> SongConfig {
    // TODO: Make online request working with wasm

    // let mut file = get_song();
    //
    // info!("Config created");
    //
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Couldn't read file into String");

    // Parse using toml and Serde
    let parsed: SongConfigToml =
        toml::from_str(HARDCODED_CONFIG).expect("Couldn't parse into SongConfigToml");

    // Process arrows
    let mut arrows = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect::<Vec<ArrowTime>>();
    // Sort arrows by spawn_time
    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // Load song audio and get the handle
    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}

#[derive(Deserialize, Debug)]
struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}
