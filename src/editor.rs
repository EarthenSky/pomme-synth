// -------------------- //
// editor

use std::sync::Arc;


use baseview::{Size, WindowScalePolicy};
use iced_baseview::{IcedWindow, Settings};

//use egui::CtxRef;
//use egui_baseview::{EguiWindow, Queue, RenderSettings, Settings};

use vst::editor::Editor;

use crate::gui::PommeGui;
use crate::params::ParamState;

pub struct PommeEditor {
    params: Arc<ParamState>,
    opened: bool,
}

impl PommeEditor {
    pub fn new(params: Arc<ParamState>) -> Self {
        Self {
            params: params,
            opened: false,
        }
    }

    #[cfg(feature = "gui_only")]
    pub fn open_blocking(&self) {
        let settings = Self::iced_settings(self.params.clone());

        IcedWindow::<PommeGui>::open_blocking(
            settings,
        );
    }

    pub fn open_parented(&mut self, parent: ParentWindow) {
        let settings = Self::iced_settings(self.params.clone());

        IcedWindow::<PommeGui>::open_parented(
            &parent,
            settings,
        );
    }

    fn iced_settings(param_state: Arc<ParamState>) -> Settings<Arc<ParamState>> {
        Settings {
            window: baseview::WindowOpenOptions {
                title: String::from("iced_baseview pomme synth title"),
                size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64), // TODO: connect this with the other gui sizing stuff
                
                // Windows currently needs scale factor 1.0, or GUI contents
                // will be too large for window (from octasine)
                #[cfg(not(target_os = "windows"))]
                scale: WindowScalePolicy::SystemScaleFactor,
                #[cfg(target_os = "windows")]
                scale: WindowScalePolicy::ScaleFactor(1.0),
            },
            ignore_non_modifier_keys: false, // TODO: want true eventually?
            flags: param_state,
        }
    }

    /*
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
    }*/
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

// -------------------- //
// host window handle

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

// This is a wrapper object for the window handle of the host application (DAW)
pub struct ParentWindow(pub *mut ::core::ffi::c_void);

unsafe impl HasRawWindowHandle for ParentWindow {
    #[cfg(target_os = "macos")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::AppKitHandle::empty();

        handle.ns_view = self.0;

        RawWindowHandle::AppKit(handle)
    }

    #[cfg(target_os = "windows")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::Win32Handle::empty();

        handle.hwnd = self.0;

        RawWindowHandle::Win32(handle)
    }

    #[cfg(target_os = "linux")]
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = raw_window_handle::XcbHandle::empty();

        handle.window = self.0 as u32;

        RawWindowHandle::Xcb(handle)
    }
}