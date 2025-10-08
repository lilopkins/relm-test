#![cfg_attr(
    all(not(debug_assertions), not(feature = "windows-keep-console-window")),
    windows_subsystem = "windows"
)]

use gtk::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

mod icon_names {
    pub use custom::*;
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = u8;

    view! {
        #[root]
        adw::Window {
            set_title: Some("Counter"),
            set_default_width: 300,
            set_default_height: 150,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 0,

                adw::HeaderBar { },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 5,
                    set_spacing: 5,

                    gtk::Button {
                        set_label: "Increment",

                        connect_clicked => AppInput::Increment,
                    },

                    gtk::Button {
                        set_label: "Decrement",

                        connect_clicked => AppInput::Decrement,
                    },

                    gtk::Label {
                        #[watch]
                        set_text: &format!("Counter: {}", model.counter),
                        set_margin_all: 5,
                    },

                    gtk::Button {
                        set_tooltip_text: Some("Papyrus Button to test icons"),
                        set_icon_name: icon_names::PAPYRUS,
                    },

                    gtk::Button {
                        set_tooltip_text: Some("Papyrus Button to test custom icons"),
                        set_icon_name: icon_names::PAPYRUS_VERTICAL_ADD_SYMBOLIC,
                    },
                },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppInput::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("uk.hpkns.relm-test");
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    app.run::<AppModel>(0);
}
