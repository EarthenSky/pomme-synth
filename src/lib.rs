// TODO: reminder to not copy octasine's code, but just use it as a template to get up to speed quickly.

#[macro_use] // use all macros from vst
extern crate vst;

mod interface;
mod editor;
mod synth;
mod logic;

use synth::PommeSynth;

// NOTE: this is our 'main'
plugin_main!(PommeSynth);