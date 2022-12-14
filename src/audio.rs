use crate::consts::AppState;
use crate::time::ControlledTime;
use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    let secs = time.seconds_since_startup();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 1.5 && 1.5 <= secs {
        audio.play(config.song_audio.clone());
    }
}

pub struct CustomAudioPlugin;
impl Plugin for CustomAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(start_song));
    }
}
