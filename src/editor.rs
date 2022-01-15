// -------------------- //
// gui / editor


use std::sync::Arc;

use baseview::{Size, WindowHandle, WindowScalePolicy};

use egui::CtxRef;
use egui_baseview::{EguiWindow, Queue, RenderSettings, Settings};

use vst::editor::Editor;

use crate::interface::ParentWindow;
use crate::params::ParamState;

pub struct PommeEditor {
    param_state: Arc<ParamState>,
    window_handle: Option<WindowHandle>,
    opened: bool,
}

impl PommeEditor {
    pub fn new(param_state: Arc<ParamState>) -> Self {
        Self {
            opened: false,
            window_handle: None,
            param_state: param_state,
        }
    }

    pub fn open_parented(&mut self, parent: ParentWindow) {
        let settings = Settings {
            window: baseview::WindowOpenOptions {
                title: String::from("pommesynth-baseview-ohyeah-egui"),
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
            },
            render_settings: RenderSettings::default(),
        };

        let window_handle = EguiWindow::open_parented(
            &parent,
            settings,
            self.param_state.clone(),
            Self::egui_init,
            Self::egui_render,
        );

        self.window_handle = Some(window_handle);
    }

    fn egui_init(_egui_ctx: &CtxRef, _queue: &mut Queue, _state: &mut Arc<ParamState>) {
        // Called once before the first frame. Allows you to do setup code    
    }

    fn egui_render(egui_ctx: &CtxRef, _queue: &mut Queue, state: &mut Arc<ParamState>) {
        egui::Window::new("egui-baseview pomme synth demo").show(&egui_ctx, |ui| {
            ui.heading("Pomme Synth");
            let mut val = state.amplitude.get();
            if ui
                .add(egui::Slider::new(&mut val, 0.0..=1.0).text("Gain"))
                .changed()
            {
                state.amplitude.set(val)
            }
            
        });
    }
}

pub const GUI_WIDTH: usize = 12 * 66;
pub const GUI_HEIGHT: usize = 12 * 61;

impl Editor for PommeEditor {
    fn size(&self) -> (i32, i32) {
        (GUI_WIDTH as i32, GUI_HEIGHT as i32)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut ::core::ffi::c_void) -> bool {
        if self.opened {
            return false;
        }

        self.open_parented(ParentWindow(parent));

        true
    }

    fn close(&mut self) {
        self.opened = false;
    }

    fn is_open(&mut self) -> bool {
        self.opened
    }
}
