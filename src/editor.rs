// -------------------- //
// gui / editor

use std::sync::Arc;
use core::sync::atomic::Ordering;

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

    #[cfg(feature = "gui_only")]
    pub fn open_blocking(&self) {
        EguiWindow::open_blocking(
            PommeEditor::egui_settings(),
            self.param_state.clone(),
            Self::egui_init,
            Self::egui_render,
        );
    }

    pub fn open_parented(&mut self, parent: ParentWindow) {
        let window_handle = EguiWindow::open_parented(
            &parent,
            PommeEditor::egui_settings(),
            self.param_state.clone(),
            Self::egui_init,
            Self::egui_render,
        );

        self.window_handle = Some(window_handle);
    }

    fn egui_settings() -> Settings {
        Settings {
            window: baseview::WindowOpenOptions {
                title: String::from("pommesynth-egui"),
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
                scale: WindowScalePolicy::SystemScaleFactor,
            },
            render_settings: RenderSettings::default(),
        }
    }

    fn egui_init(_egui_ctx: &CtxRef, _queue: &mut Queue, _state: &mut Arc<ParamState>) {
        // Called once before the first frame. Allows you to do setup code

        // NOTE: since egui is imgui, this will be very helpful!

    }

    fn egui_render(egui_ctx: &CtxRef, _queue: &mut Queue, state: &mut Arc<ParamState>) {
        // NOTE: when accessing parameters from our parameter state, we can use relaxed ordering 
        // because this is the only location where the param state can be modified.

        let master_bus = egui::panel::SidePanel::right("master bus").resizable(false);
        master_bus.show(&egui_ctx, |ui| {
            ui.heading("Master Bus"); 
            
            let mut val = state.amplitude.load(Ordering::Relaxed);
            if ui.add(egui::Slider::new(&mut val, 0.0..=1.0).text("Gain")).changed()
            {
                state.amplitude.store(val, Ordering::Relaxed);
            }

        });
        
        egui::panel::CentralPanel::default().show(&egui_ctx, |ui| {
            ui.heading("Pomme Synth");
            ui.separator();

            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right().with_cross_justify(true), |ui| {
                    for rack in &*state.rack_list {
                        rack.draw(ui);
                    }
                });
            });
        });
    }
}

// -------------------------------------- //
// param struct draw functions

impl crate::params::Rack {
    pub fn draw(&self, ui: &mut egui::Ui) {
        ui.group(|ui: &mut egui::Ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(self.name.to_owned()); // TODO: this is a copy (slow)
                ui.label(self.name.to_owned()); // TODO: this is a copy (slow)
                ui.label(self.name.to_owned()); // TODO: this is a copy (slow)
                //ui.separator();
                //ui.set_width(200.0);

                //ui.set_height(ui.available_height());
                //ui.allocate_space(ui.available_size());
            });
        });
    }
}

// ------------------------------------- //

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
