// -------------------- //
// dasp & synth logic

use std::sync::Arc;
use array_init::array_init;

use crate::utils::{Voice, MidiPitch};
use crate::params::ParamState;

use vst::event::MidiEvent;

use vst::buffer::Outputs;

// TODO: do I need this?
/// Each SAMPLE_PASS_SIZE samples, load parameter changes and processing
/// parameter values (interpolated values where applicable)
const SAMPLE_PASS_SIZE: usize = 16;

pub struct PommeLogic {
    param_state: Arc<ParamState>, // enables writing to our current param state (like graphs & stuff) // TODO: create a better interface? // TODO: should this be a non-mutable ref?
    sample_rate: f64,
    voices: [Voice; 128],
}

// TODO: should this be in the same struct as regular pommeSynth? What happens when inlining is attempted?

impl PommeLogic {
    pub fn new(param_state: Arc<ParamState>) -> Self {
        PommeLogic {
            param_state: param_state,
            sample_rate: 44100.0,
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
        }
    }

    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    // NOTE: vst plugins render audio in chunks. First, they're given a bunch of notes with data that show up in that block,
    // then process() is called to process that data. I think process() is *only* called on segments between midi events.

    //TODO: see if inline actually changes the binary (likely won't) // #[inline(always)]
    pub fn process_midi_event(&mut self, midi_event: &MidiEvent) {
        // https://www.midi.org/specifications/item/table-1-summary-of-midi-message
        // might be helpful for future: https://www.cs.cmu.edu/~music/cmsip/readings/davids-midi-spec.htm

        const EVENT: usize = 0;
        const PITCH: usize = 1;
        const VELOCITY: usize = 2;

        match midi_event.data[EVENT] {
            144 => self.note_on(midi_event.data[PITCH], midi_event.data[VELOCITY]),
            128 => self.note_off(midi_event.data[PITCH]),
            _ => (),
        }
    }

    fn note_on(&mut self, pitch: u8, velocity: u8) {
        self.voices[pitch as usize].press_key(velocity);
    }

    fn note_off(&mut self, pitch: u8) {
        self.voices[pitch as usize].release_key();
    }
    
    pub fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    // NOTE: this is about where we would implement simd
    pub fn process_f32(&mut self, output_buffer: &mut Outputs<f32>, num_samples: usize) {
        let amplitude = self.param_state.amplitude.get();

        let per_sample = self.time_per_sample();
        
        // TODO: either use an iterator, or implement a "self.active_voices"... or this is optimal enough?
        let mut active_voices: Vec<&mut Voice> = self.voices.iter_mut().filter(|voice| voice.active).collect();

        let left_channel = output_buffer.get_mut(0);
        let right_channel = output_buffer.get_mut(1);
        for i in 0..num_samples {
            // TODO: do SAMPLE_PASS_SIZE sized passes (see octasine)
            
            let mut current_wave: f32 = 0.0;
            for voice in active_voices.iter_mut() {
                current_wave += square_wave(voice.elapsed_duration, voice.midi_pitch.hz) as f32; // TODO: there should be some optimal way to do these casts.
                voice.elapsed_duration += per_sample; // TODO: duration can be turned into u32, unless we go for longer than 10 hours or so
                voice.deactivate_if_tail_ended(); // TODO: move this; it feels inefficient
            }

            //current_wave /= active_voices.len() as f32;
            // TODO: normalize instead? -> probably not

            left_channel[i] = amplitude * current_wave;
            right_channel[i] = amplitude * current_wave;
        }

    }
}

// Waves:

// TODO: bias not needed?
// when we have a wave, we want to know: starting posistion & length in wave.
fn square_wave(index: f64, frequency: f64) -> f64 {
    let x = index;
    if x % (1.0/frequency) < (1.0/frequency) * 0.5 {
        1.0
    } else {
        -1.0
    }
}

fn sine_wave() {
    // TODO: this
}

fn saw_wave() {
    // TODO: this
}

fn triangle_wave() {
    // TODO: this
}

// random in (-1.0, 1.0)
fn random_wave() -> f32 {
    (fastrand::f32() - 0.5) * 2.0
}

