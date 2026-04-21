
#[cfg(feature = "audio")]
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};

// ─── constants ────────────────────────────────────────────────────────────────
const SAMPLE_RATE: u32 = 44_100;
const MASTER_VOL: f32 = 0.18; // Keep below SFX so music stays ambient

pub struct MusicPlayer {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    handle: Option<OutputStreamHandle>,
    #[cfg(feature = "audio")]
    sink: Option<Sink>,

    muted: bool,
    current_track: usize,
}

impl MusicPlayer {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Self {
                    _stream: Some(stream),
                    handle: Some(handle),
                    sink: None,
                    muted: false,
                    current_track: 0,
                },
                Err(_) => Self {
                    _stream: None,
                    handle: None,
                    sink: None,
                    muted: false,
                    current_track: 0,
                },
            }
        }
        #[cfg(not(feature = "audio"))]
        Self { muted: false, current_track: 0 }
    }

    pub fn tick(&mut self) {
        #[cfg(feature = "audio")]
        {
            if self.muted {
                return;
            }
            let needs_next = match &self.sink {
                None => true,
                Some(s) => s.empty(),
            };
            if needs_next {
                self.play_next();
            }
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        #[cfg(feature = "audio")]
        {
            if self.muted {
                if let Some(s) = &self.sink { s.pause(); }
            } else {
                match &self.sink {
                    Some(s) if !s.empty() => s.play(),
                    _ => self.play_next(),
                }
            }
        }
    }

    #[cfg(feature = "audio")]
    fn play_next(&mut self) {
        let handle = match &self.handle {
            Some(h) => h,
            None => return,
        };
        let sink = match Sink::try_new(handle) {
            Ok(s) => s,
            Err(_) => return,
        };

        // Pre-render the track into PCM
        let samples = render_track(self.current_track);
        let buf = SamplesBuffer::new(1, SAMPLE_RATE, samples);
        sink.append(buf);
        self.sink = Some(sink);
        self.current_track = (self.current_track + 1) % TRACK_COUNT;
    }
}

// ─── Track catalogue ──────────────────────────────────────────────────────────

const TRACK_COUNT: usize = 3;

fn render_track(idx: usize) -> Vec<f32> {
    match idx {
        0 => render_boot_sequence(),   // C minor — mysterious, dark ambient
        1 => render_void_wanderer(),   // A minor pentatonic — hopeful space
        2 => render_mission_critical(),// D minor — tense, driving
        _ => render_boot_sequence(),
    }
}

// ─── Oscillator primitives ────────────────────────────────────────────────────

/// Multi-harmonic "organ" tone: fundamental + 2nd + 3rd harmonics
fn organ(freq: f32, t: f32, amp: f32) -> f32 {
    let tw = 2.0 * std::f32::consts::PI;
    (  (tw * freq * t).sin() * 0.5
     + (tw * freq * 2.0 * t).sin() * 0.25
     + (tw * freq * 3.0 * t).sin() * 0.125
    ) * amp
}

/// Soft bell: sine with exponential decay shaped for short notes
fn bell(freq: f32, t: f32, note_dur: f32, amp: f32) -> f32 {
    let progress = (t / note_dur).min(1.0);
    let env = (-progress * 3.5).exp();
    (2.0 * std::f32::consts::PI * freq * t).sin() * env * amp
}

/// Warm pad: 3 slightly-detuned sines mixed together
fn pad(freq: f32, t: f32, amp: f32) -> f32 {
    let tw = 2.0 * std::f32::consts::PI;
    let detune = freq * 0.008;
    (   (tw * freq * t).sin()
      + (tw * (freq + detune) * t).sin() * 0.7
      + (tw * (freq - detune * 0.5) * t).sin() * 0.5
    ) * (amp / 2.2)
}

/// Pulse accent: short sine blip on beat divisions
fn pulse(freq: f32, t: f32, amp: f32) -> f32 {
    let env = (-(t * 20.0).powi(2)).exp();
    (2.0 * std::f32::consts::PI * freq * t).sin() * env * amp
}

// ─── Track 1: Boot Sequence (C minor) ────────────────────────────────────────
// C2 drone organ, C minor arpeggio bell melody, Cm pad chords, subtle pulse
fn render_boot_sequence() -> Vec<f32> {
    let bpm = 76.0f32;
    let beat = 60.0 / bpm;
    let bars = 8;
    let total_secs = beat * 4.0 * bars as f32;
    let n_samples = (SAMPLE_RATE as f32 * total_secs) as usize;
    let mut out = vec![0.0f32; n_samples];

    // C minor scale degrees: C Eb G Bb C(oct)
    let scale: &[(f32, f32)] = &[
        // (freq_hz, start_beat)
        (261.63, 0.0),  // C4
        (311.13, 1.0),  // Eb4
        (392.00, 2.0),  // G4
        (466.16, 3.0),  // Bb4
        (523.25, 4.0),  // C5
        (392.00, 5.0),  // G4
        (311.13, 6.0),  // Eb4
        (261.63, 7.0),  // C4
        // repeat with variation
        (196.00, 8.0),  // G3
        (261.63, 9.0),  // C4
        (311.13, 10.0), // Eb4
        (523.25, 11.0), // C5
        (392.00, 12.0), // G4
        (261.63, 13.0), // C4
        (311.13, 14.0), // Eb4
        (392.00, 15.0), // G4
        (261.63, 16.0), // C4
        (311.13, 17.0), // Eb4
        (392.00, 18.0), // G4
        (466.16, 19.0), // Bb4
        (523.25, 20.0), // C5
        (392.00, 21.0), // G4
        (311.13, 22.0), // Eb4
        (261.63, 23.0), // C4 (long)
        (196.00, 25.0), // G3
        (130.81, 27.0), // C3 (resolve)
        (261.63, 29.0), // C4
        (311.13, 30.0), // Eb4
        (392.00, 31.0), // G4
    ];

    for i in 0..n_samples {
        let t_abs = i as f32 / SAMPLE_RATE as f32;
        let beat_abs = t_abs / beat;
        let mut s = 0.0f32;

        // Bass drone — C2 organ, always on
        s += organ(65.41, t_abs, 0.22);

        // Pad chord every 4 beats (Cm chord: C Eb G)
        let bar_phase = beat_abs % 4.0;
        let pad_env = if bar_phase < 3.5 { 1.0 } else { (4.0 - bar_phase) / 0.5 };
        s += pad(130.81, t_abs, 0.14 * pad_env); // C3
        s += pad(155.56, t_abs, 0.10 * pad_env); // Eb3
        s += pad(196.00, t_abs, 0.10 * pad_env); // G3

        // Melodic arpeggio — bell tones
        for &(freq, start_beat) in scale {
            let note_t = beat_abs - start_beat;
            if note_t >= 0.0 && note_t < 1.0 {
                s += bell(freq, note_t * beat, beat, 0.28);
            }
        }

        // Pulse on every beat
        let beat_t = (beat_abs % 1.0) * beat;
        s += pulse(98.0, beat_t, 0.08);

        out[i] = (s * MASTER_VOL).clamp(-1.0, 1.0);
    }
    out
}

// ─── Track 2: Void Wanderer (A minor pentatonic) ─────────────────────────────
// A2 organ drone, pentatonic bell melody, Am pad, syncopated pulse
fn render_void_wanderer() -> Vec<f32> {
    let bpm = 84.0f32;
    let beat = 60.0 / bpm;
    let bars = 8;
    let total_secs = beat * 4.0 * bars as f32;
    let n_samples = (SAMPLE_RATE as f32 * total_secs) as usize;
    let mut out = vec![0.0f32; n_samples];

    // A minor pentatonic: A C D E G
    let melody: &[(f32, f32)] = &[
        (440.00, 0.0),  // A4
        (523.25, 1.5),  // C5
        (587.33, 2.5),  // D5
        (659.25, 3.5),  // E5
        (784.00, 5.0),  // G5
        (659.25, 6.0),  // E5
        (587.33, 7.0),  // D5
        (523.25, 8.0),  // C5
        (440.00, 9.0),  // A4
        (329.63, 10.0), // E4
        (440.00, 11.0), // A4
        (392.00, 12.0), // G4
        (329.63, 13.5), // E4
        (293.66, 14.5), // D4
        (261.63, 15.5), // C4
        (220.00, 16.5), // A3
        (261.63, 17.5), // C4
        (329.63, 18.5), // E4
        (440.00, 19.5), // A4
        (523.25, 21.0), // C5
        (587.33, 22.0), // D5
        (659.25, 23.0), // E5
        (440.00, 24.5), // A4
        (329.63, 25.5), // E4
        (220.00, 26.5), // A3
        (329.63, 28.0), // E4
        (440.00, 29.0), // A4
        (523.25, 30.0), // C5
        (440.00, 31.0), // A4
    ];

    for i in 0..n_samples {
        let t_abs = i as f32 / SAMPLE_RATE as f32;
        let beat_abs = t_abs / beat;
        let mut s = 0.0f32;

        // Bass drone A2
        s += organ(110.0, t_abs, 0.20);

        // Pad: Am chord (A C E)
        let bar_phase = beat_abs % 4.0;
        let pad_env = if bar_phase < 3.6 { 1.0 } else { (4.0 - bar_phase) / 0.4 };
        s += pad(110.0, t_abs, 0.12 * pad_env);
        s += pad(130.81, t_abs, 0.09 * pad_env);
        s += pad(164.81, t_abs, 0.09 * pad_env);

        // Bell melody
        for &(freq, start_beat) in melody {
            let note_t = beat_abs - start_beat;
            if note_t >= 0.0 && note_t < 1.5 {
                s += bell(freq, note_t * beat, 1.5 * beat, 0.26);
            }
        }

        // Syncopated pulse on beat 2.5
        let bar_beat = beat_abs % 4.0;
        let synco_t = (bar_beat - 2.5).abs();
        if synco_t < 0.1 {
            s += pulse(110.0, synco_t * beat * 10.0, 0.07);
        }

        out[i] = (s * MASTER_VOL).clamp(-1.0, 1.0);
    }
    out
}

// ─── Track 3: Mission Critical (D minor) ─────────────────────────────────────
// D2 driving organ, faster arpeggio, Dm pad, strong downbeat pulse
fn render_mission_critical() -> Vec<f32> {
    let bpm = 96.0f32;
    let beat = 60.0 / bpm;
    let bars = 8;
    let total_secs = beat * 4.0 * bars as f32;
    let n_samples = (SAMPLE_RATE as f32 * total_secs) as usize;
    let mut out = vec![0.0f32; n_samples];

    // D minor: D F A C D(oct)
    let melody: &[(f32, f32)] = &[
        (293.66, 0.0),  // D4
        (349.23, 0.5),  // F4
        (440.00, 1.0),  // A4
        (523.25, 1.5),  // C5
        (587.33, 2.0),  // D5
        (523.25, 2.5),  // C5
        (440.00, 3.0),  // A4
        (349.23, 3.5),  // F4
        (293.66, 4.0),  // D4
        (220.00, 4.5),  // A3
        (261.63, 5.0),  // C4
        (293.66, 5.5),  // D4
        (349.23, 6.0),  // F4
        (440.00, 6.5),  // A4
        (523.25, 7.0),  // C5
        (440.00, 7.5),  // A4
        // Second half - higher register
        (587.33, 8.0),  // D5
        (698.46, 8.5),  // F5
        (880.00, 9.0),  // A5
        (698.46, 9.5),  // F5
        (587.33, 10.0), // D5
        (523.25, 10.5), // C5
        (440.00, 11.0), // A4
        (349.23, 11.5), // F4
        (293.66, 12.0), // D4
        (220.00, 12.5), // A3
        (146.83, 13.0), // D3
        (293.66, 14.0), // D4
        (349.23, 14.5), // F4
        (440.00, 15.0), // A4
        (523.25, 15.5), // C5
        (587.33, 16.0), // D5
        (440.00, 17.0), // A4
        (293.66, 18.0), // D4
        (220.00, 19.0), // A3
        (146.83, 20.0), // D3 (long)
        (293.66, 22.0), // D4
        (349.23, 23.0), // F4
        (440.00, 24.0), // A4
        (523.25, 25.0), // C5
        (587.33, 26.0), // D5
        (440.00, 27.0), // A4
        (349.23, 28.0), // F4
        (293.66, 29.0), // D4
        (220.00, 30.0), // A3
        (146.83, 31.0), // D3
    ];

    for i in 0..n_samples {
        let t_abs = i as f32 / SAMPLE_RATE as f32;
        let beat_abs = t_abs / beat;
        let mut s = 0.0f32;

        // Strong bass drone D2
        s += organ(73.42, t_abs, 0.24);

        // Dm pad (D F A)
        let bar_phase = beat_abs % 4.0;
        let pad_env = if bar_phase < 3.7 { 1.0 } else { (4.0 - bar_phase) / 0.3 };
        s += pad(146.83, t_abs, 0.13 * pad_env); // D3
        s += pad(174.61, t_abs, 0.10 * pad_env); // F3
        s += pad(220.00, t_abs, 0.10 * pad_env); // A3

        // Fast half-beat bell arpeggio
        for &(freq, start_beat) in melody {
            let note_t = beat_abs - start_beat;
            if note_t >= 0.0 && note_t < 0.5 {
                s += bell(freq, note_t * beat, 0.5 * beat, 0.30);
            }
        }

        // Strong downbeat pulse every beat
        let beat_t = (beat_abs % 1.0) * beat;
        s += pulse(73.42, beat_t, 0.12);
        // Accent on beat 1 and 3
        if (beat_abs % 4.0 < 0.1) || ((beat_abs % 4.0 - 2.0).abs() < 0.1) {
            s += pulse(146.83, beat_t, 0.10);
        }

        out[i] = (s * MASTER_VOL).clamp(-1.0, 1.0);
    }
    out
}
