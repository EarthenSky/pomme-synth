// -------------------- //
// dasp & synth logic

//use log::info;

use std::sync::Arc;
use core::sync::atomic::Ordering;

use priority_queue::PriorityQueue;
use array_init::array_init;

use crate::utils::{Voice, MidiPitch, NoteEvent};
use crate::params::ParamState;

use vst::event::MidiEvent;
use vst::buffer::Outputs;

// TODO: do I need this?
/// Each SAMPLE_PASS_SIZE samples, load parameter changes and processing
/// parameter values (interpolated values where applicable)
const SAMPLE_PASS_SIZE: usize = 16;

pub struct PommeLogic {
    param_state: Arc<ParamState>, // enables writing to our current param state (like graphs & stuff) // TODO: create a better interface?
    
    sample_rate: f64,
    block_size: i64,

    note_events: PriorityQueue<NoteEvent, i64>,
    voices: [Voice; 128],
}

// TODO: What happens when inlining is attempted?

impl PommeLogic {
    pub fn new(param_state: Arc<ParamState>) -> Self {
        PommeLogic {
            param_state: param_state,
            
            sample_rate: 44100.0,
            block_size: 512,
            
            note_events: PriorityQueue::new(),
            voices: array_init(|i| Voice::new(MidiPitch::new(i as u8))),
        }
    }

    fn time_per_sample(&self) -> f64 {
        1.0 / self.sample_rate
    }

    // NOTE: vst plugins render audio in chunks. First, they're given a bunch of notes with data that show up in that block,
    // then process() is called to process that data.

    pub fn push_midi_event(&mut self, midi_event: &MidiEvent) {
        let frames_to_end = self.block_size - midi_event.delta_frames as i64;
        self.note_events.push(NoteEvent::from(midi_event), frames_to_end);
    }

    pub fn pop_note_event(&mut self) -> Option<(NoteEvent, i64)> {
        if let Some((event, frames_to_end)) = self.note_events.pop() {
            Some((event, self.block_size - frames_to_end))
        } else {
            None
        }
    }

    // apply the affects of an event to the current state
    pub fn process_note_event(&mut self, note_event: NoteEvent)  {
        // might be helpful for future: https://www.cs.cmu.edu/~music/cmsip/readings/davids-midi-spec.htm
        
        // TODO: see octasine's implementation to see if I'm missing anything here

        match note_event.event {
            144 => self.note_on(note_event.pitch, note_event.velocity),
            128 => self.note_off(note_event.pitch),
            _ => (),
        }
    }

    // TODO: manually inline these functions?
    fn note_on(&mut self, pitch: u8, velocity: u8) {
        self.voices[pitch as usize].press_key(velocity);
    }
    fn note_off(&mut self, pitch: u8) {
        self.voices[pitch as usize].release_key();
    }
    
    pub fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    pub fn set_block_size(&mut self, size: i64) {
        self.block_size = size;
    }

    // NOTE: this is about where we would implement simd
    pub fn process_f32(&mut self, output_buffer: &mut Outputs<f32>, num_samples: usize) {
        // TODO: do parameter automation & interpolation

        // TODO: should I make a copy of the param_state right here, so we know it won't change 
        // the entire block (I don't want interprocess communication during the algorithm) -> probably.
        let amplitude = self.param_state.amplitude.load(Ordering::Relaxed);

        let per_sample = self.time_per_sample();

        let mut start_frame: usize;
        let mut end_frame: usize = 0;
        while end_frame < num_samples {
            start_frame = end_frame;

            // figure out when next event hits:

            let possible_event: Option<NoteEvent>;
            if let Some((event, delta_frames)) = self.pop_note_event() {
                end_frame = delta_frames as usize;
                possible_event = Some(event);
            } else {
                // this is the last iteration
                end_frame = num_samples;
                possible_event = None;
            }

            // generate signal until event starts:

            let left_channel = output_buffer.get_mut(0);
            let right_channel = output_buffer.get_mut(1);
            for i in start_frame..end_frame {
                // TODO: do SAMPLE_PASS_SIZE sized passes (see octasine)
                
                let mut current_wave: f64 = 0.0;
                // TODO: implement a "self.active_voices"... unless this is optimal enough?
                for voice in self.voices.iter_mut().filter(|voice| voice.active) {
                    current_wave += square_wave(voice.elapsed_duration, voice.midi_pitch.hz);
                    voice.elapsed_duration += per_sample;
                    voice.deactivate_if_tail_ended(); // TODO: move this; it feels inefficient
                }
    
                //current_wave /= active_voices.len() as f32;
                // TODO: normalize instead? -> probably not

                // cast to f32 last
                left_channel[i] = (amplitude * current_wave) as f32; 
                right_channel[i] = (amplitude * current_wave) as f32;
            }

            // process event:
            
            if let Some(note_event) = possible_event {
                self.process_note_event(note_event);
            }
        }
    }

}

// Waves:

//TODO: see if inline actually changes the binary (likely won't) // #[inline(always)]
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

