/// Audio system for GitQuest.
///
/// Two layers:
///   1. SoundManager — one-shot sound effects (keypress, correct, error, etc.)
///   2. MusicPlayer  — continuous looping ambient background music with mute toggle
///
/// Design mirrors rebels-in-the-sky: _stream kept alive in struct so audio
/// device is not dropped. Sink::pause()/play() for instant mute/unmute.
pub mod music_player;

pub use music_player::MusicPlayer;

// ── Sound effects ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sound {
    KeyPress,
    Correct,
    Error,
    LevelComplete,
    Transition,
    GameComplete,
}

pub struct SoundManager {
    #[cfg(feature = "audio")]
    _stream: Option<rodio::OutputStream>,
    #[cfg(feature = "audio")]
    handle: Option<rodio::OutputStreamHandle>,
}

impl SoundManager {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match rodio::OutputStream::try_default() {
                Ok((stream, handle)) => Self {
                    _stream: Some(stream),
                    handle: Some(handle),
                },
                Err(_) => Self { _stream: None, handle: None },
            }
        }
        #[cfg(not(feature = "audio"))]
        Self {}
    }

    pub fn play(&self, sound: Sound) {
        #[cfg(feature = "audio")]
        {
            if let Some(h) = &self.handle {
                play_synth(h, sound);
            }
        }
        let _ = sound;
    }
}

// ── Synthesised SFX ───────────────────────────────────────────────────────────

#[cfg(feature = "audio")]
use std::time::Duration;

#[cfg(feature = "audio")]
use rodio::{
    source::{SineWave, Source},
    OutputStreamHandle, Sink,
};

#[cfg(feature = "audio")]
fn play_synth(handle: &OutputStreamHandle, sound: Sound) {
    macro_rules! tone {
        ($freq:expr, $dur:expr, $amp:expr) => {
            SineWave::new($freq)
                .take_duration(Duration::from_secs_f32($dur))
                .amplify($amp)
                .fade_in(Duration::from_millis(5))
        };
    }

    let sink = match Sink::try_new(handle) {
        Ok(s) => s,
        Err(_) => return,
    };

    match sound {
        Sound::KeyPress => {
            sink.append(tone!(800.0, 0.04, 0.18));
        }
        Sound::Correct => {
            sink.append(tone!(880.0, 0.10, 0.3));
            sink.append(tone!(1100.0, 0.12, 0.3));
        }
        Sound::Error => {
            sink.append(tone!(220.0, 0.15, 0.45));
        }
        Sound::LevelComplete => {
            sink.append(tone!(523.25, 0.13, 0.35));
            sink.append(tone!(659.25, 0.13, 0.35));
            sink.append(tone!(784.0,  0.13, 0.35));
            sink.append(tone!(1046.5, 0.40, 0.35));
        }
        Sound::Transition => {
            for &(f, d) in &[
                (200.0f32, 0.08f32),
                (400.0,    0.08),
                (700.0,    0.08),
                (1100.0,   0.08),
                (1600.0,   0.08),
                (2000.0,   0.20),
            ] {
                sink.append(tone!(f, d, 0.28));
            }
        }
        Sound::GameComplete => {
            sink.append(tone!(523.25, 0.11, 0.35));
            sink.append(tone!(659.25, 0.11, 0.35));
            sink.append(tone!(784.0,  0.11, 0.35));
            sink.append(tone!(987.77, 0.11, 0.35));
            sink.append(tone!(1046.5, 0.45, 0.35));
            sink.append(tone!(784.0,  0.11, 0.35));
            sink.append(tone!(1046.5, 0.70, 0.35));
        }
    }

    sink.detach();
}

// ── Synth oscillator (used by MusicPlayer too) ────────────────────────────────

/// Simple sine-wave oscillator that `Source` uses.
#[allow(dead_code)]
pub struct SynthSource {
    pub freq: f32,
    pub amplitude: f32,
    pub sample_rate: u32,
    pub total_samples: u32,
    pub current_sample: u32,
}

impl SynthSource {
    #[allow(dead_code)]
    pub fn new(freq: f32, duration_secs: f32, amplitude: f32) -> Self {
        let sample_rate = 44_100u32;
        Self {
            freq,
            amplitude,
            sample_rate,
            total_samples: (sample_rate as f32 * duration_secs) as u32,
            current_sample: 0,
        }
    }
}

impl Iterator for SynthSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current_sample >= self.total_samples {
            return None;
        }
        let t = self.current_sample as f32 / self.sample_rate as f32;
        let env = 1.0 - (self.current_sample as f32 / self.total_samples as f32);
        let sample = (2.0 * std::f32::consts::PI * self.freq * t).sin() * env * self.amplitude;
        self.current_sample += 1;
        Some(sample)
    }
}

impl rodio::source::Source for SynthSource {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.total_samples as f32 / self.sample_rate as f32,
        ))
    }
}
