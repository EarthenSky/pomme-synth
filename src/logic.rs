// -------------------- //
// dasp & synth logic

use std::sync::Arc;
use crate::params::ParamState;

use vst::api::Events;
use vst::event::Event;

use vst::buffer::Outputs;

pub struct PommeLogic {
    param_state: Arc<ParamState>, // enables writing to our current param state (like graphs & stuff) // TODO: create a better interface
    notes_held: u8,
}

const NOTE_ON: u8 = 144;
const NOTE_OFF: u8 = 128;

impl PommeLogic {
    pub fn new(param_state: Arc<ParamState>) -> Self {
        PommeLogic {
            notes_held: 0,
            param_state: param_state,
        }
    }

    pub fn process_events(&mut self, events: &Events) {
        for generic_event in events.events() {
            match generic_event {
                Event::Midi(event) => {
                    // Check if it's a noteon or noteoff event.
                    // https://www.midi.org/specifications/item/table-1-summary-of-midi-message
                    // might be helpful for future: https://www.cs.cmu.edu/~music/cmsip/readings/davids-midi-spec.htm
                    match event.data[0] {
                        NOTE_ON => self.notes_held += 1,
                        NOTE_OFF => self.notes_held -= 1,
                        _ => (),
                    }
                    // TODO: if we cared about the pitch of the note, it's stored in `ev.data[1]`.
                },
                _ => (),
            }
        }
    }

    pub fn process(&self, output_buffer: &mut Outputs<f32>) {
        if self.notes_held == 0 { 
            return;
        }

        let amplitude = self.param_state.amplitude.get();

        for output_channel in output_buffer.into_iter() {
            for output_sample in output_channel {
                // random in (-1.0, 1.0)
                *output_sample = amplitude * (fastrand::f32() - 0.5) * 2.0; 
            }
        }

    }
}