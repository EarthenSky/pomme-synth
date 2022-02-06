# week1 
- get vst host for testing -> https://vaporsoft.net/creating-an-audio-plugin-with-rust-vst/ as reference
- implement a basic plugin & basic ui using https://github.com/greatest-ape/OctaSine as reference -> baseview i think?
- octasine uses: vst & baseview for gui
- https://github.com/emilk/egui#goals for gui instead of iced -> write custom widgets if needed.
- got a window showing up using baseview
- fml found a template doing exactly what I wanted... https://github.com/DGriffin91/egui_baseview_test_vst2/blob/main/src/lib.rs
- got egui_webview working (albeit with a bit of glitchy behaviour)

# week... 2
- got a polyphonic osc working & with correct event handling
- determined that egui will not work for my cases, since it is pretty buggy -> will use iced instead 
  - https://www.reddit.com/r/rust/comments/rmvwvy/gui_could_someone_please_explain_the_differences/
- 

# res:
- https://github.com/antonok-edm/ampli-Fe
- https://github.com/greatest-ape/OctaSine
- https://icons8.com/lunacy -> for designing gui

# immediate todo:
- implement a rough outline of the gui -> figure out how to build & test ONLY the gui, without the vst part -> pass the function to another cargo module?
- implement multiple osc modules
- implement volume & pitch knobs
- implement simple env (adsr)
- implement a filter
- implement a little dropdown window with settings?

# todos
- will want to look into simd stuff
- https://github.com/free-audio/clap/blob/b21634849ee95ad341d4aa406d2e011e816ae68a/include/clap/plugin-factory.h as a backend?
- look for a standard MIDI crate
- TODO: when writing simd support, write a macro to make it so that the math is very clear, then do an hn post about it.
- analyze https://github.com/askeksa/Cinter to see if I'm missing anything or not
- I think druid gui is more in line with what this synth is trying to accomplish (& for the daw later)

### teach-notes:
- rust workspaces are interesting, and useful for large projects that are composed of multiple parts. However, for smaller projects they may make things more difficult.
- What is an immediate mode gui? An interface where each command immediately updates the display. For example, drawing a box would not send input to a buffer, or do any other kind of pre-processing.
  - Based on a reccomendations by the developer of octasine, and some random person on stackoverflow who supposedly did their phd on imgui (and claims it is highly performant), I'm going to be using egui for this project.
- Rust generics are not the simplest, but are easy to understand once you learn them. (TODO: teach generics when needed)

#### days done:
-
