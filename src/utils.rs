// TODO: call this file 'util'

use vst::event::MidiEvent;

// -------------------------- //
// Midi Pitch:

#[derive(Debug, Copy, Clone)]
pub struct MidiPitch {
    pub hz: f64,
}

impl MidiPitch {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    // Midi notes can be 0-127
    pub fn new(midi_pitch: u8) -> Self {
        Self {
            hz: Self::to_hz(midi_pitch),
        }
    }

    fn to_hz(midi_pitch: u8) -> f64 {
        let note_diff = f64::from(midi_pitch as i8 - Self::A4_PITCH);

        (note_diff / 12.0).exp2() * Self::A4_FREQ
    }
}

// -------------------------- //
// Key Velocity:

#[derive(Debug, Copy, Clone)]
pub struct KeyVelocity(pub f64);

impl Default for KeyVelocity {
    fn default() -> KeyVelocity {
        KeyVelocity(100.0 / 127.0)
    }
}

impl KeyVelocity {
    pub fn from_midi_velocity(midi_velocity: u8) -> Self {
        if midi_velocity == 0 {
            Self::default() // TODO: is this valid behaviour in standard DAWs / other plugins?
        } else {
            Self(f64::from(midi_velocity) / 127.0)
        }
    }
}

// -------------------------- //

// TODO: do we need this? what does phase really mean aside from bias?

/// Phase. value >= 0.0 && value < 1.0
#[derive(Debug, Copy, Clone)]
pub struct Phase(pub f64);

// -------------------------- //

// TODO:
// if no phase, I don't think we need this either.
#[derive(Debug, Copy, Clone)]
pub struct VoiceOperator {
    pub last_phase: Phase,
    //pub volume_envelope: (f32, f32, f32, f32), // adsr
}

impl Default for VoiceOperator {
    fn default() -> Self {
        Self {
            last_phase: Phase(0.0),
            //volume_envelope: (0.0, 0.0, 0.0, 0.0),
        }
    }
}

// -------------------------- //
// Voices:

#[derive(Debug, Clone)]
pub struct Voice {
    pub active: bool,
    pub midi_pitch: MidiPitch,
    pub key_pressed: bool,
    pub key_velocity: KeyVelocity, // TODO: integrate this
    pub elapsed_duration: f64,
    // envelope
}

impl Voice {
    pub fn new(midi_pitch: MidiPitch) -> Self {
        Self {
            active: false,
            midi_pitch: midi_pitch,
            elapsed_duration: 0.0,
            key_pressed: false,
            key_velocity: KeyVelocity::default(),
        }
    }
    
    // TODO: is inline neccesary? #[inline]
    pub fn press_key(&mut self, velocity: u8) {
        self.key_velocity = KeyVelocity::from_midi_velocity(velocity);
        self.key_pressed = true;
        self.elapsed_duration = 0.0;
    
        // TODO: restart envelope & voice based lfos & stuff here
    
        self.active = true;
    }
    
    // TODO: is inline neccesary? #[inline]
    pub fn release_key(&mut self) {
        self.key_pressed = false;
    }
    
    // TODO: is inline neccesary? #[inline]
    pub fn deactivate_if_tail_ended(&mut self) {
        // TODO: for envelope
        // deactivate if the decay is over
        // if all_envelopes_ended {    
        //     self.active = false;
        // }
        if !self.key_pressed {
            self.active = false;
        }
    }
}

// -------------------------- //
// Custom Note Event:

// TODO: should I call this MidiEvent, but reference it by path?

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct NoteEvent {
    pub event: u8,
    pub pitch: u8,
    pub velocity: u8,
    pub delta_frames: i32,  // num frames since the start of the current block
}
impl NoteEvent {
    pub fn from(e: &MidiEvent) -> NoteEvent {
        NoteEvent {
            event: e.data[0],
            pitch: e.data[1],
            velocity: e.data[2],
            delta_frames: e.delta_frames,
        }
    }
}
impl PartialEq for NoteEvent {
    fn eq(&self, other: &Self) -> bool {
        // We only care about the order of events in the current block, and we 
        // empty the queue when process is called.
        self.delta_frames == other.delta_frames
    }
}