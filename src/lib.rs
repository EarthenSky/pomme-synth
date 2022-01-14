// TODO: reminder to not copy octasine's code, but just use it as a template to get up to speed quickly.

// -------------------- //
// vst stuff

#[macro_use] // use all macros from vst
extern crate vst;

use vst::plugin::{Info, Plugin};
use vst::plugin::HostCallback;

// use log::{info};

use std::sync::Arc;

struct PommeSynth {
    editor: Option<Gui<Arc<SyncState>>>
}

impl PommeSynth {
    fn create(host: Option<HostCallback>) -> PommeSynth {
        let sync = Arc::new(SyncState {
            host: host,
            //presets: built_in_preset_bank(),
            //settings,
        });

        let editor = Gui::new(sync.clone());

        PommeSynth {
            editor: Some(editor)
        }
    }
}

impl Default for PommeSynth {
    fn default() -> Self {
        Self::create(None)
    }
}

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
            unique_id: 31254, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn ::vst::editor::Editor>> {
        // OHhhh, when called, this removes our editor from the synth.... for some reason.
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn ::vst::editor::Editor>)
        } else {
            None
        }
    }
}

plugin_main!(PommeSynth); // Important!

pub struct SyncState {
    /// Host should always be set when running as real plugin, but having the
    /// option of leaving this field empty is useful when benchmarking.
    pub host: Option<HostCallback>,
    //pub presets: PresetBank,
    //pub settings: Settings,
}


// -------------------- //
// gui

use std::time::Duration;

use rtrb::{Consumer, RingBuffer};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use baseview::{Size, Event, EventStatus, Window, WindowHandler, WindowScalePolicy};

use vst::editor::Editor;

pub struct Gui<H: Clone> {
    sync_state: H,
    opened: bool,
}

impl<H: Clone> Gui<H> {
    pub fn new(sync_state: H) -> Self {
        Self {
            sync_state,
            opened: false,
        }
    }

    // NOTE: not dependent on sync_handle, but used to be
    fn baseview_settings(/*sync_handle: H*/) -> baseview::WindowOpenOptions {
        baseview::WindowOpenOptions {
            size: Size::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
            #[cfg(not(target_os = "windows"))]
            scale: WindowScalePolicy::SystemScaleFactor,
            // Windows currently needs scale factor 1.0, or GUI contents
            // will be too large for window
            #[cfg(target_os = "windows")]
            scale: WindowScalePolicy::ScaleFactor(1.0),
            title: "pommesynth-baseview-ohyeah".to_string(),
        }
    }

    pub fn open_parented(parent: ParentWindow, sync_handle: H) {
        let (mut tx, rx) = RingBuffer::new(128);

        ::std::thread::spawn(move || loop {
            ::std::thread::sleep(Duration::from_secs(5));

            if let Err(_) = tx.push(Message::Hello) {
                println!("Failed sending message");
            }
        });

        // TODO: replace this with regular Window
        Window::open_parented(
            &parent,
            Self::baseview_settings(),
            |_| OpenWindowExample { rx }
        );
    }

    /*
    // NOTE: this is only for 
    pub fn open_blocking(sync_handle: H) {
        let settings = Self::get_iced_baseview_settings(sync_handle);

        IcedWindow::<OctaSineIcedApplication<_>>::open_blocking(settings);
    }
    */
}

pub const GUI_WIDTH: usize = 12 * 66;
pub const GUI_HEIGHT: usize = 12 * 61;

impl<H: Clone> Editor for Gui<H> {
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

        Self::open_parented(ParentWindow(parent), self.sync_state.clone());

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

// -------------------- //


#[derive(Debug, Clone)]
enum Message {
    Hello,
}

struct OpenWindowExample {
    rx: Consumer<Message>,
}

impl WindowHandler for OpenWindowExample {
    fn on_frame(&mut self, _window: &mut Window) {
        while let Ok(message) = self.rx.pop() {
            println!("Message: {:?}", message);
        }
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) -> EventStatus {
        match event {
            Event::Mouse(e) => println!("Mouse event: {:?}", e),
            Event::Keyboard(e) => println!("Keyboard event: {:?}", e),
            Event::Window(e) => println!("Window event: {:?}", e),
        }

        EventStatus::Captured
    }
}


/*
fn main() {
    let window_open_options = baseview::WindowOpenOptions {
        title: "baseview".into(),
        size: baseview::Size::new(512.0, 512.0),
        scale: WindowScalePolicy::SystemScaleFactor,
    };

    let (mut tx, rx) = RingBuffer::new(128);

    ::std::thread::spawn(move || loop {
        ::std::thread::sleep(Duration::from_secs(5));

        if let Err(_) = tx.push(Message::Hello) {
            println!("Failed sending message");
        }
    });

    Window::open_blocking(window_open_options, |_| OpenWindowExample { rx });
}
*/