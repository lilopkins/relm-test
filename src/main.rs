use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

struct AppWidgets {
    label: gtk::Label,
}

impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = u8;
    type Root = gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Counter")
            .default_width(300)
            .default_height(150)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter: init };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let btn_inc = gtk::Button::with_label("Increment");
        let btn_dec = gtk::Button::with_label("Decrement");

        let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        label.set_margin_all(5);

        root.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&btn_inc);
        vbox.append(&btn_dec);
        vbox.append(&label);

        btn_inc.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Increment);
        }));

        btn_dec.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Decrement);
        }));

        let widgets = AppWidgets { label };

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

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Counter: {}", self.counter));
    }
}

fn main() {
    let app = RelmApp::new("uk.hpkns.relm-test");
    app.run::<AppModel>(0);
}
