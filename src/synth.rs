// -------------------- //
// vst stuff

use vst::plugin::{Info, Plugin};
use vst::editor::Editor;
use vst::plugin::HostCallback;

// use log::{info};

use crate::editor::PommeEditor;
use crate::logic::PommeLogic;

pub struct PommeSynth {
    editor: Option<PommeEditor>,
    logic: PommeLogic, // TODO: maybe just change this into a bunch of function calls?
    host: Option<HostCallback>, // NOTE: leave this for now
}

impl PommeSynth {
    fn create(host: Option<HostCallback>) -> PommeSynth {        
        let editor = PommeEditor::new();

        PommeSynth {
            editor: Some(editor),
            logic: PommeLogic {},
            host: host,
        }
    }
}

impl Default for PommeSynth {    
    fn default() -> Self {
        Self::create(None)
    }
}

// vst2.4 api interaction
impl Plugin for PommeSynth {
    fn new(host: HostCallback) -> PommeSynth {
        Self::create(Some(host))
    }

    fn init(&mut self) {
        // TODO: ask editor for this info if need be
        //info!("loaded with host vst version: {}", self.host.vst_version()); // logging
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Pomme Synth".to_string(),
            unique_id: 31254, // Used by hosts to differentiate between plugins

            ..Default::default()
        }
    }

    // OHhhh, when called, this removes our editor from the synth.... for some reason.
    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}