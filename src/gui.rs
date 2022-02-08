// -------------------- //
// gui

use std::sync::Arc;
use core::sync::atomic::Ordering;

//use baseview::{};

use iced_baseview::{executor, WindowQueue, Application, Command};
use iced_baseview::{slider, scrollable, Element, Container, Column, Row, Slider, Text, Rule};
use iced_baseview::{Length, Alignment};

// for the time being
use iced_baseview::*;

// TODO: state should probably be the same as use crate::params::ParamState.... or not?

use crate::params::ParamState;

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(u32),
}

// Stores the GuiState -> Rename? -> PommeGuiState -> yes, TODO: this
pub struct PommeGui {
    params: Arc<ParamState>,

    slider_state: slider::State,
    slider_value: u32,
    slider_text: String,

    rack_view_state: scrollable::State,
}

impl Application for PommeGui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Arc<ParamState>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let gui = Self {
            params: flags,

            slider_state: slider::State::new(),
            slider_value: 0,
            slider_text: String::from("0"),

            rack_view_state: scrollable::State::new(),
        };

        (gui, Command::none())
    }

    fn update(&mut self, _window: &mut WindowQueue, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SliderChanged(value) => {
                self.slider_value = value;

                let fval = value as f64 / 100.0;
                self.slider_text = format!("{}", fval);
                self.params.amplitude.store(fval, Ordering::Relaxed); // TODO: check octasine if I need to use atomic values
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let slider_widget = Slider::new(
            &mut self.slider_state,
            0..=100,
            self.slider_value,
            Message::SliderChanged,
        );

        let rack_view = Scrollable::new(&mut self.rack_view_state)
            .width(Length::FillPortion(7))
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            //.align_items(Alignment)
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing A"))
            .push(Text::new("Thing B"));

        let main_bus = Column::new()
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(20)
            .spacing(20)
            .push(Text::new("Main Bus"))
            .push(Rule::horizontal(10))
            .push(Text::new("Gain:"))
            .push(slider_widget)
            .push(Text::new(self.slider_text.as_str()));
            
        let content = Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            //.align_items(Alignment::Center)
            .push(rack_view)
            .push(main_bus);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// -------------------------------------- //
// param struct draw functions

/*
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
*/