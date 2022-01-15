// -------------------- //
// vst stuff

use std::sync::Arc;
use log::info;

use vst::plugin::{CanDo, Info, Plugin, Category};
use vst::plugin::HostCallback;
use vst::editor::Editor;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;

use crate::params::ParamState;
use crate::editor::PommeEditor;
use crate::logic::PommeLogic;

pub struct PommeSynth {
    editor: Option<PommeEditor>,
    logic: PommeLogic,
    _host: Option<HostCallback>, // NOTE: leave this for now
}

impl PommeSynth {
    fn create(host: Option<HostCallback>) -> PommeSynth {
        // init() goes here
        // TODO: figure out how the logging works
        simple_logging::log_to_file("pomme_synth.log", log::LevelFilter::Info).unwrap();

        let param_state = Arc::new(ParamState::default());

        let editor = PommeEditor::new(param_state.clone());
        let logic = PommeLogic::new(param_state.clone());

        PommeSynth {
            editor: Some(editor),
            logic: logic,
            _host: host,
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
            name: "Pomme Synth".to_owned(),
            vendor: "earthen_sky".to_owned(),
            version: 0001,
            unique_id: 7231254, // Used by hosts to differentiate between plugins

            inputs: 0,
            outputs: 2, 
            //parameters: 1, // TODO: do this later

            category: Category::Synth,

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

    // Here's the function that allows us to receive events
    fn process_events(&mut self, events: &Events) {
        // TODO: separate events at a higher level, like octasine
        // NOTE: we can raise events here if we need to
        self.logic.process_events(&events);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_input_buffer, mut output_buffer) = buffer.split();
    
        self.logic.process(&mut output_buffer);
    }

    // TODO: need to figure out what all these things are
    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent
            //| CanDo::ReceiveTimeInfo
            //| CanDo::SendEvents
            | CanDo::ReceiveEvents => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}