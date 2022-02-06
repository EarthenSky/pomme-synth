
//#[macro_use] // use all macros from vst
//extern crate vst;

extern crate priority_queue;

mod params;     // daw visisble params?
mod interface;  // 
mod editor;     // gui & visuals
mod synth;      // the main synth (vst2.4 stuff)
mod logic;      // math & processing
mod utils;      // helper objects & functions

// TODO: do we need to include the vst crate here?

fn main() {
    #[cfg(feature = "gui_only")] 
    {
        let synth = synth::PommeSynth::create(None);
        synth.show_editor();
    }
}