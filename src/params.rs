use atomic_float::AtomicF64;

// TODO: module & rack params should be part of the synth's internal param state
pub struct Module {
    name: String
}

pub struct Rack {
    pub color: egui::Color32,
    pub name: String,
    pub modules: Vec<Module>,
}
impl Rack {
    pub fn new() -> Self {
        Rack {
            color: egui::Color32::BROWN,
            name: "default rack name".to_owned(),
            modules: Vec::new(),
        }
    }
}

// ----------------------------------------- //

pub struct ParamState {
    pub amplitude: AtomicF64,
    pub rack_list: Vec<Rack>,
}

impl Default for ParamState {
    fn default() -> ParamState {
        ParamState {
            amplitude: AtomicF64::new(0.5),
            rack_list: vec![Rack::new(), Rack::new(), Rack::new()],
        }
    }
}

// ----------------------------------------- //

/*
impl PluginParameters for MyState {
    // the `get_parameter` function reads the value of a parameter.
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.amplitude.get(),
            _ => 0.0,
        }
    }

    // the `set_parameter` function sets the value of a parameter.
    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.amplitude.set(val),
            _ => (),
        }
    }

    // This is what will display underneath our control.  We can
    // format it into a string that makes the most since.
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", (self.amplitude.get() - 0.5) * 2f32),
            _ => "".to_string(),
        }
    }

    // This shows the control's name.
    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Amplitude",
            _ => "",
        }
        .to_string()
    }
}*/