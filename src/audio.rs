use crate::types::SongConfig;
use bevy::prelude::*;

fn start_song(audio: Res<Audio>, time: Res<Time>, config: Res<SongConfig>) {
    let secs = time.elapsed_seconds_f64();
    let secs_last = secs - time.delta_seconds_f64();

    if secs_last <= 1.5 && 1.5 <= secs {
        audio.play(config.song_audio.clone());
    }
}

pub struct CustomAudioPlugin;
impl Plugin for CustomAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_song);
    }
}
