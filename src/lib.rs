// TODO: reminder to not copy octasine's code, but just use it as a template to get up to speed quickly

#[macro_use] // use all macros from vst
extern crate vst;

extern crate priority_queue;

mod params;     // daw visisble params?
mod editor;     // managing window & connection to parent (daw)
mod gui;        // iced gui code

mod synth;      // the main synth (vst2.4 stuff)
mod logic;      // math & processing
mod utils;      // helper objects & functions

use synth::PommeSynth;

#[cfg(not(feature = "gui_only"))]
plugin_main!(PommeSynth);