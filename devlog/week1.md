# week1 
- get vst host for testing -> https://vaporsoft.net/creating-an-audio-plugin-with-rust-vst/ as reference
- implement a basic plugin & basic ui using https://github.com/greatest-ape/OctaSine as reference -> baseview i think?
- octasine uses: vst & baseview for gui
- https://github.com/emilk/egui#goals for gui instead of iced -> write custom widgets if needed.
- got a window showing up using baseview
- fml found a template doing exactly what I wanted... https://github.com/DGriffin91/egui_baseview_test_vst2/blob/main/src/lib.rs
- got egui_webview working (albeit with a bit of glitchy behaviour)

# res:
- https://github.com/antonok-edm/ampli-Fe
- 

# immediate todo
- implement static next
- implement a single polyphonic osc next
- 

# todos
- will want to look into simd stuff
- will want fastrand probably
- 

### teach-notes:
- rust workspaces are interesting, and useful for large projects that are composed of multiple parts. However, for smaller projects they may make things more difficult.
- What is an immediate mode gui? An interface where each command immediately updates the display. For example, drawing a box would not send input to a buffer, or do any other kind of pre-processing.
  - Based on a reccomendations by the developer of octasine, and some random person on stackoverflow who supposedly did their phd on imgui (and claims it is highly performant), I'm going to be using egui for this project.
- Rust generics are not the simplest, but are easy to understand once you learn them. (TODO: teach generics when needed)
- 

#### days done:
- tuesday
- wednesday
- thursday
- friday
- monday -> 0m
- 

